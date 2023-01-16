#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::{prelude::*, IMContextSimple, *};
use gtkrs::{IMContext, InputHints, InputPurpose};
#[derive(Clone)]
pub struct IMContextSimpleBuilder {
    obj: IMContextSimple,
}
impl Default for IMContextSimpleBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl IMContextSimpleBuilder {
    pub fn input_hints(&mut self, value: InputHints) -> &mut Self {
        self.obj.set_property("input_hints", value);
        self
    }
    pub fn input_purpose(&mut self, value: InputPurpose) -> &mut Self {
        self.obj.set_property("input_purpose", value);
        self
    }
    pub fn bind(&mut self) -> IMContextSimpleBinder {
        IMContextSimpleBinder { builder: self }
    }
    pub fn connect(&mut self) -> IMContextSimpleSignals {
        IMContextSimpleSignals { builder: self }
    }
}
impl crate::prelude::Builder for IMContextSimpleBuilder {
    type Target = IMContextSimple;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for IMContextSimpleBuilder {
    type Target = IMContextSimple;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct IMContextSimpleBinder<'builder> {
    builder: &'builder mut IMContextSimpleBuilder,
}
impl<'builder> IMContextSimpleBinder<'builder> {
    pub fn input_hints(
        &mut self,
        value: &(impl Prop<InputHints> + ?Sized),
    ) -> &mut IMContextSimpleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_hints", val));
        value.connect_update(move |value| obj.set_property("input_hints", value));
        self.builder
    }
    pub fn input_purpose(
        &mut self,
        value: &(impl Prop<InputPurpose> + ?Sized),
    ) -> &mut IMContextSimpleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_purpose", val));
        value.connect_update(move |value| obj.set_property("input_purpose", value));
        self.builder
    }
}
pub struct IMContextSimpleSignals<'builder> {
    builder: &'builder mut IMContextSimpleBuilder,
}
impl<'builder> IMContextSimpleSignals<'builder> {}
impl ForteExt for IMContextSimple {
    type Builder = IMContextSimpleBuilder;
}
#[macro_export]
macro_rules ! IMContextSimple { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: IMContextSimple :: forte ()) $ ($ rest) * } } }
