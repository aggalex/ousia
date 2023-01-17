use syn::{Ident, Visibility};
use std::collections::HashMap;
use quote::__private::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::token::Struct;
use syn::{Field, ItemStruct, Type};

pub struct Builder {
    name: Ident,
    vis: Visibility,
    properties: HashMap<Ident, Field>
}

impl From<ItemStruct> for Builder {
    fn from(class: ItemStruct) -> Self {
        Builder {
            name: class.ident.clone(),
            vis: class.vis.clone(),
            properties: class.fields.into_iter()
                .map(|field| (field.ident.unwrap(), field))
                .collect()
        }
    }
}

impl ToTokens for Builder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let builder_name = format_ident!("{}Builder", &self.name);
        let object = format_ident!("{}Object", &self.name);
        let vis = &self.vis;

        let props = self.properties.iter()
            .map(|(name, field)| {
                let name_str = name.to_string();
                let Field { vis, ty, .. } = &field;
                quote! {
                    #vis fn #name (&mut self, value: #ty) -> &mut Self {
                        self.obj.set_property(#name_str, &BoxedAnyObject::new(value));
                        self
                    }
                }
            })
            .collect::<Vec<_>>();

        *tokens = quote! {
            #vis struct #builder_name {
                obj: #object
            }

            impl #builder_name {
                #( #props )*
            }
        }
    }
}