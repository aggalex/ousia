use std::collections::HashMap;
use std::fmt::Debug;
use quote::__private::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, File, ImplItem, Item, ItemUse, Type, UseTree};
use syn::spanned::Spanned;
use crate::prop::Property;
use crate::signal::Signal;

const ALLOWED_IMPORTS: &[&str] = &["crate", "glib", "gio"];

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub setters: HashMap<String, Property>,
    pub used: Vec<GtkImport>,
    pub signals: HashMap<String, Signal>
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
            .filter_map(|token| match token {
                TokenTree::Ident(ident) => Some(ident.to_string()),
                _ => None
            });

        let name = loop {
            let token = iter.next().ok_or(syn::Error::new(mac.span(), "no struct declaration found in glib::wrapper!"))?;
            last_token = match (&*last_token, &*token) {
                (_, "pub") => token,
                ("pub", "struct") => token,
                ("struct", _) => break token,
                _ => "".to_string()
            }
        };

        let used = file.items.iter()
            .filter_map(|item| match item {
                Item::Use(u) => Some(u),
                _ => None,
            })
            .filter_map(|item| GtkImport::try_from(item).ok())
            .collect();

        let builder_name = format!("{name}Builder");

        const EXCLUDED: &[&str] = &["new", "build"];

        let setters = file.items.iter()
            .filter_map(|item| match item {
                Item::Impl(item) => Some(item),
                _ => None
            })
            .filter(|item| item.trait_.is_none())
            .filter(|item| match &*item.self_ty {
                Type::Path(path) => path.path.segments
                    .last().unwrap().ident == builder_name,
                _ => false
            })
            .next()
            .ok_or(syn::Error::new(file.span(), "No builder implementation found"))?
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Method(f) => Some(f),
                _ => None
            })
            .filter(|f| !EXCLUDED.contains(&&*f.sig.ident.to_string()))
            .filter_map(|f| Property::try_from(f).ok())
            .map(|prop| (prop.name.to_string(), prop))
            .collect();

        let signals = file.items.iter()
            .filter_map(|item| match item {
                Item::Impl(item) => Some(item),
                _ => None
            })
            .filter(|item| item.trait_.as_ref()
                .and_then(|item| item.1.segments.last())
                .map(|seg| seg.ident.to_string().ends_with("Ext"))
                .unwrap_or(false))
            .flat_map(|item| &item.items)
            .filter_map(|item| match item {
                ImplItem::Method(m) => Some(m),
                _ => None
            })
            .filter(|m| m.sig.ident.to_string().starts_with("connect_"))
            .map(Signal::try_from)
            .map(|sig|
                sig.map(|sig| (sig.name.clone(), sig))
            )
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Class {
            name,
            used,
            setters,
            signals
        })
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let builder_name = format_ident!("{}Builder", self.name);
        let uses = self.used.iter()
            .map(ToTokens::to_token_stream)
            .collect::<Vec<_>>();
        let name = format_ident!("{}", &self.name);
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

        *tokens = quote! {
            #![allow(dead_code, unused_imports)]

            #( #uses )*
            use gtkrs::{ *, #name, prelude::*, traits::* };
            use crate::prelude::*;

            #[derive(Default)]
            pub struct #builder_name {
                builder: gtkrs::builders::#builder_name,
                on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::#name) + 'static>>,
                object: Option<gtkrs::#name>
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
                    let obj = std::mem::take(&mut self.builder).build();
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

            #[macro_export]
            macro_rules! #name {
                { $($rest:tt)* } => {
                    ousia! { (gtkrs::#name::ousia()) $($rest)* }
                }
            }
        };
    }
}