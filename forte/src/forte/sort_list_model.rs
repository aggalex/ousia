#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::Sorter;
use gtkrs::{prelude::*, SortListModel, *};
#[derive(Clone)]
pub struct SortListModelBuilder {
    obj: SortListModel,
}
impl Default for SortListModelBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl SortListModelBuilder {
    pub fn incremental(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("incremental", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn sorter(&mut self, value: &impl IsA<Sorter>) -> &mut Self {
        self.obj.set_property("sorter", value);
        self
    }
    pub fn bind(&mut self) -> SortListModelBinder {
        SortListModelBinder { builder: self }
    }
    pub fn connect(&mut self) -> SortListModelSignals {
        SortListModelSignals { builder: self }
    }
}
impl crate::prelude::Builder for SortListModelBuilder {
    type Target = SortListModel;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for SortListModelBuilder {
    type Target = SortListModel;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct SortListModelBinder<'builder> {
    builder: &'builder mut SortListModelBuilder,
}
impl<'builder> SortListModelBinder<'builder> {
    pub fn incremental(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut SortListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("incremental", val));
        value.connect_update(move |value| obj.set_property("incremental", value));
        self.builder
    }
    pub fn model<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut SortListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn sorter<T: IsA<Sorter>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut SortListModelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sorter", val));
        value.connect_update(move |value| obj.set_property("sorter", value));
        self.builder
    }
}
pub struct SortListModelSignals<'builder> {
    builder: &'builder mut SortListModelBuilder,
}
impl<'builder> SortListModelSignals<'builder> {}
impl ForteExt for SortListModel {
    type Builder = SortListModelBuilder;
}
#[macro_export]
macro_rules ! SortListModel { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: SortListModel :: forte ()) $ ($ rest) * } } }
