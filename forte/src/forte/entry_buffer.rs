#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EntryBuffer, *};
#[derive(Clone)]
pub struct EntryBufferBuilder {
    obj: EntryBuffer,
}
impl Default for EntryBufferBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EntryBufferBuilder {
    pub fn max_length(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_length", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn bind(&mut self) -> EntryBufferBinder {
        EntryBufferBinder { builder: self }
    }
    pub fn connect(&mut self) -> EntryBufferSignals {
        EntryBufferSignals { builder: self }
    }
}
impl crate::prelude::Builder for EntryBufferBuilder {
    type Target = EntryBuffer;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EntryBufferBuilder {
    type Target = EntryBuffer;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EntryBufferBinder<'builder> {
    builder: &'builder mut EntryBufferBuilder,
}
impl<'builder> EntryBufferBinder<'builder> {
    pub fn max_length(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_length", val));
        value.connect_update(move |value| obj.set_property("max_length", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
}
pub struct EntryBufferSignals<'builder> {
    builder: &'builder mut EntryBufferBuilder,
}
impl<'builder> EntryBufferSignals<'builder> {
    pub fn length_notify(&mut self, f: impl Fn(&EntryBuffer) + 'static) -> &mut EntryBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_length_notify(f);
        &mut self.builder
    }
    pub fn text_notify(&mut self, f: impl Fn(&EntryBuffer) + 'static) -> &mut EntryBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_text_notify(f);
        &mut self.builder
    }
    pub fn max_length_notify(
        &mut self,
        f: impl Fn(&EntryBuffer) + 'static,
    ) -> &mut EntryBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_max_length_notify(f);
        &mut self.builder
    }
}
impl ForteExt for EntryBuffer {
    type Builder = EntryBufferBuilder;
}
#[macro_export]
macro_rules ! EntryBuffer { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EntryBuffer :: forte ()) $ ($ rest) * } } }
