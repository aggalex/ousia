#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, GesturePan, *};
use gtkrs::{
    EventController, Gesture, GestureDrag, GestureSingle, Orientation, PanDirection,
    PropagationLimit, PropagationPhase,
};
#[derive(Clone)]
pub struct GesturePanBuilder {
    obj: GesturePan,
}
impl Default for GesturePanBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl GesturePanBuilder {
    pub fn touch_only(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("touch_only", value);
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
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn exclusive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("exclusive", value);
        self
    }
    pub fn button(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("button", value);
        self
    }
    pub fn n_points(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("n_points", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> GesturePanBinder {
        GesturePanBinder { builder: self }
    }
    pub fn connect(&mut self) -> GesturePanSignals {
        GesturePanSignals { builder: self }
    }
}
impl crate::prelude::Builder for GesturePanBuilder {
    type Target = GesturePan;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for GesturePanBuilder {
    type Target = GesturePan;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct GesturePanBinder<'builder> {
    builder: &'builder mut GesturePanBuilder,
}
impl<'builder> GesturePanBinder<'builder> {
    pub fn touch_only(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("touch_only", val));
        value.connect_update(move |value| obj.set_property("touch_only", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn exclusive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("exclusive", val));
        value.connect_update(move |value| obj.set_property("exclusive", value));
        self.builder
    }
    pub fn button(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("button", val));
        value.connect_update(move |value| obj.set_property("button", value));
        self.builder
    }
    pub fn n_points(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("n_points", val));
        value.connect_update(move |value| obj.set_property("n_points", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut GesturePanBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct GesturePanSignals<'builder> {
    builder: &'builder mut GesturePanBuilder,
}
impl<'builder> GesturePanSignals<'builder> {}
impl ForteExt for GesturePan {
    type Builder = GesturePanBuilder;
}
#[macro_export]
macro_rules ! GesturePan { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: GesturePan :: forte ()) $ ($ rest) * } } }
