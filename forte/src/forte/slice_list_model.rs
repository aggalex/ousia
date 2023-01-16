#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, SliceListModel, *};
#[derive(Clone)]
pub struct SliceListModelBuilder {
    obj: SliceListModel,
}
impl Default for SliceListModelBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl SliceListModelBuilder {
    pub fn offset(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("offset", value);
        self
    }
    pub fn size(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("size", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn bind(&mut self) -> SliceListModelBinder {
        SliceListModelBinder { builder: self }
    }
    pub fn connect(&mut self) -> SliceListModelSignals {
        SliceListModelSignals { builder: self }
    }
}
impl crate::prelude::Builder for SliceListModelBuilder {
    type Target = SliceListModel;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for SliceListModelBuilder {
    type Target = SliceListModel;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct SliceListModelBinder<'builder> {
    builder: &'builder mut SliceListModelBuilder,
}
impl<'builder> SliceListModelBinder<'builder> {
    pub fn offset(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut SliceListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("offset", val));
        value.connect_update(move |value| obj.set_property("offset", value));
        self.builder
    }
    pub fn size(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut SliceListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size", val));
        value.connect_update(move |value| obj.set_property("size", value));
        self.builder
    }
    pub fn model<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut SliceListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
}
pub struct SliceListModelSignals<'builder> {
    builder: &'builder mut SliceListModelBuilder,
}
impl<'builder> SliceListModelSignals<'builder> {}
impl ForteExt for SliceListModel {
    type Builder = SliceListModelBuilder;
}
#[macro_export]
macro_rules ! SliceListModel { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: SliceListModel :: forte ()) $ ($ rest) * } } }
