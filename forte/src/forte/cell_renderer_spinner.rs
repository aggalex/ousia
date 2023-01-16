#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, CellRendererSpinner, *};
use gtkrs::{CellRenderer, CellRendererMode, IconSize};
#[derive(Clone)]
pub struct CellRendererSpinnerBuilder {
    obj: CellRendererSpinner,
}
impl Default for CellRendererSpinnerBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl CellRendererSpinnerBuilder {
    pub fn pulse(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("pulse", value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("yalign", value);
        self
    }
    pub fn cell_background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("cell_background_rgba", value);
        self
    }
    pub fn cell_background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("cell_background", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn xpad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("xpad", value);
        self
    }
    pub fn ypad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("ypad", value);
        self
    }
    pub fn size(&mut self, value: IconSize) -> &mut Self {
        self.obj.set_property("size", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width", value);
        self
    }
    pub fn mode(&mut self, value: CellRendererMode) -> &mut Self {
        self.obj.set_property("mode", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn active(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("active", value);
        self
    }
    pub fn height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height", value);
        self
    }
    pub fn is_expanded(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expanded", value);
        self
    }
    pub fn is_expander(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expander", value);
        self
    }
    pub fn cell_background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cell_background_set", value);
        self
    }
    pub fn bind(&mut self) -> CellRendererSpinnerBinder {
        CellRendererSpinnerBinder { builder: self }
    }
    pub fn connect(&mut self) -> CellRendererSpinnerSignals {
        CellRendererSpinnerSignals { builder: self }
    }
}
impl crate::prelude::Builder for CellRendererSpinnerBuilder {
    type Target = CellRendererSpinner;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for CellRendererSpinnerBuilder {
    type Target = CellRendererSpinner;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct CellRendererSpinnerBinder<'builder> {
    builder: &'builder mut CellRendererSpinnerBuilder,
}
impl<'builder> CellRendererSpinnerBinder<'builder> {
    pub fn pulse(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pulse", val));
        value.connect_update(move |value| obj.set_property("pulse", value));
        self.builder
    }
    pub fn yalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("yalign", val));
        value.connect_update(move |value| obj.set_property("yalign", value));
        self.builder
    }
    pub fn cell_background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_rgba", val));
        value.connect_update(move |value| obj.set_property("cell_background_rgba", value));
        self.builder
    }
    pub fn cell_background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background", val));
        value.connect_update(move |value| obj.set_property("cell_background", value));
        self.builder
    }
    pub fn visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn xpad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xpad", val));
        value.connect_update(move |value| obj.set_property("xpad", value));
        self.builder
    }
    pub fn ypad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ypad", val));
        value.connect_update(move |value| obj.set_property("ypad", value));
        self.builder
    }
    pub fn size(
        &mut self,
        value: &(impl Prop<IconSize> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size", val));
        value.connect_update(move |value| obj.set_property("size", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width", val));
        value.connect_update(move |value| obj.set_property("width", value));
        self.builder
    }
    pub fn mode(
        &mut self,
        value: &(impl Prop<CellRendererMode> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mode", val));
        value.connect_update(move |value| obj.set_property("mode", value));
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn active(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("active", val));
        value.connect_update(move |value| obj.set_property("active", value));
        self.builder
    }
    pub fn height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height", val));
        value.connect_update(move |value| obj.set_property("height", value));
        self.builder
    }
    pub fn is_expanded(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expanded", val));
        value.connect_update(move |value| obj.set_property("is_expanded", value));
        self.builder
    }
    pub fn is_expander(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expander", val));
        value.connect_update(move |value| obj.set_property("is_expander", value));
        self.builder
    }
    pub fn cell_background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererSpinnerBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_set", val));
        value.connect_update(move |value| obj.set_property("cell_background_set", value));
        self.builder
    }
}
pub struct CellRendererSpinnerSignals<'builder> {
    builder: &'builder mut CellRendererSpinnerBuilder,
}
impl<'builder> CellRendererSpinnerSignals<'builder> {}
impl ForteExt for CellRendererSpinner {
    type Builder = CellRendererSpinnerBuilder;
}
#[macro_export]
macro_rules ! CellRendererSpinner { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: CellRendererSpinner :: forte ()) $ ($ rest) * } } }
