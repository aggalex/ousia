use std::collections::HashMap;
use std::path::PathBuf;
use quote::__private::{TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, File, Item};

struct Submodule {
    path: PathBuf,
    attributes: Vec<Attribute>
}

impl ToTokens for Submodule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut filename = self.path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        if filename.ends_with(".rs") {
            filename = &filename[..filename.len() - ".rs".len()]
        }

        let ident = format_ident!("{}", filename);
        let attrs = &self.attributes;

        *tokens = quote! {
            #( #attrs )*
            pub mod #ident
        }
    }
}

pub struct Module(HashMap<String, Submodule>);

impl Module {
    pub fn fill_features (mut self, file: File) -> Self {
        let vec = file.items.into_iter()
            .filter_map(|item| match item {
                Item::Mod(m) => Some(m),
                _ => None,
            })
            .filter_map(|item| self.0.remove(&format!("{}.rs",
                item.ident.to_string()
            )).map(|ok| (item, ok)))
            .map(|(item, sub)| Submodule {
                attributes: item.attrs,
                ..sub
            })
            .map(|sub| (sub.path.file_name().unwrap()
                            .to_str().unwrap().to_string(), sub))
            .collect();

        Self(vec)
    }
}

impl From<Vec<PathBuf>> for Module {
    fn from(m: Vec<PathBuf>) -> Self {
        Self(
             m.into_iter()
                 .map(|path| Submodule {
                     path,
                     attributes: vec![]
                 })
                 .map(|sub| (sub.path.file_name().unwrap()
                     .to_str().unwrap().to_string(), sub))
                 .collect::<HashMap<_, _>>()
        )
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let modules = self.0.values()
            .map(Submodule::to_token_stream)
            .collect::<Vec<_>>();

        *tokens = quote! {
            #( #modules; )*
        };
    }
}