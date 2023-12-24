use std::collections::HashMap;
use std::fmt::Debug;
use quote::{format_ident, quote, ToTokens};
use quote::__private::TokenStream;
use proc_macro2::TokenTree;
use syn::{Attribute, File, Ident, ImplItem, Item, ItemUse, TraitItem, Type, UseTree};
use syn::spanned::Spanned;
use crate::prop::Property;
use crate::signal::Signal;

const ALLOWED_IMPORTS: &[&str] = &["crate", "glib", "gio"];

#[derive(Debug, Clone)]
pub struct GtkImport {
    tree: UseTree,
    attrs: Vec<Attribute>
}

impl TryFrom<&ItemUse> for GtkImport {
    type Error = ();

    fn try_from(item: &ItemUse) -> Result<Self, Self::Error> {
        let tree = match &item.tree {
            UseTree::Path(p) if p.ident.to_string() == "crate" => &*p.tree,
            UseTree::Path(p) if ALLOWED_IMPORTS.contains(&&p.ident.to_string()[..])
            => &item.tree,
            _ => return Err(())
        }.clone();

        let attrs = item.attrs.clone();

        Ok(GtkImport {
            tree,
            attrs
        })
    }
}

impl ToTokens for GtkImport {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let GtkImport { tree, attrs } = &self;
        *tokens = quote! {
            #( #attrs )*
            use gtkrs::#tree;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub setters: HashMap<String, Property>,
    pub used: Vec<GtkImport>,
    pub signals: HashMap<String, Signal>,
    pub inherits: Vec<String>,
    pub constructible: bool
}

impl Class {
    pub fn add_builder_from_file(&mut self, file: &File) -> Result<&mut Self, syn::Error> {
        const EXCLUDED: &[&str] = &["new", "build"];

        let setter_iter = file.items.iter()
            .filter_map(|item| match item {
                Item::Impl(item) => Some(item),
                _ => None
            })
            .filter(|item| item.trait_.is_none())
            .filter(|item| match &*item.self_ty {
                Type::Path(path) => path.path.segments
                    .last().unwrap().ident == self.builder_name().to_string(),
                _ => false
            })
            .next()
            .ok_or(syn::Error::new(file.span(), "No builder implementation found"))?
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Fn(f) => Some(f),
                _ => None
            })
            .filter(|f| !EXCLUDED.contains(&&*f.sig.ident.to_string()))
            .filter_map(|f| Property::try_from(f).ok())
            .map(|prop| (prop.name.to_string(), prop));

        self.constructible = true;
        self.setters.extend(setter_iter);

        Ok(self)
    }

    pub fn add_signals_from_file(&mut self, file: &File) -> Result<&mut Self, syn::Error> {
        let signal_iter = file.items.iter()
            .filter_map(|item| match item {
                Item::Trait(item) => Some(item),
                _ => None
            })
            .filter(|item| item.ident.to_string().ends_with("Ext"))
            .map(|item| {
                let quote = quote!(#item).to_string();
                item
            })
            .flat_map(|item| &item.items)
            .filter_map(|item| match item {
                TraitItem::Fn(m) => Some(m),
                _ => None
            })
            .filter(|m| m.sig.ident.to_string().starts_with("connect_"))
            .map(Signal::try_from)
            .map(|sig|
                sig.map(|sig| (sig.name.clone(), sig))
            )
            .collect::<Result<HashMap<_, _>, _>>()?;

        self.signals.extend(signal_iter);

        Ok(self)
    }

    pub fn add_props_from_class(&mut self, class: &Class) -> &mut Self {
        self.setters.extend(class.setters.clone());
        self
    }

    pub fn add_signals_from_class(&mut self, class: &Class) -> &mut Self {
        self.signals.extend(class.signals.clone());
        self
    }
}

impl TryFrom<File> for Class {
    type Error = syn::Error;

    fn try_from(file: File) -> Result<Class, syn::Error> {
        let mut last_token: String = "".to_string();

        let mac = file.items.iter()
            .filter_map(|item| match item {
                Item::Macro(mac) => Some(mac),
                _ => None
            })
            .filter(|mac| {
                mac.mac.path.segments
                    .last()
                    .map(|name| name.ident.to_string() == "wrapper")
                    .unwrap_or(false)
            })
            .next()
            .ok_or(syn::Error::new(file.span(), "No declaration of gobject class found"))?;

        let mut iter = mac
            .mac.tokens.clone().into_iter()
            .take_while(|token| match token {
                TokenTree::Punct(p) => p.as_char() != ';',
                _ => true
            })
            .filter_map(|token| match token {
                TokenTree::Ident(ident) => Some(ident.to_string()),
                _ => None
            });

        // automata to find name of struct
        let name = loop {
            let token = iter.next().ok_or(syn::Error::new(mac.span(), "no struct declaration found in glib::wrapper!"))?;
            last_token = match (&*last_token, &*token) {
                (_, "pub") => token,
                ("pub", "struct") => token,
                ("struct", _) => {
                    break token
                },
                _ => "".to_string()
            }
        };

        const PARENT_TOKENS: &'static [&'static str] = &[
            "implements",
            "extends"
        ];

