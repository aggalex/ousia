#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::ShortcutTrigger;
use gtkrs::{prelude::*, AlternativeTrigger, *};
#[derive(Clone)]
pub struct AlternativeTriggerBuilder {
    obj: AlternativeTrigger,
}
impl Default for AlternativeTriggerBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl AlternativeTriggerBuilder {
    pub fn first(&mut self, value: &impl IsA<ShortcutTrigger>) -> &mut Self {
        self.obj.set_property("first", value);
        self
    }
    pub fn second(&mut self, value: &impl IsA<ShortcutTrigger>) -> &mut Self {
        self.obj.set_property("second", value);
        self
    }
    pub fn bind(&mut self) -> AlternativeTriggerBinder {
        AlternativeTriggerBinder { builder: self }
    }
    pub fn connect(&mut self) -> AlternativeTriggerSignals {
        AlternativeTriggerSignals { builder: self }
    }
}
impl crate::prelude::Builder for AlternativeTriggerBuilder {
    type Target = AlternativeTrigger;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for AlternativeTriggerBuilder {
    type Target = AlternativeTrigger;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct AlternativeTriggerBinder<'builder> {
    builder: &'builder mut AlternativeTriggerBuilder,
}
impl<'builder> AlternativeTriggerBinder<'builder> {
    pub fn first<T: IsA<ShortcutTrigger>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut AlternativeTriggerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("first", val));
        value.connect_update(move |value| obj.set_property("first", value));
        self.builder
    }
    pub fn second<T: IsA<ShortcutTrigger>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut AlternativeTriggerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("second", val));
        value.connect_update(move |value| obj.set_property("second", value));
        self.builder
    }
}
pub struct AlternativeTriggerSignals<'builder> {
    builder: &'builder mut AlternativeTriggerBuilder,
}
impl<'builder> AlternativeTriggerSignals<'builder> {}
impl ForteExt for AlternativeTrigger {
    type Builder = AlternativeTriggerBuilder;
}
#[macro_export]
macro_rules ! AlternativeTrigger { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: AlternativeTrigger :: forte ()) $ ($ rest) * } } }
