#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EventControllerScroll, *};
use gtkrs::{EventController, EventControllerScrollFlags, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct EventControllerScrollBuilder {
    obj: EventControllerScroll,
}
impl Default for EventControllerScrollBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EventControllerScrollBuilder {
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn flags(&mut self, value: EventControllerScrollFlags) -> &mut Self {
        self.obj.set_property("flags", value);
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
    pub fn bind(&mut self) -> EventControllerScrollBinder {
        EventControllerScrollBinder { builder: self }
    }
    pub fn connect(&mut self) -> EventControllerScrollSignals {
        EventControllerScrollSignals { builder: self }
    }
}
impl crate::prelude::Builder for EventControllerScrollBuilder {
    type Target = EventControllerScroll;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EventControllerScrollBuilder {
    type Target = EventControllerScroll;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EventControllerScrollBinder<'builder> {
    builder: &'builder mut EventControllerScrollBuilder,
}
impl<'builder> EventControllerScrollBinder<'builder> {
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut EventControllerScrollBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn flags(
        &mut self,
        value: &(impl Prop<EventControllerScrollFlags> + ?Sized),
    ) -> &mut EventControllerScrollBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("flags", val));
        value.connect_update(move |value| obj.set_property("flags", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EventControllerScrollBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut EventControllerScrollBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
}
pub struct EventControllerScrollSignals<'builder> {
    builder: &'builder mut EventControllerScrollBuilder,
}
impl<'builder> EventControllerScrollSignals<'builder> {}
impl ForteExt for EventControllerScroll {
    type Builder = EventControllerScrollBuilder;
}
#[macro_export]
macro_rules ! EventControllerScroll { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EventControllerScroll :: forte ()) $ ($ rest) * } } }
