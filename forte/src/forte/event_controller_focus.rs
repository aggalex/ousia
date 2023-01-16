#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EventControllerFocus, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct EventControllerFocusBuilder {
    obj: EventControllerFocus,
}
impl Default for EventControllerFocusBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EventControllerFocusBuilder {
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
    pub fn bind(&mut self) -> EventControllerFocusBinder {
        EventControllerFocusBinder { builder: self }
    }
    pub fn connect(&mut self) -> EventControllerFocusSignals {
        EventControllerFocusSignals { builder: self }
    }
}
impl crate::prelude::Builder for EventControllerFocusBuilder {
    type Target = EventControllerFocus;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EventControllerFocusBuilder {
    type Target = EventControllerFocus;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EventControllerFocusBinder<'builder> {
    builder: &'builder mut EventControllerFocusBuilder,
}
impl<'builder> EventControllerFocusBinder<'builder> {
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EventControllerFocusBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut EventControllerFocusBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut EventControllerFocusBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
}
pub struct EventControllerFocusSignals<'builder> {
    builder: &'builder mut EventControllerFocusBuilder,
}
impl<'builder> EventControllerFocusSignals<'builder> {}
impl ForteExt for EventControllerFocus {
    type Builder = EventControllerFocusBuilder;
}
#[macro_export]
macro_rules ! EventControllerFocus { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EventControllerFocus :: forte ()) $ ($ rest) * } } }
