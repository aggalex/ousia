#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, GestureRotate, *};
use gtkrs::{EventController, Gesture, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct GestureRotateBuilder {
    obj: GestureRotate,
}
impl Default for GestureRotateBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl GestureRotateBuilder {
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn n_points(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("n_points", value);
        self
    }
    pub fn bind(&mut self) -> GestureRotateBinder {
        GestureRotateBinder { builder: self }
    }
    pub fn connect(&mut self) -> GestureRotateSignals {
        GestureRotateSignals { builder: self }
    }
}
impl crate::prelude::Builder for GestureRotateBuilder {
    type Target = GestureRotate;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for GestureRotateBuilder {
    type Target = GestureRotate;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct GestureRotateBinder<'builder> {
    builder: &'builder mut GestureRotateBuilder,
}
impl<'builder> GestureRotateBinder<'builder> {
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut GestureRotateBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut GestureRotateBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut GestureRotateBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn n_points(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GestureRotateBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("n_points", val));
        value.connect_update(move |value| obj.set_property("n_points", value));
        self.builder
    }
}
pub struct GestureRotateSignals<'builder> {
    builder: &'builder mut GestureRotateBuilder,
}
impl<'builder> GestureRotateSignals<'builder> {}
impl ForteExt for GestureRotate {
    type Builder = GestureRotateBuilder;
}
#[macro_export]
macro_rules ! GestureRotate { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: GestureRotate :: forte ()) $ ($ rest) * } } }
