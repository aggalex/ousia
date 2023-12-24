use quote::__private::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, FnArg, Ident, ImplItemFn, Type, TypeParamBound};
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub ty: Type,
    pub attrs: Vec<Attribute>
}

impl Property {
    pub fn bind_method(&self, builder_name: &Ident) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let ty = &self.ty;
        let property_name = &self.name;
        let attrs = &self.attrs;

        fn unpack_ty(ty: &Type) -> (TokenStream, TokenStream) {
            match ty {
                Type::Reference(r) => {
                    let (generics, ty) = unpack_ty(&*r.elem);
                    (quote! ( #generics R: AsRef<#ty> ), quote!( R ))
                }
                Type::ImplTrait(item) => {
                    let bounds = &item.bounds;
                    (quote! { T: #bounds, }, quote! { T })
                },
                t => (quote! {}, quote! { #t })
            }
        }

        let (generic, ty_token) = unpack_ty(ty);

        if let Type::ImplTrait(item) = ty {
            let not_bindable = !item.bounds.iter()
                .filter_map(|bound| match bound {
                    TypeParamBound::Trait(tr) => Some(tr),
                    _ => None
                })
                .map(|tr| tr.path.segments.first().unwrap().ident.to_string())
                .any(|trait_name| trait_name == "Into");

            if not_bindable {
                return quote! {}
            }
        }

        quote! {
            #( #attrs )*
            pub fn #name <#generic>(&mut self,
                handler: &(impl Handler<#ty_token> + ?Sized + 'static)
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
        let attrs = &self.attrs;

        *tokens = quote! {
            #( #attrs )*
            pub fn #name(&mut self, value: #ty) -> &mut Self {
                self.builder = std::mem::take(&mut self.builder).map(|builder| builder.#name(value));
                self
            }
        };
    }
}
