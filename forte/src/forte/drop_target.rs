#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, DropTarget, *};
use gtkrs::{EventController, PropagationLimit, PropagationPhase};
#[derive(Clone)]
pub struct DropTargetBuilder {
    obj: DropTarget,
}
impl Default for DropTargetBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl DropTargetBuilder {
    pub fn propagation_limit(&mut self, value: PropagationLimit) -> &mut Self {
        self.obj.set_property("propagation_limit", value);
        self
    }
    pub fn actions(&mut self, value: gdk::DragAction) -> &mut Self {
        self.obj.set_property("actions", value);
        self
    }
    pub fn formats(&mut self, value: &gdk::ContentFormats) -> &mut Self {
        self.obj.set_property("formats", value);
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
    pub fn preload(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("preload", value);
        self
    }
    pub fn bind(&mut self) -> DropTargetBinder {
        DropTargetBinder { builder: self }
    }
    pub fn connect(&mut self) -> DropTargetSignals {
        DropTargetSignals { builder: self }
    }
}
impl crate::prelude::Builder for DropTargetBuilder {
    type Target = DropTarget;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for DropTargetBuilder {
    type Target = DropTarget;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct DropTargetBinder<'builder> {
    builder: &'builder mut DropTargetBuilder,
}
impl<'builder> DropTargetBinder<'builder> {
    pub fn propagation_limit(
        &mut self,
        value: &(impl Prop<PropagationLimit> + ?Sized),
    ) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_limit", val));
        value.connect_update(move |value| obj.set_property("propagation_limit", value));
        self.builder
    }
    pub fn actions(
        &mut self,
        value: &(impl Prop<gdk::DragAction> + ?Sized),
    ) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("actions", val));
        value.connect_update(move |value| obj.set_property("actions", value));
        self.builder
    }
    pub fn formats(
        &mut self,
        value: &(impl Prop<gdk::ContentFormats> + ?Sized),
    ) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("formats", val));
        value.connect_update(move |value| obj.set_property("formats", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn propagation_phase(
        &mut self,
        value: &(impl Prop<PropagationPhase> + ?Sized),
    ) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagation_phase", val));
        value.connect_update(move |value| obj.set_property("propagation_phase", value));
        self.builder
    }
    pub fn preload(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut DropTargetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("preload", val));
        value.connect_update(move |value| obj.set_property("preload", value));
        self.builder
    }
}
pub struct DropTargetSignals<'builder> {
    builder: &'builder mut DropTargetBuilder,
}
impl<'builder> DropTargetSignals<'builder> {}
impl ForteExt for DropTarget {
    type Builder = DropTargetBuilder;
}
#[macro_export]
macro_rules ! DropTarget { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: DropTarget :: forte ()) $ ($ rest) * } } }
