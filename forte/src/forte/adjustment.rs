#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Adjustment, *};
#[derive(Clone)]
pub struct AdjustmentBuilder {
    obj: Adjustment,
}
impl Default for AdjustmentBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl AdjustmentBuilder {
    pub fn page_increment(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("page_increment", value);
        self
    }
    pub fn upper(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("upper", value);
        self
    }
    pub fn value(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("value", value);
        self
    }
    pub fn page_size(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("page_size", value);
        self
    }
    pub fn step_increment(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("step_increment", value);
        self
    }
    pub fn lower(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("lower", value);
        self
    }
    pub fn bind(&mut self) -> AdjustmentBinder {
        AdjustmentBinder { builder: self }
    }
    pub fn connect(&mut self) -> AdjustmentSignals {
        AdjustmentSignals { builder: self }
    }
}
impl crate::prelude::Builder for AdjustmentBuilder {
    type Target = Adjustment;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for AdjustmentBuilder {
    type Target = Adjustment;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct AdjustmentBinder<'builder> {
    builder: &'builder mut AdjustmentBuilder,
}
impl<'builder> AdjustmentBinder<'builder> {
    pub fn page_increment(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("page_increment", val));
        value.connect_update(move |value| obj.set_property("page_increment", value));
        self.builder
    }
    pub fn upper(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("upper", val));
        value.connect_update(move |value| obj.set_property("upper", value));
        self.builder
    }
    pub fn value(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("value", val));
        value.connect_update(move |value| obj.set_property("value", value));
        self.builder
    }
    pub fn page_size(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("page_size", val));
        value.connect_update(move |value| obj.set_property("page_size", value));
        self.builder
    }
    pub fn step_increment(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("step_increment", val));
        value.connect_update(move |value| obj.set_property("step_increment", value));
        self.builder
    }
    pub fn lower(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("lower", val));
        value.connect_update(move |value| obj.set_property("lower", value));
        self.builder
    }
}
pub struct AdjustmentSignals<'builder> {
    builder: &'builder mut AdjustmentBuilder,
}
impl<'builder> AdjustmentSignals<'builder> {
    pub fn page_size_notify(
        &mut self,
        f: impl Fn(&Adjustment) + 'static,
    ) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_page_size_notify(f);
        &mut self.builder
    }
    pub fn step_increment_notify(
        &mut self,
        f: impl Fn(&Adjustment) + 'static,
    ) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_step_increment_notify(f);
        &mut self.builder
    }
    pub fn page_increment_notify(
        &mut self,
        f: impl Fn(&Adjustment) + 'static,
    ) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_page_increment_notify(f);
        &mut self.builder
    }
    pub fn value_notify(&mut self, f: impl Fn(&Adjustment) + 'static) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_value_notify(f);
        &mut self.builder
    }
    pub fn lower_notify(&mut self, f: impl Fn(&Adjustment) + 'static) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_lower_notify(f);
        &mut self.builder
    }
    pub fn changed(&mut self, f: impl Fn(&Adjustment) + 'static) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_changed(f);
        &mut self.builder
    }
    pub fn upper_notify(&mut self, f: impl Fn(&Adjustment) + 'static) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_upper_notify(f);
        &mut self.builder
    }
    pub fn value_changed(&mut self, f: impl Fn(&Adjustment) + 'static) -> &mut AdjustmentBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_value_changed(f);
        &mut self.builder
    }
}
impl ForteExt for Adjustment {
    type Builder = AdjustmentBuilder;
}
#[macro_export]
macro_rules ! Adjustment { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Adjustment :: forte ()) $ ($ rest) * } } }
