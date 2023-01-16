use quote::__private::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, FnArg, Ident, ImplItemMethod, Type, TypeParamBound};
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

        fn unpack_ty(ty: &Type) -> (TokenStream, bool, TokenStream) {
            match ty {
                Type::Reference(r) => {
                    let (generics, _, ty) = unpack_ty(&*r.elem);
                    (generics, false, ty)
                }
                Type::ImplTrait(item) => {
                    let bounds = &item.bounds;
                    (quote! { <T: #bounds> }, true, quote! { T })
                },
                t => (quote! {}, true, quote! { #t })
            }
        }

        let (generic, asterisk, ty_token) = unpack_ty(ty);

        let clone = if asterisk {
            quote! ( .clone() )
        } else {
            quote! (  )
        };

        if let Type::ImplTrait(item) = ty {
            let not_bindable = !item.bounds.iter()
                .filter_map(|bound| match bound {
                    TypeParamBound::Trait(tr) => Some(tr),
                    _ => None
                })
                .any(|tr| tr.path.segments.first().unwrap().ident.to_string() == "IsA");

            if not_bindable {
                return quote! {}
            }
        }

        quote! {
            #( #attrs )*
            pub fn #name #generic(&mut self, value: &(impl Prop<#ty_token> + ?Sized + 'static)) -> &mut #builder_name {
                value.with(|val| self.builder.builder = std::mem::take(&mut self.builder.builder).#name(val #clone));
                let value = value.clone();
                self.builder.on_build(move |obj| {
                    let obj = obj.clone();
                    value.connect_update(move |value| obj.set_property(#property_name, value));
                });
                self.builder
            }
        }
    }
}

impl TryFrom<&ImplItemMethod> for Property {
    type Error = syn::Error;

    fn try_from(f: &ImplItemMethod) -> Result<Self, syn::Error> {
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
                self.builder = std::mem::take(&mut self.builder).#name(value);
                self
            }
        };
    }
}
