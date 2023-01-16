#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EventControllerMotion, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct EventControllerMotionBuilder {
    obj: EventControllerMotion,
}
impl Default for EventControllerMotionBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EventControllerMotionBuilder {
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn bind(&mut self) -> EventControllerMotionBinder {
        EventControllerMotionBinder { builder: self }
    }
    pub fn connect(&mut self) -> EventControllerMotionSignals {
        EventControllerMotionSignals { builder: self }
    }
}
impl crate::prelude::Builder for EventControllerMotionBuilder {
    type Target = EventControllerMotion;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EventControllerMotionBuilder {
    type Target = EventControllerMotion;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EventControllerMotionBinder<'builder> {
    builder: &'builder mut EventControllerMotionBuilder,
}
impl<'builder> EventControllerMotionBinder<'builder> {
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EventControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut EventControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut EventControllerMotionBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
}
pub struct EventControllerMotionSignals<'builder> {
    builder: &'builder mut EventControllerMotionBuilder,
}
impl<'builder> EventControllerMotionSignals<'builder> {}
impl ForteExt for EventControllerMotion {
    type Builder = EventControllerMotionBuilder;
}
#[macro_export]
macro_rules ! EventControllerMotion { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EventControllerMotion :: forte ()) $ ($ rest) * } } }
