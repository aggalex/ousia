#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, StringFilter, *};
use gtkrs::{Expression, Filter, StringFilterMatchMode};
#[derive(Clone)]
pub struct StringFilterBuilder {
    obj: StringFilter,
}
impl Default for StringFilterBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl StringFilterBuilder {
    pub fn expression(&mut self, value: impl AsRef<Expression>) -> &mut Self {
        self.obj.set_property("expression", value);
        self
    }
    pub fn match_mode(&mut self, value: StringFilterMatchMode) -> &mut Self {
        self.obj.set_property("match_mode", value);
        self
    }
    pub fn search(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("search", value);
        self
    }
    pub fn ignore_case(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("ignore_case", value);
        self
    }
    pub fn bind(&mut self) -> StringFilterBinder {
        StringFilterBinder { builder: self }
    }
    pub fn connect(&mut self) -> StringFilterSignals {
        StringFilterSignals { builder: self }
    }
}
impl crate::prelude::Builder for StringFilterBuilder {
    type Target = StringFilter;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for StringFilterBuilder {
    type Target = StringFilter;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct StringFilterBinder<'builder> {
    builder: &'builder mut StringFilterBuilder,
}
impl<'builder> StringFilterBinder<'builder> {
    pub fn match_mode(
        &mut self,
        value: &(impl Prop<StringFilterMatchMode> + ?Sized),
    ) -> &mut StringFilterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("match_mode", val));
        value.connect_update(move |value| obj.set_property("match_mode", value));
        self.builder
    }
    pub fn search(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut StringFilterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("search", val));
        value.connect_update(move |value| obj.set_property("search", value));
        self.builder
    }
    pub fn ignore_case(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut StringFilterBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ignore_case", val));
        value.connect_update(move |value| obj.set_property("ignore_case", value));
        self.builder
    }
}
pub struct StringFilterSignals<'builder> {
    builder: &'builder mut StringFilterBuilder,
}
impl<'builder> StringFilterSignals<'builder> {}
impl ForteExt for StringFilter {
    type Builder = StringFilterBuilder;
}
#[macro_export]
macro_rules ! StringFilter { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: StringFilter :: forte ()) $ ($ rest) * } } }
