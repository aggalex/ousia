use std::collections::HashSet;
use std::ptr::addr_of;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, Attribute, Item};
use syn::__private::TokenStream2;
use syn::meta::ParseNestedMeta;
use syn::parse::Parser;
use syn::spanned::Spanned;

pub trait AttributeExtension {
    fn is_not_synonymous_doc_to_item(&self, name: &str) -> bool;
}

struct Doc {
    alias: Option<String>,
    hidden: bool,
}

impl TryFrom<&Attribute> for Doc {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        if !attr.path().is_ident("doc") {
            return Err(syn::Error::new(attr.span(), "Not a doc attribute"));
        }

        let mut doc = Doc {
            alias: None,
            hidden: false,
        };

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("alias") {
                let value: syn::LitStr = meta.value()?.parse()?;
                doc.alias = Some(value.value());
            } else if meta.path.is_ident("hidden") {
                doc.hidden = true;
            }
            Ok(())
        })?;

        Ok(doc)
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Tagged<Item> {
    pub tag: Vec<FeatureTag>,
    pub item: Item,
}

pub trait Map<Item> {
    type Output<X>;

    fn map<X>(self, f: impl FnOnce(Item) -> X) -> Self::Output<X>;
}

impl<Item> Map<Item> for Tagged<Item> {
    type Output<X> = Tagged<X>;

    fn map<X>(self, f: impl FnOnce(Item) -> X) -> Tagged<X> {
        Tagged::new(self.tag, f(self.item))
    }
}

impl<'a, Item> Map<&'a Item> for &'a Tagged<Item> {
    type Output<X> = Tagged<X>;

    fn map<X>(self, f: impl FnOnce(&'a Item) -> X) -> Tagged<X> {
        Tagged::new(self.tag.clone(), f(&self.item))
    }
}

impl<Item: ToTokens> ToTokens for Tagged<Item> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.tag.iter()
            .map(|attr| attr.to_token_stream())
            .collect::<Vec<_>>();
        let tt = self.item.to_token_stream();

        *tokens = quote! {
            #( #attrs )*
            #tt
        }
    }
}

