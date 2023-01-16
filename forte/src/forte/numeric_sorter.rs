#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, NumericSorter, *};
use gtkrs::{Expression, SortType, Sorter};
#[derive(Clone)]
pub struct NumericSorterBuilder {
    obj: NumericSorter,
}
impl Default for NumericSorterBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl NumericSorterBuilder {
    pub fn expression(&mut self, value: impl AsRef<Expression>) -> &mut Self {
        self.obj.set_property("expression", value);
        self
    }
    pub fn sort_order(&mut self, value: SortType) -> &mut Self {
        self.obj.set_property("sort_order", value);
        self
    }
    pub fn bind(&mut self) -> NumericSorterBinder {
        NumericSorterBinder { builder: self }
    }
    pub fn connect(&mut self) -> NumericSorterSignals {
        NumericSorterSignals { builder: self }
    }
}
impl crate::prelude::Builder for NumericSorterBuilder {
    type Target = NumericSorter;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for NumericSorterBuilder {
    type Target = NumericSorter;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct NumericSorterBinder<'builder> {
    builder: &'builder mut NumericSorterBuilder,
}
impl<'builder> NumericSorterBinder<'builder> {
    pub fn sort_order(
        &mut self,
        value: &(impl Prop<SortType> + ?Sized),
    ) -> &mut NumericSorterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sort_order", val));
        value.connect_update(move |value| obj.set_property("sort_order", value));
        self.builder
    }
}
pub struct NumericSorterSignals<'builder> {
    builder: &'builder mut NumericSorterBuilder,
}
impl<'builder> NumericSorterSignals<'builder> {}
impl ForteExt for NumericSorter {
    type Builder = NumericSorterBuilder;
}
#[macro_export]
macro_rules ! NumericSorter { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: NumericSorter :: forte ()) $ ($ rest) * } } }
