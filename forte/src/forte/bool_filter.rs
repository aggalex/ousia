#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, BoolFilter, *};
use gtkrs::{Expression, Filter};
#[derive(Clone)]
pub struct BoolFilterBuilder {
    obj: BoolFilter,
}
impl Default for BoolFilterBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl BoolFilterBuilder {
    pub fn expression(&mut self, value: impl AsRef<Expression>) -> &mut Self {
        self.obj.set_property("expression", value);
        self
    }
    pub fn invert(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("invert", value);
        self
    }
    pub fn bind(&mut self) -> BoolFilterBinder {
        BoolFilterBinder { builder: self }
    }
    pub fn connect(&mut self) -> BoolFilterSignals {
        BoolFilterSignals { builder: self }
    }
}
impl crate::prelude::Builder for BoolFilterBuilder {
    type Target = BoolFilter;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for BoolFilterBuilder {
    type Target = BoolFilter;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct BoolFilterBinder<'builder> {
    builder: &'builder mut BoolFilterBuilder,
}
impl<'builder> BoolFilterBinder<'builder> {
    pub fn invert(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoolFilterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invert", val));
        value.connect_update(move |value| obj.set_property("invert", value));
        self.builder
    }
}
pub struct BoolFilterSignals<'builder> {
    builder: &'builder mut BoolFilterBuilder,
}
impl<'builder> BoolFilterSignals<'builder> {}
impl ForteExt for BoolFilter {
    type Builder = BoolFilterBuilder;
}
#[macro_export]
macro_rules ! BoolFilter { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: BoolFilter :: forte ()) $ ($ rest) * } } }
