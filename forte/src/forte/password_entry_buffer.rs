#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::EntryBuffer;
use gtkrs::{prelude::*, PasswordEntryBuffer, *};
#[derive(Clone)]
pub struct PasswordEntryBufferBuilder {
    obj: PasswordEntryBuffer,
}
impl Default for PasswordEntryBufferBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl PasswordEntryBufferBuilder {
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn max_length(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_length", value);
        self
    }
    pub fn bind(&mut self) -> PasswordEntryBufferBinder {
        PasswordEntryBufferBinder { builder: self }
    }
    pub fn connect(&mut self) -> PasswordEntryBufferSignals {
        PasswordEntryBufferSignals { builder: self }
    }
}
impl crate::prelude::Builder for PasswordEntryBufferBuilder {
    type Target = PasswordEntryBuffer;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for PasswordEntryBufferBuilder {
    type Target = PasswordEntryBuffer;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct PasswordEntryBufferBinder<'builder> {
    builder: &'builder mut PasswordEntryBufferBuilder,
}
impl<'builder> PasswordEntryBufferBinder<'builder> {
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PasswordEntryBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn max_length(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut PasswordEntryBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_length", val));
        value.connect_update(move |value| obj.set_property("max_length", value));
        self.builder
    }
}
pub struct PasswordEntryBufferSignals<'builder> {
    builder: &'builder mut PasswordEntryBufferBuilder,
}
impl<'builder> PasswordEntryBufferSignals<'builder> {}
impl ForteExt for PasswordEntryBuffer {
    type Builder = PasswordEntryBufferBuilder;
}
#[macro_export]
macro_rules ! PasswordEntryBuffer { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: PasswordEntryBuffer :: forte ()) $ ($ rest) * } } }
