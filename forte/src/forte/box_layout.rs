#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, BoxLayout, *};
use gtkrs::{BaselinePosition, LayoutManager, Orientable, Orientation};
#[derive(Clone)]
pub struct BoxLayoutBuilder {
    obj: BoxLayout,
}
impl Default for BoxLayoutBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl BoxLayoutBuilder {
    pub fn homogeneous(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("homogeneous", value);
        self
    }
    pub fn spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("spacing", value);
        self
    }
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn baseline_position(&mut self, value: BaselinePosition) -> &mut Self {
        self.obj.set_property("baseline_position", value);
        self
    }
    pub fn bind(&mut self) -> BoxLayoutBinder {
        BoxLayoutBinder { builder: self }
    }
    pub fn connect(&mut self) -> BoxLayoutSignals {
        BoxLayoutSignals { builder: self }
    }
}
impl crate::prelude::Builder for BoxLayoutBuilder {
    type Target = BoxLayout;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for BoxLayoutBuilder {
    type Target = BoxLayout;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct BoxLayoutBinder<'builder> {
    builder: &'builder mut BoxLayoutBuilder,
}
impl<'builder> BoxLayoutBinder<'builder> {
    pub fn homogeneous(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("homogeneous", val));
        value.connect_update(move |value| obj.set_property("homogeneous", value));
        self.builder
    }
    pub fn spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("spacing", val));
        value.connect_update(move |value| obj.set_property("spacing", value));
        self.builder
    }
    pub fn orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut BoxLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn baseline_position(
        &mut self,
        value: &(impl Prop<BaselinePosition> + ?Sized),
    ) -> &mut BoxLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("baseline_position", val));
        value.connect_update(move |value| obj.set_property("baseline_position", value));
        self.builder
    }
}
pub struct BoxLayoutSignals<'builder> {
    builder: &'builder mut BoxLayoutBuilder,
}
impl<'builder> BoxLayoutSignals<'builder> {}
impl ForteExt for BoxLayout {
    type Builder = BoxLayoutBuilder;
}
#[macro_export]
macro_rules ! BoxLayout { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: BoxLayout :: forte ()) $ ($ rest) * } } }
