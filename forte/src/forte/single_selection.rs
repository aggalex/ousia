#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::SelectionModel;
use gtkrs::{prelude::*, SingleSelection, *};
#[derive(Clone)]
pub struct SingleSelectionBuilder {
    obj: SingleSelection,
}
impl Default for SingleSelectionBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl SingleSelectionBuilder {
    pub fn model(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn autoselect(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("autoselect", value);
        self
    }
    pub fn can_unselect(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_unselect", value);
        self
    }
    pub fn selected(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("selected", value);
        self
    }
    pub fn bind(&mut self) -> SingleSelectionBinder {
        SingleSelectionBinder { builder: self }
    }
    pub fn connect(&mut self) -> SingleSelectionSignals {
        SingleSelectionSignals { builder: self }
    }
}
impl crate::prelude::Builder for SingleSelectionBuilder {
    type Target = SingleSelection;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for SingleSelectionBuilder {
    type Target = SingleSelection;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct SingleSelectionBinder<'builder> {
    builder: &'builder mut SingleSelectionBuilder,
}
impl<'builder> SingleSelectionBinder<'builder> {
    pub fn model<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut SingleSelectionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn autoselect(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SingleSelectionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("autoselect", val));
        value.connect_update(move |value| obj.set_property("autoselect", value));
        self.builder
    }
    pub fn can_unselect(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SingleSelectionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_unselect", val));
        value.connect_update(move |value| obj.set_property("can_unselect", value));
        self.builder
    }
    pub fn selected(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut SingleSelectionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("selected", val));
        value.connect_update(move |value| obj.set_property("selected", value));
        self.builder
    }
}
pub struct SingleSelectionSignals<'builder> {
    builder: &'builder mut SingleSelectionBuilder,
}
impl<'builder> SingleSelectionSignals<'builder> {}
impl ForteExt for SingleSelection {
    type Builder = SingleSelectionBuilder;
}
#[macro_export]
macro_rules ! SingleSelection { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: SingleSelection :: forte ()) $ ($ rest) * } } }
