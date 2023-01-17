use quote::__private::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, GenericParam, Ident, ImplItemMethod, PathArguments, ReturnType, Type, TypeParamBound};
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct Signal {
    pub name: String,
    pub args: Vec<Type>,
    pub ret: ReturnType,
    pub attrs: Vec<Attribute>
}

impl Signal {

    pub fn to_token_stream(&self, obj: &Ident, builder: &Ident) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let ty = &self.args;
        let connector = format_ident!("connect_{}", name);
        let ret = &self.ret;
        let attrs = &self.attrs;

        quote! {
            #( #attrs )*
            pub fn #name(&mut self, f: impl Fn(&#obj, #(#ty),*) #ret + 'static) -> &mut #builder {
                self.builder.on_build(move |obj| { obj.#connector(f); });
                &mut self.builder
            }
        }
    }
}

impl TryFrom<&ImplItemMethod> for Signal {
    type Error = syn::Error;

    fn try_from(item: &ImplItemMethod) -> Result<Self, Self::Error> {
        let mut name = item.sig.ident.to_string();
        if !name.starts_with("connect_") {
            return Err(syn::Error::new(item.sig.ident.span(), "Not a signal"));
        }

        name = name.chars()
            .skip("connect_".len())
            .collect::<String>();

        let inputs_span = item.sig.inputs.span();

        let ang = item.sig.generics.params.iter()
            .filter_map(|arg| match arg {
                GenericParam::Type(ty) => Some(&ty.bounds),
                _ => None
            })
            .filter_map(|bounds| bounds.iter()
                .filter_map(|bound| match bound {
                    TypeParamBound::Trait(tr) => Some(tr),
                    _ => None
                })
                .filter_map(|bound| {
                    if bound.path.segments.last()?
                        .ident.to_string().starts_with("Fn") {
                        Some(bound)
                    } else {
                        None
                    }
                })
                .next()
            )
            .filter_map(|tr| match &tr.path.segments.last().as_ref()?.arguments {
                PathArguments::Parenthesized(ang) => Some(ang),
                _ => None
            })
            .next()
            .ok_or_else(|| syn::Error::new(inputs_span, "Unable to find closure argument"))?;

        let args = ang.inputs
            .iter()
            .skip(1)
            .cloned()
            .collect();

        let ret = ang.output.clone();

        let attrs = item.attrs.clone();

        Ok(Signal {
            name,
            args,
            ret,
            attrs
        })
    }
}