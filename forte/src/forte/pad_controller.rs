#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::{prelude::*, PadController, *};
use gtkrs::{EventController, PadActionType, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct PadControllerBuilder {
    obj: PadController,
}
impl Default for PadControllerBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl PadControllerBuilder {
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn pad(&mut self, value: &gdk::Device) -> &mut Self {
        self.obj.set_property("pad", value);
        self
    }
    pub fn action_group(&mut self, value: &impl IsA<gio::ActionGroup>) -> &mut Self {
        self.obj.set_property("action_group", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> PadControllerBinder {
        PadControllerBinder { builder: self }
    }
    pub fn connect(&mut self) -> PadControllerSignals {
        PadControllerSignals { builder: self }
    }
}
impl crate::prelude::Builder for PadControllerBuilder {
    type Target = PadController;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for PadControllerBuilder {
    type Target = PadController;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct PadControllerBinder<'builder> {
    builder: &'builder mut PadControllerBuilder,
}
impl<'builder> PadControllerBinder<'builder> {
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut PadControllerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut PadControllerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn pad(&mut self, value: &(impl Prop<gdk::Device> + ?Sized)) -> &mut PadControllerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pad", val));
        value.connect_update(move |value| obj.set_property("pad", value));
        self.builder
    }
    pub fn action_group<T: IsA<gio::ActionGroup>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut PadControllerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action_group", val));
        value.connect_update(move |value| obj.set_property("action_group", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PadControllerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct PadControllerSignals<'builder> {
    builder: &'builder mut PadControllerBuilder,
}
impl<'builder> PadControllerSignals<'builder> {}
impl ForteExt for PadController {
    type Builder = PadControllerBuilder;
}
#[macro_export]
macro_rules ! PadController { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: PadController :: forte ()) $ ($ rest) * } } }
