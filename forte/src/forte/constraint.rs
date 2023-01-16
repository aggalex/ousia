#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{prelude::*, translate::*};
use gtkrs::{prelude::*, Constraint, *};
use gtkrs::{ConstraintAttribute, ConstraintRelation, ConstraintTarget};
#[derive(Clone)]
pub struct ConstraintBuilder {
    obj: Constraint,
}
impl Default for ConstraintBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ConstraintBuilder {
    pub fn constant(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("constant", value);
        self
    }
    pub fn target_attribute(&mut self, value: ConstraintAttribute) -> &mut Self {
        self.obj.set_property("target_attribute", value);
        self
    }
    pub fn relation(&mut self, value: ConstraintRelation) -> &mut Self {
        self.obj.set_property("relation", value);
        self
    }
    pub fn target(&mut self, value: &impl IsA<ConstraintTarget>) -> &mut Self {
        self.obj.set_property("target", value);
        self
    }
    pub fn source_attribute(&mut self, value: ConstraintAttribute) -> &mut Self {
        self.obj.set_property("source_attribute", value);
        self
    }
    pub fn strength(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("strength", value);
        self
    }
    pub fn multiplier(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("multiplier", value);
        self
    }
    pub fn source(&mut self, value: &impl IsA<ConstraintTarget>) -> &mut Self {
        self.obj.set_property("source", value);
        self
    }
    pub fn bind(&mut self) -> ConstraintBinder {
        ConstraintBinder { builder: self }
    }
    pub fn connect(&mut self) -> ConstraintSignals {
        ConstraintSignals { builder: self }
    }
}
impl crate::prelude::Builder for ConstraintBuilder {
    type Target = Constraint;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ConstraintBuilder {
    type Target = Constraint;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ConstraintBinder<'builder> {
    builder: &'builder mut ConstraintBuilder,
}
impl<'builder> ConstraintBinder<'builder> {
    pub fn constant(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("constant", val));
        value.connect_update(move |value| obj.set_property("constant", value));
        self.builder
    }
    pub fn target_attribute(
        &mut self,
        value: &(impl Prop<ConstraintAttribute> + ?Sized),
    ) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("target_attribute", val));
        value.connect_update(move |value| obj.set_property("target_attribute", value));
        self.builder
    }
    pub fn relation(
        &mut self,
        value: &(impl Prop<ConstraintRelation> + ?Sized),
    ) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("relation", val));
        value.connect_update(move |value| obj.set_property("relation", value));
        self.builder
    }
    pub fn target<T: IsA<ConstraintTarget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("target", val));
        value.connect_update(move |value| obj.set_property("target", value));
        self.builder
    }
    pub fn source_attribute(
        &mut self,
        value: &(impl Prop<ConstraintAttribute> + ?Sized),
    ) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("source_attribute", val));
        value.connect_update(move |value| obj.set_property("source_attribute", value));
        self.builder
    }
    pub fn strength(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strength", val));
        value.connect_update(move |value| obj.set_property("strength", value));
        self.builder
    }
    pub fn multiplier(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("multiplier", val));
        value.connect_update(move |value| obj.set_property("multiplier", value));
        self.builder
    }
    pub fn source<T: IsA<ConstraintTarget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ConstraintBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("source", val));
        value.connect_update(move |value| obj.set_property("source", value));
        self.builder
    }
}
pub struct ConstraintSignals<'builder> {
    builder: &'builder mut ConstraintBuilder,
}
impl<'builder> ConstraintSignals<'builder> {}
impl ForteExt for Constraint {
    type Builder = ConstraintBuilder;
}
#[macro_export]
macro_rules ! Constraint { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Constraint :: forte ()) $ ($ rest) * } } }
