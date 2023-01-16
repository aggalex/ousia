#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EntryCompletion, *};
use gtkrs::{Buildable, CellArea, CellLayout, TreeIter, TreeModel};
#[derive(Clone)]
pub struct EntryCompletionBuilder {
    obj: EntryCompletion,
}
impl Default for EntryCompletionBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EntryCompletionBuilder {
    pub fn popup_completion(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("popup_completion", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<TreeModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn popup_single_match(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("popup_single_match", value);
        self
    }
    pub fn text_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("text_column", value);
        self
    }
    pub fn cell_area(&mut self, value: &impl IsA<CellArea>) -> &mut Self {
        self.obj.set_property("cell_area", value);
        self
    }
    pub fn inline_selection(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("inline_selection", value);
        self
    }
    pub fn minimum_key_length(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("minimum_key_length", value);
        self
    }
    pub fn inline_completion(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("inline_completion", value);
        self
    }
    pub fn popup_set_width(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("popup_set_width", value);
        self
    }
    pub fn bind(&mut self) -> EntryCompletionBinder {
        EntryCompletionBinder { builder: self }
    }
    pub fn connect(&mut self) -> EntryCompletionSignals {
        EntryCompletionSignals { builder: self }
    }
}
impl crate::prelude::Builder for EntryCompletionBuilder {
    type Target = EntryCompletion;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EntryCompletionBuilder {
    type Target = EntryCompletion;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EntryCompletionBinder<'builder> {
    builder: &'builder mut EntryCompletionBuilder,
}
impl<'builder> EntryCompletionBinder<'builder> {
    pub fn popup_completion(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("popup_completion", val));
        value.connect_update(move |value| obj.set_property("popup_completion", value));
        self.builder
    }
    pub fn model<T: IsA<TreeModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn popup_single_match(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("popup_single_match", val));
        value.connect_update(move |value| obj.set_property("popup_single_match", value));
        self.builder
    }
    pub fn text_column(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_column", val));
        value.connect_update(move |value| obj.set_property("text_column", value));
        self.builder
    }
    pub fn cell_area<T: IsA<CellArea>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_area", val));
        value.connect_update(move |value| obj.set_property("cell_area", value));
        self.builder
    }
    pub fn inline_selection(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inline_selection", val));
        value.connect_update(move |value| obj.set_property("inline_selection", value));
        self.builder
    }
    pub fn minimum_key_length(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("minimum_key_length", val));
        value.connect_update(move |value| obj.set_property("minimum_key_length", value));
        self.builder
    }
    pub fn inline_completion(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inline_completion", val));
        value.connect_update(move |value| obj.set_property("inline_completion", value));
        self.builder
    }
    pub fn popup_set_width(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryCompletionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("popup_set_width", val));
        value.connect_update(move |value| obj.set_property("popup_set_width", value));
        self.builder
    }
}
pub struct EntryCompletionSignals<'builder> {
    builder: &'builder mut EntryCompletionBuilder,
}
impl<'builder> EntryCompletionSignals<'builder> {}
impl ForteExt for EntryCompletion {
    type Builder = EntryCompletionBuilder;
}
#[macro_export]
macro_rules ! EntryCompletion { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EntryCompletion :: forte ()) $ ($ rest) * } } }
