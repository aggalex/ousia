#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, GridLayout, *};
use gtkrs::{BaselinePosition, LayoutManager};
#[derive(Clone)]
pub struct GridLayoutBuilder {
    obj: GridLayout,
}
impl Default for GridLayoutBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl GridLayoutBuilder {
    pub fn row_spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("row_spacing", value);
        self
    }
    pub fn row_homogeneous(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("row_homogeneous", value);
        self
    }
    pub fn column_homogeneous(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("column_homogeneous", value);
        self
    }
    pub fn column_spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("column_spacing", value);
        self
    }
    pub fn baseline_row(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("baseline_row", value);
        self
    }
    pub fn bind(&mut self) -> GridLayoutBinder {
        GridLayoutBinder { builder: self }
    }
    pub fn connect(&mut self) -> GridLayoutSignals {
        GridLayoutSignals { builder: self }
    }
}
impl crate::prelude::Builder for GridLayoutBuilder {
    type Target = GridLayout;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for GridLayoutBuilder {
    type Target = GridLayout;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct GridLayoutBinder<'builder> {
    builder: &'builder mut GridLayoutBuilder,
}
impl<'builder> GridLayoutBinder<'builder> {
    pub fn row_spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut GridLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("row_spacing", val));
        value.connect_update(move |value| obj.set_property("row_spacing", value));
        self.builder
    }
    pub fn row_homogeneous(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut GridLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("row_homogeneous", val));
        value.connect_update(move |value| obj.set_property("row_homogeneous", value));
        self.builder
    }
    pub fn column_homogeneous(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut GridLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("column_homogeneous", val));
        value.connect_update(move |value| obj.set_property("column_homogeneous", value));
        self.builder
    }
    pub fn column_spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut GridLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("column_spacing", val));
        value.connect_update(move |value| obj.set_property("column_spacing", value));
        self.builder
    }
    pub fn baseline_row(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut GridLayoutBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("baseline_row", val));
        value.connect_update(move |value| obj.set_property("baseline_row", value));
        self.builder
    }
}
pub struct GridLayoutSignals<'builder> {
    builder: &'builder mut GridLayoutBuilder,
}
impl<'builder> GridLayoutSignals<'builder> {}
impl ForteExt for GridLayout {
    type Builder = GridLayoutBuilder;
}
#[macro_export]
macro_rules ! GridLayout { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: GridLayout :: forte ()) $ ($ rest) * } } }
