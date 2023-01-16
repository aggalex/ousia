#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, EventControllerKey, *};
use gtkrs::{EventController, IMContext, PropagationLimit, PropagationPhase, Widget};
#[derive(Clone)]
pub struct EventControllerKeyBuilder {
    obj: EventControllerKey,
}
impl Default for EventControllerKeyBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EventControllerKeyBuilder {
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> EventControllerKeyBinder {
        EventControllerKeyBinder { builder: self }
    }
    pub fn connect(&mut self) -> EventControllerKeySignals {
        EventControllerKeySignals { builder: self }
    }
}
impl crate::prelude::Builder for EventControllerKeyBuilder {
    type Target = EventControllerKey;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EventControllerKeyBuilder {
    type Target = EventControllerKey;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EventControllerKeyBinder<'builder> {
    builder: &'builder mut EventControllerKeyBuilder,
}
impl<'builder> EventControllerKeyBinder<'builder> {
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut EventControllerKeyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut EventControllerKeyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EventControllerKeyBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct EventControllerKeySignals<'builder> {
    builder: &'builder mut EventControllerKeyBuilder,
}
impl<'builder> EventControllerKeySignals<'builder> {}
impl ForteExt for EventControllerKey {
    type Builder = EventControllerKeyBuilder;
}
#[macro_export]
macro_rules ! EventControllerKey { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: EventControllerKey :: forte ()) $ ($ rest) * } } }
