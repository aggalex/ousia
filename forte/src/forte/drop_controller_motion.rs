#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, DropControllerMotion, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct DropControllerMotionBuilder {
    obj: DropControllerMotion,
}
impl Default for DropControllerMotionBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl DropControllerMotionBuilder {
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> DropControllerMotionBinder {
        DropControllerMotionBinder { builder: self }
    }
    pub fn connect(&mut self) -> DropControllerMotionSignals {
        DropControllerMotionSignals { builder: self }
    }
}
impl crate::prelude::Builder for DropControllerMotionBuilder {
    type Target = DropControllerMotion;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for DropControllerMotionBuilder {
    type Target = DropControllerMotion;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct DropControllerMotionBinder<'builder> {
    builder: &'builder mut DropControllerMotionBuilder,
}
impl<'builder> DropControllerMotionBinder<'builder> {
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut DropControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut DropControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut DropControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct DropControllerMotionSignals<'builder> {
    builder: &'builder mut DropControllerMotionBuilder,
}
impl<'builder> DropControllerMotionSignals<'builder> {}
impl ForteExt for DropControllerMotion {
    type Builder = DropControllerMotionBuilder;
}
#[macro_export]
macro_rules ! DropControllerMotion { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: DropControllerMotion :: forte ()) $ ($ rest) * } } }
