#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::{prelude::*, IMMulticontext, *};
use gtkrs::{IMContext, InputHints, InputPurpose};
#[derive(Clone)]
pub struct IMMulticontextBuilder {
    obj: IMMulticontext,
}
impl Default for IMMulticontextBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl IMMulticontextBuilder {
    pub fn input_hints(&mut self, value: InputHints) -> &mut Self {
        self.obj.set_property("input_hints", value);
        self
    }
    pub fn input_purpose(&mut self, value: InputPurpose) -> &mut Self {
        self.obj.set_property("input_purpose", value);
        self
    }
    pub fn bind(&mut self) -> IMMulticontextBinder {
        IMMulticontextBinder { builder: self }
    }
    pub fn connect(&mut self) -> IMMulticontextSignals {
        IMMulticontextSignals { builder: self }
    }
}
impl crate::prelude::Builder for IMMulticontextBuilder {
    type Target = IMMulticontext;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for IMMulticontextBuilder {
    type Target = IMMulticontext;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct IMMulticontextBinder<'builder> {
    builder: &'builder mut IMMulticontextBuilder,
}
impl<'builder> IMMulticontextBinder<'builder> {
    pub fn input_hints(
        &mut self,
        value: &(impl Prop<InputHints> + ?Sized),
    ) -> &mut IMMulticontextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_hints", val));
        value.connect_update(move |value| obj.set_property("input_hints", value));
        self.builder
    }
    pub fn input_purpose(
        &mut self,
        value: &(impl Prop<InputPurpose> + ?Sized),
    ) -> &mut IMMulticontextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_purpose", val));
        value.connect_update(move |value| obj.set_property("input_purpose", value));
        self.builder
    }
}
pub struct IMMulticontextSignals<'builder> {
    builder: &'builder mut IMMulticontextBuilder,
}
impl<'builder> IMMulticontextSignals<'builder> {}
impl ForteExt for IMMulticontext {
    type Builder = IMMulticontextBuilder;
}
#[macro_export]
macro_rules ! IMMulticontext { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: IMMulticontext :: forte ()) $ ($ rest) * } } }
