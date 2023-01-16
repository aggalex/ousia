#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, DropTargetAsync, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct DropTargetAsyncBuilder {
    obj: DropTargetAsync,
}
impl Default for DropTargetAsyncBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl DropTargetAsyncBuilder {
    pub fn formats(&mut self, value: &gdk::ContentFormats) -> &mut Self {
        self.obj.set_property("formats", value);
        self
    }
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn actions(&mut self, value: gdk::DragAction) -> &mut Self {
        self.obj.set_property("actions", value);
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
    pub fn bind(&mut self) -> DropTargetAsyncBinder {
        DropTargetAsyncBinder { builder: self }
    }
    pub fn connect(&mut self) -> DropTargetAsyncSignals {
        DropTargetAsyncSignals { builder: self }
    }
}
impl crate::prelude::Builder for DropTargetAsyncBuilder {
    type Target = DropTargetAsync;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for DropTargetAsyncBuilder {
    type Target = DropTargetAsync;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct DropTargetAsyncBinder<'builder> {
    builder: &'builder mut DropTargetAsyncBuilder,
}
impl<'builder> DropTargetAsyncBinder<'builder> {
    pub fn formats(
        &mut self,
        value: &(impl Prop<gdk::ContentFormats> + ?Sized),
    ) -> &mut DropTargetAsyncBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("formats", val));
        value.connect_update(move |value| obj.set_property("formats", value));
        self.builder
    }
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut DropTargetAsyncBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn actions(
        &mut self,
        value: &(impl Prop<gdk::DragAction> + ?Sized),
    ) -> &mut DropTargetAsyncBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("actions", val));
        value.connect_update(move |value| obj.set_property("actions", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut DropTargetAsyncBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut DropTargetAsyncBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
}
pub struct DropTargetAsyncSignals<'builder> {
    builder: &'builder mut DropTargetAsyncBuilder,
}
impl<'builder> DropTargetAsyncSignals<'builder> {}
impl ForteExt for DropTargetAsync {
    type Builder = DropTargetAsyncBuilder;
}
#[macro_export]
macro_rules ! DropTargetAsync { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: DropTargetAsync :: forte ()) $ ($ rest) * } } }
