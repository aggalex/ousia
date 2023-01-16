#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, CellRendererToggle, *};
use gtkrs::{CellRenderer, CellRendererMode, TreePath};
#[derive(Clone)]
pub struct CellRendererToggleBuilder {
    obj: CellRendererToggle,
}
impl Default for CellRendererToggleBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl CellRendererToggleBuilder {
    pub fn radio(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("radio", value);
        self
    }
    pub fn cell_background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cell_background_set", value);
        self
    }
    pub fn height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height", value);
        self
    }
    pub fn mode(&mut self, value: CellRendererMode) -> &mut Self {
        self.obj.set_property("mode", value);
        self
    }
    pub fn xpad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("xpad", value);
        self
    }
    pub fn is_expander(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expander", value);
        self
    }
    pub fn activatable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("activatable", value);
        self
    }
    pub fn active(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("active", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn is_expanded(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expanded", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn inconsistent(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("inconsistent", value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("yalign", value);
        self
    }
    pub fn ypad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("ypad", value);
        self
    }
    pub fn width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width", value);
        self
    }
    pub fn cell_background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("cell_background", value);
        self
    }
    pub fn cell_background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("cell_background_rgba", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn bind(&mut self) -> CellRendererToggleBinder {
        CellRendererToggleBinder { builder: self }
    }
    pub fn connect(&mut self) -> CellRendererToggleSignals {
        CellRendererToggleSignals { builder: self }
    }
}
impl crate::prelude::Builder for CellRendererToggleBuilder {
    type Target = CellRendererToggle;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for CellRendererToggleBuilder {
    type Target = CellRendererToggle;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct CellRendererToggleBinder<'builder> {
    builder: &'builder mut CellRendererToggleBuilder,
}
impl<'builder> CellRendererToggleBinder<'builder> {
    pub fn radio(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("radio", val));
        value.connect_update(move |value| obj.set_property("radio", value));
        self.builder
    }
    pub fn cell_background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_set", val));
        value.connect_update(move |value| obj.set_property("cell_background_set", value));
        self.builder
    }
    pub fn height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height", val));
        value.connect_update(move |value| obj.set_property("height", value));
        self.builder
    }
    pub fn mode(
        &mut self,
        value: &(impl Prop<CellRendererMode> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mode", val));
        value.connect_update(move |value| obj.set_property("mode", value));
        self.builder
    }
    pub fn xpad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xpad", val));
        value.connect_update(move |value| obj.set_property("xpad", value));
        self.builder
    }
    pub fn is_expander(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expander", val));
        value.connect_update(move |value| obj.set_property("is_expander", value));
        self.builder
    }
    pub fn activatable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("activatable", val));
        value.connect_update(move |value| obj.set_property("activatable", value));
        self.builder
    }
    pub fn active(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("active", val));
        value.connect_update(move |value| obj.set_property("active", value));
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn is_expanded(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expanded", val));
        value.connect_update(move |value| obj.set_property("is_expanded", value));
        self.builder
    }
    pub fn visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn inconsistent(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inconsistent", val));
        value.connect_update(move |value| obj.set_property("inconsistent", value));
        self.builder
    }
    pub fn yalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("yalign", val));
        value.connect_update(move |value| obj.set_property("yalign", value));
        self.builder
    }
    pub fn ypad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ypad", val));
        value.connect_update(move |value| obj.set_property("ypad", value));
        self.builder
    }
    pub fn width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width", val));
        value.connect_update(move |value| obj.set_property("width", value));
        self.builder
    }
    pub fn cell_background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background", val));
        value.connect_update(move |value| obj.set_property("cell_background", value));
        self.builder
    }
    pub fn cell_background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_rgba", val));
        value.connect_update(move |value| obj.set_property("cell_background_rgba", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererToggleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
}
pub struct CellRendererToggleSignals<'builder> {
    builder: &'builder mut CellRendererToggleBuilder,
}
impl<'builder> CellRendererToggleSignals<'builder> {}
impl ForteExt for CellRendererToggle {
    type Builder = CellRendererToggleBuilder;
}
#[macro_export]
macro_rules ! CellRendererToggle { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: CellRendererToggle :: forte ()) $ ($ rest) * } } }
