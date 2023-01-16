#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::Filter;
use gtkrs::{prelude::*, FilterListModel, *};
#[derive(Clone)]
pub struct FilterListModelBuilder {
    obj: FilterListModel,
}
impl Default for FilterListModelBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl FilterListModelBuilder {
    pub fn model(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn filter(&mut self, value: &impl IsA<Filter>) -> &mut Self {
        self.obj.set_property("filter", value);
        self
    }
    pub fn incremental(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("incremental", value);
        self
    }
    pub fn bind(&mut self) -> FilterListModelBinder {
        FilterListModelBinder { builder: self }
    }
    pub fn connect(&mut self) -> FilterListModelSignals {
        FilterListModelSignals { builder: self }
    }
}
impl crate::prelude::Builder for FilterListModelBuilder {
    type Target = FilterListModel;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for FilterListModelBuilder {
    type Target = FilterListModel;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct FilterListModelBinder<'builder> {
    builder: &'builder mut FilterListModelBuilder,
}
impl<'builder> FilterListModelBinder<'builder> {
    pub fn model<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FilterListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn filter<T: IsA<Filter>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FilterListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("filter", val));
        value.connect_update(move |value| obj.set_property("filter", value));
        self.builder
    }
    pub fn incremental(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut FilterListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("incremental", val));
        value.connect_update(move |value| obj.set_property("incremental", value));
        self.builder
    }
}
pub struct FilterListModelSignals<'builder> {
    builder: &'builder mut FilterListModelBuilder,
}
impl<'builder> FilterListModelSignals<'builder> {}
impl ForteExt for FilterListModel {
    type Builder = FilterListModelBuilder;
}
#[macro_export]
macro_rules ! FilterListModel { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: FilterListModel :: forte ()) $ ($ rest) * } } }
