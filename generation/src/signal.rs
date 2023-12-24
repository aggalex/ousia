use quote::__private::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, GenericParam, Ident, ImplItemFn, Meta, MetaNameValue, PathArguments, ReturnType, TraitItem, TraitItemFn, Type, TypeParamBound};
use syn::spanned::Spanned;

#[derive(Clone, Debug)]
pub struct Signal {
    pub name: String,
    pub args: Vec<Type>,
    pub ret: ReturnType,
    pub attrs: Vec<Attribute>,
    pub fn_bound: String,
}

impl Signal {

    pub fn to_token_stream(&self, obj: &Ident, builder: &Ident) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let ty = &self.args;
        let connector = format_ident!("connect_{}", name);
        let ret = &self.ret;
        let attrs = &self.attrs
            .iter()
            .filter(|attr| {
                let quote = quote!(#attr).to_string();
                let Meta::List(ref list) = attr.meta else { return true };
                if !list.path.segments.last().is_some_and(|path| path.ident == "doc") {
                    return true
                }
                let Ok(arg) = list.parse_args::<MetaNameValue>() else { return true };

                let path_quote = quote!(#arg).to_string();
                !arg.path.segments.last().is_some_and(|seg| seg.ident == "alias")
            })
            .collect::<Vec<_>>();
        let fn_bound = format_ident!("{}", self.fn_bound);

        quote! {
            #( #attrs )*
            pub fn #name(&mut self, f: impl #fn_bound(&#obj, #(#ty),*) #ret + 'static) -> &mut #builder {
                self.builder.on_build(move |obj| { obj.#connector(f); });
                &mut self.builder
            }
        }
    }
}

impl TryFrom<&TraitItemFn> for Signal {
    type Error = syn::Error;

    fn try_from(item: &TraitItemFn) -> Result<Self, Self::Error> {
        let mut name = item.sig.ident.to_string();
        if !name.starts_with("connect_") {
            return Err(syn::Error::new(item.sig.ident.span(), "Not a signal"));
        }

        if &name[..] == "connect_clicked" {
            println!("breakpoint");
        }

        name = name.chars()
            .skip("connect_".len())
            .collect::<String>();

        let inputs_span = item.sig.inputs.span();

        let (fn_bound, ang) = item.sig.generics.params.iter()
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
                    let fn_bound = bound.path.segments.last()?
                        .ident.to_string();
                    if fn_bound.starts_with("Fn") {
                        Some((fn_bound, bound))
                    } else {
                        None
                    }
                })
                .next()
            )
            .filter_map(|(fn_bound, tr)| match &tr.path.segments.last().as_ref()?.arguments {
                PathArguments::Parenthesized(ang) => Some((fn_bound, ang)),
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
            attrs,
            fn_bound
        })
    }
}