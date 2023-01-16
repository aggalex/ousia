#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EventControllerLegacy, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct EventControllerLegacyBuilder {
    obj: EventControllerLegacy,
}
impl Default for EventControllerLegacyBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EventControllerLegacyBuilder {
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn bind(&mut self) -> EventControllerLegacyBinder {
        EventControllerLegacyBinder { builder: self }
    }
    pub fn connect(&mut self) -> EventControllerLegacySignals {
        EventControllerLegacySignals { builder: self }
    }
}
impl crate::prelude::Builder for EventControllerLegacyBuilder {
    type Target = EventControllerLegacy;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EventControllerLegacyBuilder {
    type Target = EventControllerLegacy;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EventControllerLegacyBinder<'builder> {
    builder: &'builder mut EventControllerLegacyBuilder,
}
impl<'builder> EventControllerLegacyBinder<'builder> {
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut EventControllerLegacyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EventControllerLegacyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut EventControllerLegacyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
}
pub struct EventControllerLegacySignals<'builder> {
    builder: &'builder mut EventControllerLegacyBuilder,
}
impl<'builder> EventControllerLegacySignals<'builder> {}
impl ForteExt for EventControllerLegacy {
    type Builder = EventControllerLegacyBuilder;
}
#[macro_export]
macro_rules ! EventControllerLegacy { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EventControllerLegacy :: forte ()) $ ($ rest) * } } }
