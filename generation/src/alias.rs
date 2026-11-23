use std::collections::HashMap;
use std::ops::Deref;
use syn::{File, Item, Type};
use syn::spanned::Spanned;
use crate::class::Class;

pub struct Alias {
    name: String,
    class: String,
}

impl TryFrom<File> for Alias {
    type Error = syn::Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let alias = file.items.iter()
            .filter_map(|item| match item {
                Item::Type(t) => Some(t),
                _ => None,
            }).next()
            .ok_or(syn::Error::new(file.span(), "No alias found"))?;

        let ty = match alias.ty.deref() {
            Type::Path(path) => path.path.segments.last().unwrap().ident.clone(),
            _ => return Err(syn::Error::new(file.span(), "Alias can only be derived on type paths"))
        };

        Ok(Alias {
            name: alias.ident.to_string(),
            class: ty.to_string(),
        })
    }
}

pub enum ClassOrAlias {
    Class(Class),
    Alias(Alias),
}

impl TryFrom<File> for ClassOrAlias {
    type Error = syn::Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        Class::try_from(file.clone()).map(ClassOrAlias::Class)
            .or_else(|err| Alias::try_from(file)
                .map(ClassOrAlias::Alias)
                .map_err(|_| err)
            )
    }
}

impl ClassOrAlias {
    fn name(&self) -> &str {
        match self {
            ClassOrAlias::Alias(alias) => &alias.name,
            ClassOrAlias::Class(class) => &class.name,
        }
    }
    fn decay<'a>(&'a self, generator: &'a HashMap<String, ClassOrAlias>) -> &'a Class {
        let mut current = self;
        while let ClassOrAlias::Alias(alias) = current {
            current = &generator[&alias.class];
        }
        match current {
            ClassOrAlias::Class(class) => class,
            ClassOrAlias::Alias(_) => unreachable!()
        }
    }
}