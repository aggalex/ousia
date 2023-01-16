#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, GestureZoom, *};
use gtkrs::{EventController, Gesture, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct GestureZoomBuilder {
    obj: GestureZoom,
}
impl Default for GestureZoomBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl GestureZoomBuilder {
    pub fn n_points(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("n_points", value);
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
    pub fn propagation_phase(&mut self, value: PropagationPhase) -> &mut Self {
        self.obj.set_property("propagation_phase", value);
        self
    }
    pub fn bind(&mut self) -> GestureZoomBinder {
        GestureZoomBinder { builder: self }
    }
    pub fn connect(&mut self) -> GestureZoomSignals {
        GestureZoomSignals { builder: self }
    }
}
impl crate::prelude::Builder for GestureZoomBuilder {
    type Target = GestureZoom;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for GestureZoomBuilder {
    type Target = GestureZoom;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct GestureZoomBinder<'builder> {
    builder: &'builder mut GestureZoomBuilder,
}
impl<'builder> GestureZoomBinder<'builder> {
    pub fn n_points(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GestureZoomBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("n_points", val));
        value.connect_update(move |value| obj.set_property("n_points", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut GestureZoomBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut GestureZoomBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut GestureZoomBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
}
pub struct GestureZoomSignals<'builder> {
    builder: &'builder mut GestureZoomBuilder,
}
impl<'builder> GestureZoomSignals<'builder> {}
impl ForteExt for GestureZoom {
    type Builder = GestureZoomBuilder;
}
#[macro_export]
macro_rules ! GestureZoom { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: GestureZoom :: forte ()) $ ($ rest) * } } }