        let inherits = iter
            .skip_while(|token| !PARENT_TOKENS.contains(&&**token))
            .filter(|token| !PARENT_TOKENS.contains(&&**token))
            .collect();

        let used = file.items.iter()
            .filter_map(|item| match item {
                Item::Use(u) => Some(u),
                _ => None,
            })
            .filter_map(|item| GtkImport::try_from(item).ok())
            .collect();

        let mut class = Class {
            name,
            used,
            setters: HashMap::new(),
            signals: HashMap::new(),
            inherits,
            constructible: false
        };

        if &class.name == "Button" {
            println!("Button inherits: {:?}", class.inherits);
        }

        let _ = class
            .add_builder_from_file(&file);

        let _ = class
            .add_signals_from_file(&file);

        Ok(class)
    }
}

impl Class {

    fn builder_name(&self) -> Ident {
        format_ident!("{}Builder", self.name)
    }

    fn template_name(&self) -> Ident {
        format_ident!("{}Template", self.name)
    }

    fn class_name(&self) -> Ident {
        format_ident!("{}", &self.name)
    }

    fn create_builder(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let builder_name = self.builder_name();
        let binder_name = format_ident!("{}Binder", &self.name);
        let signals_name = format_ident!("{}Signals", &self.name);

        let setters = self.setters.values()
            .map(ToTokens::to_token_stream)
            .collect::<Vec<_>>();

        let binders = self.setters.values()
            .map(|prop| prop.bind_method(&builder_name))
            .collect::<Vec<_>>();

        let signals = self.signals.values()
            .map(|sig| sig.to_token_stream(
                &name,
                &builder_name
            ))
            .collect::<Vec<_>>();

        quote! {
            pub struct #builder_name {
                builder: Option<gtkrs::builders::#builder_name>,
                on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::#name) + 'static>>,
                object: Option<gtkrs::#name>
            }

            impl Default for #builder_name {
                fn default() -> Self {
                    Self {
                        builder: Some(gtkrs::#name::builder()),
                        on_build: Vec::new(),
                        object: None
                    }
                }
            }

            impl #builder_name {

                #( #setters )*

                pub fn bind(&mut self) -> #binder_name {
                    #binder_name {
                        builder: self
                    }
                }

                pub fn connect(&mut self) -> #signals_name {
                    #signals_name {
                        builder: self
                    }
                }

                pub fn on_build(&mut self, f: impl FnOnce(&gtkrs::#name) + 'static) -> &mut Self {
                    self.on_build.push(std::boxed::Box::new(f));
                    self
                }

            }

            impl crate::prelude::Builder for #builder_name {
                type Target = #name;

                fn build(&mut self, func: impl Fn(Self::Target)) {
                    func(self.create());
                }
                fn create(&mut self) -> Self::Target {
                    let obj = std::mem::take(&mut self.builder).expect("Builder is being processed").build();
                    std::mem::take(&mut self.on_build).into_iter()
                        .for_each(|f| f(&obj));
                    obj
                }
            }

            pub struct #binder_name<'builder> {
                builder: &'builder mut #builder_name
            }

            impl<'builder> #binder_name<'builder> {
                #( #binders )*
            }

            pub struct #signals_name<'builder> {
                builder: &'builder mut #builder_name
            }

            impl<'builder> #signals_name<'builder> {
                #( #signals )*
            }

            impl ForteExt for #name {
                type Builder = #builder_name;
            }
        }
    }

}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let builder_name = self.builder_name();
        let name = self.class_name();

        let uses = self.used.iter()
            .map(ToTokens::to_token_stream)
            .collect::<Vec<_>>();

        let cleanup = match self.signals.contains_key("destroy") {
            true => quote!( self.connect_destroy(move |_| _f()); ),
            false => quote!()
        };

        let builder = self.create_builder();

        *tokens = quote! {
            #![allow(dead_code, unused_imports)]

            #( #uses )*
            use gtkrs::{ *, #name, prelude::*, traits::* };
            use crate::prelude::*;
            use rxrust::prelude::*;

            #builder

            impl Cleanup for #name {
                fn cleanup(&self, mut _f: impl Fn() + 'static) {
                    #cleanup
                }
            }

            #[macro_export]
            macro_rules! #name {
                { $($rest:tt)* } => {
                    ousia! { ( ::ousia::ousia::#builder_name::default()) $($rest)* }
                }
            }
        };
    }
}