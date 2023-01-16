#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
use gtkrs::Collation;
use gtkrs::{prelude::*, StringSorter, *};
use gtkrs::{Expression, Sorter};
#[derive(Clone)]
pub struct StringSorterBuilder {
    obj: StringSorter,
}
impl Default for StringSorterBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl StringSorterBuilder {
    pub fn ignore_case(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("ignore_case", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn collation(&mut self, value: Collation) -> &mut Self {
        self.obj.set_property("collation", value);
        self
    }
    pub fn expression(&mut self, value: impl AsRef<Expression>) -> &mut Self {
        self.obj.set_property("expression", value);
        self
    }
    pub fn bind(&mut self) -> StringSorterBinder {
        StringSorterBinder { builder: self }
    }
    pub fn connect(&mut self) -> StringSorterSignals {
        StringSorterSignals { builder: self }
    }
}
impl crate::prelude::Builder for StringSorterBuilder {
    type Target = StringSorter;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for StringSorterBuilder {
    type Target = StringSorter;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct StringSorterBinder<'builder> {
    builder: &'builder mut StringSorterBuilder,
}
impl<'builder> StringSorterBinder<'builder> {
    pub fn ignore_case(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut StringSorterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ignore_case", val));
        value.connect_update(move |value| obj.set_property("ignore_case", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn collation(
        &mut self,
        value: &(impl Prop<Collation> + ?Sized),
    ) -> &mut StringSorterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("collation", val));
        value.connect_update(move |value| obj.set_property("collation", value));
        self.builder
    }
}
pub struct StringSorterSignals<'builder> {
    builder: &'builder mut StringSorterBuilder,
}
impl<'builder> StringSorterSignals<'builder> {}
impl ForteExt for StringSorter {
    type Builder = StringSorterBuilder;
}
#[macro_export]
macro_rules ! StringSorter { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: StringSorter :: forte ()) $ ($ rest) * } } }
