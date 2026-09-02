use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, FnArg, Ident, ImplItemFn, Token, Type, TypeParamBound};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::attribute::AttributeExtension;

struct Generics {
    traits: Punctuated<TypeParamBound, Token![+]>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub ty: Type,
    pub attrs: Vec<Attribute>
}

fn decode_type(raw_ty: &Type) -> (Vec<TokenStream>, TokenStream) {
    match raw_ty {
        Type::ImplTrait(r#trait) => {
            let bounds = &r#trait.bounds;
            (
                vec![quote!(T: #bounds)],
                quote!(T),
            )
        },
        Type::Reference(reference) => {
            let (mut generics, ty) = decode_type(reference.elem.as_ref());
            generics.insert(0, quote!('a));
            (generics, quote!(&'a (#ty)))
        },
        ty => (vec![], quote!(#ty))
    }
}

impl Property {
    pub fn bind_method(&self, builder_name: &Ident) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let property_name = &self.name;
        let raw_ty = &self.ty;
        let attrs = &self.attrs
            .iter()
            .filter(|attr| attr.is_not_synonymous_doc_to_item(property_name))
            .collect::<Vec<_>>();

        let (generics, ty) = decode_type(raw_ty);

        let generics = if generics.len() > 0 {
            quote! { < #(#generics),* > }
        } else {
            quote! {}
        };

        eprintln!("{generics}");

        quote! {
            #( #attrs )*
            pub fn #name #generics(&mut self,
                handler: &(impl HandlerOf<#ty> + ?Sized + 'static)
            ) -> &mut #builder_name {
                let handler = handler.clone();
                self.builder.on_build(move |obj| {
                    handler.handle(obj, #property_name);
                });
                self.builder
            }
        }
    }
}

impl TryFrom<&ImplItemFn> for Property {
    type Error = syn::Error;

    fn try_from(f: &ImplItemFn) -> Result<Self, syn::Error> {
        Ok(Property {
            name: f.sig.ident.to_string(),
            ty: *f.sig.inputs.iter()
                .filter_map(|arg| match arg {
                    FnArg::Typed(pat) => Some(pat),
                    _ => None
                })
                .next()
                .ok_or_else(|| syn::Error::new(f.span(), "no suitable arguments in function"))?
                .ty
                .clone(),
            attrs: f.attrs.clone()
        })
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let ty = &self.ty;
        let attrs = &self.attrs.iter()
            .filter(|attr| attr.is_not_synonymous_doc_to_item(&self.name))
            .collect::<Vec<_>>();

        *tokens = quote! {
            #( #attrs )*
            pub fn #name(&mut self, value: #ty) -> &mut Self {
                self.builder = Some(
                    self.builder.take().unwrap().#name(value)
                );
                self
            }
        };
    }
}
