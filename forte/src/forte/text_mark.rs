#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::TextBuffer;
use gtkrs::{prelude::*, TextMark, *};
#[derive(Clone)]
pub struct TextMarkBuilder {
    obj: TextMark,
}
impl Default for TextMarkBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TextMarkBuilder {
    pub fn left_gravity(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("left_gravity", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> TextMarkBinder {
        TextMarkBinder { builder: self }
    }
    pub fn connect(&mut self) -> TextMarkSignals {
        TextMarkSignals { builder: self }
    }
}
impl crate::prelude::Builder for TextMarkBuilder {
    type Target = TextMark;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TextMarkBuilder {
    type Target = TextMark;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TextMarkBinder<'builder> {
    builder: &'builder mut TextMarkBuilder,
}
impl<'builder> TextMarkBinder<'builder> {
    pub fn left_gravity(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextMarkBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("left_gravity", val));
        value.connect_update(move |value| obj.set_property("left_gravity", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextMarkBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct TextMarkSignals<'builder> {
    builder: &'builder mut TextMarkBuilder,
}
impl<'builder> TextMarkSignals<'builder> {}
impl ForteExt for TextMark {
    type Builder = TextMarkBuilder;
}
#[macro_export]
macro_rules ! TextMark { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TextMark :: forte ()) $ ($ rest) * } } }