impl<Item> Tagged<Item> {
    pub fn new(tag: Vec<FeatureTag>, item: Item) -> Self {
        Self { tag, item }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum FeatureTag {
    Tag {
        tag: Ident,
        value: String,
    },
    Not(Box<FeatureTag>),
    All(Vec<FeatureTag>),
    Any(Vec<FeatureTag>),
    False,
    True
}

impl FeatureTag {
    /// Generate the inner `cfg`-style expression token stream (without the
    /// surrounding `#[cfg(...)]` wrapper).
    fn expr(&self, tokens: &mut TokenStream) {
        match self {
            FeatureTag::Tag { tag, value } => {
                let value = syn::LitStr::new(value, proc_macro2::Span::call_site());
                tokens.extend(quote! { #tag = #value });
            }
            FeatureTag::Not(x) => {
                let inner = x.serialize_expr();
                tokens.extend(quote! { not(#inner) });
            }
            FeatureTag::All(xs) => {
                let inner = xs.iter().map(FeatureTag::serialize_expr).collect::<Vec<_>>();
                tokens.extend(quote! { all(#(#inner),*) });
            }
            FeatureTag::Any(xs) => {
                let inner = xs.iter().map(FeatureTag::serialize_expr).collect::<Vec<_>>();
                tokens.extend(quote! { any(#(#inner),*) });
            }
            FeatureTag::False => tokens.extend(quote! { false }),
            FeatureTag::True => tokens.extend(quote! { true }),
        }
    }

    fn serialize_expr(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.expr(&mut tokens);
        tokens
    }

    /// Serialize this feature expression into a stable, parseable string.
    pub fn to_expr_string(&self) -> String {
        self.serialize_expr().to_string()
    }

    /// Parse a feature expression string back into a `FeatureTag`.
    pub fn from_expr_string(text: &str) -> syn::Result<Self> {
        let expr: TokenStream = syn::parse_str(text)
            .map_err(|err| syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid feature expression `{text}`: {err}"),
            ))?;
        let attrs = syn::Attribute::parse_outer
            .parse2(quote! { #[cfg(#expr)] })
            .map_err(|err| syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid feature expression `{text}`: {err}"),
            ))?;
        let attr = attrs.into_iter().next().ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid feature expression `{text}`"),
            )
        })?;
        Self::try_from(&attr)
    }

    fn arg_list(&self) -> TokenStream {
        match self {
            FeatureTag::Tag { tag, value } => quote! { #tag = #value },
            FeatureTag::Not(x) => {
                let args = x.arg_list();
                quote! { not(#args) }
            },
            FeatureTag::All(x) => {
                let args: Vec<_> = x.iter()
                    .map(|x| x.arg_list())
                    .collect();
                quote! { all(#(#args),*) }
            },
            FeatureTag::Any(x) => {
                let args: Vec<_> = x.iter()
                    .map(|x| x.arg_list())
                    .collect();
                quote! { any(#(#args),*) }
            },
            FeatureTag::False => quote! { false },
            FeatureTag::True => quote! { true },
        }
    }

    pub fn simplify(self) -> FeatureTag {
        match self {
            FeatureTag::Tag {..} => self,
            FeatureTag::Not(x) => match *x {
                FeatureTag::Not(box y) => y.simplify(),
                x => FeatureTag::Not(Box::new(x.simplify()))
            },
            FeatureTag::All(x) => {
                let simplified = x.into_iter().map(FeatureTag::simplify).collect::<Vec<_>>();
                let contradictory = simplified.iter()
                    .filter_map(|tag| if let FeatureTag::Not(content) = tag { Some(&**content) } else { None })
                    .any(|x| simplified.iter().any(|y| x.eq(y)));
                if contradictory {
                    FeatureTag::False
                } else {
                    FeatureTag::Any(simplified)
                }
            }
            FeatureTag::Any(x) => {
                let mut simplified = x.into_iter().map(FeatureTag::simplify).collect::<Vec<_>>();
                let mut excluded = HashSet::<usize>::new();
                for (i, item) in simplified.iter().enumerate() {
                    if excluded.contains(&i) {
                        continue;
                    }
                    if let FeatureTag::Not(box y) = item {
                        let similar = simplified.iter().enumerate().find(|(_, x)| (*x).eq(y));
                        if let Some((j, x)) = similar {
                            excluded.insert(i);
                            excluded.insert(j);
                        }
                    }
                }
                for i in excluded.iter() {
                    simplified.remove(*i);
                }
                FeatureTag::Any(simplified)
            },
            x => x
        }
    }

    fn parse_nested_feature_meta(meta: ParseNestedMeta) -> syn::Result<FeatureTag> {
        let mut feat = Box::new(Err(syn::Error::new(meta.path.span(), "Unrecognized feature attribute")));

        if meta.path.is_ident("not") {
            meta.parse_nested_meta(|meta| {
                *feat = Ok(
                    FeatureTag::Not(
                        Box::new(FeatureTag::parse_nested_feature_meta(meta)?)
                    )
                );
                Ok(())
            })?;
        } else if meta.path.is_ident("all") || meta.path.is_ident("any") {
            let mut metas = Vec::new();
            meta.parse_nested_meta(|meta| {
                metas.push(FeatureTag::parse_nested_feature_meta(meta)?);
                Ok(())
            })?;
            *feat = if meta.path.is_ident("all") {
                Ok(FeatureTag::All(metas))
            } else {
                Ok(FeatureTag::Any(metas))
            }
        } else if meta.path.is_ident("false") {
            *feat = Ok(FeatureTag::False)
        } else if meta.path.is_ident("true") {
            *feat = Ok(FeatureTag::True)
        } else {
            let tag: syn::Ident = meta.path.get_ident()
                .ok_or(syn::Error::new(meta.path.span(), "Expected assignment"))?
                .clone();
            let value: syn::LitStr = meta.value()?.parse()?;
            *feat = Ok(FeatureTag::Tag { tag, value: value.value() })
        }

        *feat
    }

}

impl ToTokens for FeatureTag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arg_list = self.arg_list();

        *tokens = quote! {
            #[cfg(#arg_list)]
        }
    }
}

impl TryFrom<&Attribute> for FeatureTag {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        if !attr.path().is_ident("cfg") {
            return Err(syn::Error::new(attr.span(), "Not a doc attribute"));
        }

        let mut feat = FeatureTag::Any(Vec::new());

        attr.parse_nested_meta(|meta| {
            feat = FeatureTag::parse_nested_feature_meta(meta)?;
            Ok(())
        })?;

        Ok(feat)
    }
}

impl AttributeExtension for Attribute {
    fn is_not_synonymous_doc_to_item(&self, name: &str) -> bool {
        !Doc::try_from(self).ok()
            .and_then(|doc| doc.alias)
            .map(|alias| alias == name)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Ident;

    fn tag(name: &str, value: &str) -> FeatureTag {
        FeatureTag::Tag {
            tag: Ident::new(name, proc_macro2::Span::call_site()),
            value: value.to_string(),
        }
    }

    #[test]
    fn expr_string_round_trips() {
        let cases = vec![
            tag("v4_10", "v4_10"),
            FeatureTag::Not(Box::new(tag("v4_10", "v4_10"))),
            FeatureTag::All(vec![tag("a", "a"), tag("b", "b")]),
            FeatureTag::Any(vec![tag("a", "a"), tag("b", "b")]),
            FeatureTag::Not(Box::new(FeatureTag::All(vec![
                tag("a", "a"),
                tag("b", "b"),
            ]))),
            FeatureTag::False,
            FeatureTag::True,
            FeatureTag::Any(vec![
                FeatureTag::All(vec![tag("x", "x"), tag("y", "y")]),
                FeatureTag::Not(Box::new(tag("z", "z"))),
            ]),
        ];

        for case in &cases {
            let text = case.to_expr_string();
            let parsed = FeatureTag::from_expr_string(&text)
                .unwrap_or_else(|err| panic!("failed to parse `{text}`: {err}"));
            assert_eq!(case, &parsed, "round-trip mismatch for `{text}`");
        }
    }
}