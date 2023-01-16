#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, GestureLongPress, *};
use gtkrs::{EventController, Gesture, GestureSingle, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct GestureLongPressBuilder {
    obj: GestureLongPress,
}
impl Default for GestureLongPressBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl GestureLongPressBuilder {
    pub fn n_points(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("n_points", value);
        self
    }
    pub fn exclusive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("exclusive", value);
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
    pub fn delay_factor(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("delay_factor", value);
        self
    }
    pub fn button(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("button", value);
        self
    }
    pub fn touch_only(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("touch_only", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> GestureLongPressBinder {
        GestureLongPressBinder { builder: self }
    }
    pub fn connect(&mut self) -> GestureLongPressSignals {
        GestureLongPressSignals { builder: self }
    }
}
impl crate::prelude::Builder for GestureLongPressBuilder {
    type Target = GestureLongPress;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for GestureLongPressBuilder {
    type Target = GestureLongPress;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct GestureLongPressBinder<'builder> {
    builder: &'builder mut GestureLongPressBuilder,
}
impl<'builder> GestureLongPressBinder<'builder> {
    pub fn n_points(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("n_points", val));
        value.connect_update(move |value| obj.set_property("n_points", value));
        self.builder
    }
    pub fn exclusive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("exclusive", val));
        value.connect_update(move |value| obj.set_property("exclusive", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn delay_factor(
        &mut self,
        value: &(impl Prop<f64> + ?Sized),
    ) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("delay_factor", val));
        value.connect_update(move |value| obj.set_property("delay_factor", value));
        self.builder
    }
    pub fn button(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("button", val));
        value.connect_update(move |value| obj.set_property("button", value));
        self.builder
    }
    pub fn touch_only(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("touch_only", val));
        value.connect_update(move |value| obj.set_property("touch_only", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut GestureLongPressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct GestureLongPressSignals<'builder> {
    builder: &'builder mut GestureLongPressBuilder,
}
impl<'builder> GestureLongPressSignals<'builder> {}
impl ForteExt for GestureLongPress {
    type Builder = GestureLongPressBuilder;
}
#[macro_export]
macro_rules ! GestureLongPress { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: GestureLongPress :: forte ()) $ ($ rest) * } } }
