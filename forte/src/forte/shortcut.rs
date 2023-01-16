#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Shortcut, *};
use gtkrs::{ShortcutAction, ShortcutTrigger};
#[derive(Clone)]
pub struct ShortcutBuilder {
    obj: Shortcut,
}
impl Default for ShortcutBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ShortcutBuilder {
    pub fn trigger(&mut self, value: &impl IsA<ShortcutTrigger>) -> &mut Self {
        self.obj.set_property("trigger", value);
        self
    }
    pub fn action(&mut self, value: &impl IsA<ShortcutAction>) -> &mut Self {
        self.obj.set_property("action", value);
        self
    }
    pub fn arguments(&mut self, value: &glib::Variant) -> &mut Self {
        self.obj.set_property("arguments", value);
        self
    }
    pub fn bind(&mut self) -> ShortcutBinder {
        ShortcutBinder { builder: self }
    }
    pub fn connect(&mut self) -> ShortcutSignals {
        ShortcutSignals { builder: self }
    }
}
impl crate::prelude::Builder for ShortcutBuilder {
    type Target = Shortcut;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ShortcutBuilder {
    type Target = Shortcut;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ShortcutBinder<'builder> {
    builder: &'builder mut ShortcutBuilder,
}
impl<'builder> ShortcutBinder<'builder> {
    pub fn trigger<T: IsA<ShortcutTrigger>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ShortcutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("trigger", val));
        value.connect_update(move |value| obj.set_property("trigger", value));
        self.builder
    }
    pub fn action<T: IsA<ShortcutAction>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ShortcutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action", val));
        value.connect_update(move |value| obj.set_property("action", value));
        self.builder
    }
    pub fn arguments(
        &mut self,
        value: &(impl Prop<glib::Variant> + ?Sized),
    ) -> &mut ShortcutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("arguments", val));
        value.connect_update(move |value| obj.set_property("arguments", value));
        self.builder
    }
}
pub struct ShortcutSignals<'builder> {
    builder: &'builder mut ShortcutBuilder,
}
impl<'builder> ShortcutSignals<'builder> {}
impl ForteExt for Shortcut {
    type Builder = ShortcutBuilder;
}
#[macro_export]
macro_rules ! Shortcut { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Shortcut :: forte ()) $ ($ rest) * } } }
