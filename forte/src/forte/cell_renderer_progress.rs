#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, CellRendererProgress, *};
use gtkrs::{CellRenderer, CellRendererMode, Orientable, Orientation};
#[derive(Clone)]
pub struct CellRendererProgressBuilder {
    obj: CellRendererProgress,
}
impl Default for CellRendererProgressBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl CellRendererProgressBuilder {
    pub fn cell_background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cell_background_set", value);
        self
    }
    pub fn mode(&mut self, value: CellRendererMode) -> &mut Self {
        self.obj.set_property("mode", value);
        self
    }
    pub fn value(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("value", value);
        self
    }
    pub fn cell_background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("cell_background", value);
        self
    }
    pub fn pulse(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pulse", value);
        self
    }
    pub fn width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn text_xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("text_xalign", value);
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
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
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
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn text_yalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("text_yalign", value);
        self
    }
    pub fn ypad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("ypad", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn cell_background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("cell_background_rgba", value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("yalign", value);
        self
    }
    pub fn inverted(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("inverted", value);
        self
    }
    pub fn bind(&mut self) -> CellRendererProgressBinder {
        CellRendererProgressBinder { builder: self }
    }
    pub fn connect(&mut self) -> CellRendererProgressSignals {
        CellRendererProgressSignals { builder: self }
    }
}
impl crate::prelude::Builder for CellRendererProgressBuilder {
    type Target = CellRendererProgress;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for CellRendererProgressBuilder {
    type Target = CellRendererProgress;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct CellRendererProgressBinder<'builder> {
    builder: &'builder mut CellRendererProgressBuilder,
}
impl<'builder> CellRendererProgressBinder<'builder> {
    pub fn cell_background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_set", val));
        value.connect_update(move |value| obj.set_property("cell_background_set", value));
        self.builder
    }
    pub fn mode(
        &mut self,
        value: &(impl Prop<CellRendererMode> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mode", val));
        value.connect_update(move |value| obj.set_property("mode", value));
        self.builder
    }
    pub fn value(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("value", val));
        value.connect_update(move |value| obj.set_property("value", value));
        self.builder
    }
    pub fn cell_background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background", val));
        value.connect_update(move |value| obj.set_property("cell_background", value));
        self.builder
    }
    pub fn pulse(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pulse", val));
        value.connect_update(move |value| obj.set_property("pulse", value));
        self.builder
    }
    pub fn width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width", val));
        value.connect_update(move |value| obj.set_property("width", value));
        self.builder
    }
    pub fn xalign(
        &mut self,
        value: &(impl Prop<f32> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn text_xalign(
        &mut self,
        value: &(impl Prop<f32> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_xalign", val));
        value.connect_update(move |value| obj.set_property("text_xalign", value));
        self.builder
    }
    pub fn height(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height", val));
        value.connect_update(move |value| obj.set_property("height", value));
        self.builder
    }
    pub fn is_expanded(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expanded", val));
        value.connect_update(move |value| obj.set_property("is_expanded", value));
        self.builder
    }
    pub fn visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn xpad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xpad", val));
        value.connect_update(move |value| obj.set_property("xpad", value));
        self.builder
    }
    pub fn is_expander(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expander", val));
        value.connect_update(move |value| obj.set_property("is_expander", value));
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn text_yalign(
        &mut self,
        value: &(impl Prop<f32> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_yalign", val));
        value.connect_update(move |value| obj.set_property("text_yalign", value));
        self.builder
    }
    pub fn ypad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ypad", val));
        value.connect_update(move |value| obj.set_property("ypad", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn cell_background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_rgba", val));
        value.connect_update(move |value| obj.set_property("cell_background_rgba", value));
        self.builder
    }
    pub fn yalign(
        &mut self,
        value: &(impl Prop<f32> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("yalign", val));
        value.connect_update(move |value| obj.set_property("yalign", value));
        self.builder
    }
    pub fn inverted(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererProgressBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inverted", val));
        value.connect_update(move |value| obj.set_property("inverted", value));
        self.builder
    }
}
pub struct CellRendererProgressSignals<'builder> {
    builder: &'builder mut CellRendererProgressBuilder,
}
impl<'builder> CellRendererProgressSignals<'builder> {}
impl ForteExt for CellRendererProgress {
    type Builder = CellRendererProgressBuilder;
}
#[macro_export]
macro_rules ! CellRendererProgress { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: CellRendererProgress :: forte ()) $ ($ rest) * } } }
