#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ConstraintGuide, *};
use gtkrs::{ConstraintStrength, ConstraintTarget};
#[derive(Clone)]
pub struct ConstraintGuideBuilder {
    obj: ConstraintGuide,
}
impl Default for ConstraintGuideBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ConstraintGuideBuilder {
    pub fn max_height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_height", value);
        self
    }
    pub fn min_height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("min_height", value);
        self
    }
    pub fn max_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_width", value);
        self
    }
    pub fn nat_height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("nat_height", value);
        self
    }
    pub fn nat_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("nat_width", value);
        self
    }
    pub fn strength(&mut self, value: ConstraintStrength) -> &mut Self {
        self.obj.set_property("strength", value);
        self
    }
    pub fn min_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("min_width", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn bind(&mut self) -> ConstraintGuideBinder {
        ConstraintGuideBinder { builder: self }
    }
    pub fn connect(&mut self) -> ConstraintGuideSignals {
        ConstraintGuideSignals { builder: self }
    }
}
impl crate::prelude::Builder for ConstraintGuideBuilder {
    type Target = ConstraintGuide;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ConstraintGuideBuilder {
    type Target = ConstraintGuide;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ConstraintGuideBinder<'builder> {
    builder: &'builder mut ConstraintGuideBuilder,
}
impl<'builder> ConstraintGuideBinder<'builder> {
    pub fn max_height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_height", val));
        value.connect_update(move |value| obj.set_property("max_height", value));
        self.builder
    }
    pub fn min_height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("min_height", val));
        value.connect_update(move |value| obj.set_property("min_height", value));
        self.builder
    }
    pub fn max_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width", val));
        value.connect_update(move |value| obj.set_property("max_width", value));
        self.builder
    }
    pub fn nat_height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("nat_height", val));
        value.connect_update(move |value| obj.set_property("nat_height", value));
        self.builder
    }
    pub fn nat_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("nat_width", val));
        value.connect_update(move |value| obj.set_property("nat_width", value));
        self.builder
    }
    pub fn strength(
        &mut self,
        value: &(impl Prop<ConstraintStrength> + ?Sized),
    ) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strength", val));
        value.connect_update(move |value| obj.set_property("strength", value));
        self.builder
    }
    pub fn min_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("min_width", val));
        value.connect_update(move |value| obj.set_property("min_width", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ConstraintGuideBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
}
pub struct ConstraintGuideSignals<'builder> {
    builder: &'builder mut ConstraintGuideBuilder,
}
impl<'builder> ConstraintGuideSignals<'builder> {}
impl ForteExt for ConstraintGuide {
    type Builder = ConstraintGuideBuilder;
}
#[macro_export]
macro_rules ! ConstraintGuide { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ConstraintGuide :: forte ()) $ ($ rest) * } } }
