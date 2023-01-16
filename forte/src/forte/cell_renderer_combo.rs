#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, CellRendererCombo, *};
use gtkrs::{CellRenderer, CellRendererMode, CellRendererText, TreeIter, TreeModel, TreePath};
#[derive(Clone)]
pub struct CellRendererComboBuilder {
    obj: CellRendererCombo,
}
impl Default for CellRendererComboBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl CellRendererComboBuilder {
    pub fn attributes(&mut self, value: &pango::AttrList) -> &mut Self {
        self.obj.set_property("attributes", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<TreeModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("background", value);
        self
    }
    pub fn cell_background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("cell_background", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height", value);
        self
    }
    pub fn variant(&mut self, value: pango::Variant) -> &mut Self {
        self.obj.set_property("variant", value);
        self
    }
    pub fn align_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("align_set", value);
        self
    }
    pub fn scale(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("scale", value);
        self
    }
    pub fn size(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("size", value);
        self
    }
    pub fn style_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("style_set", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn mode(&mut self, value: CellRendererMode) -> &mut Self {
        self.obj.set_property("mode", value);
        self
    }
    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable", value);
        self
    }
    pub fn width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_chars", value);
        self
    }
    pub fn width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width", value);
        self
    }
    pub fn cell_background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("cell_background_rgba", value);
        self
    }
    pub fn ellipsize_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("ellipsize_set", value);
        self
    }
    pub fn is_expanded(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expanded", value);
        self
    }
    pub fn single_paragraph_mode(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("single_paragraph_mode", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn family(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("family", value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("yalign", value);
        self
    }
    pub fn placeholder_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("placeholder_text", value);
        self
    }
    pub fn editable_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable_set", value);
        self
    }
    pub fn variant_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("variant_set", value);
        self
    }
    pub fn size_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("size_set", value);
        self
    }
    pub fn weight_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("weight_set", value);
        self
    }
    pub fn size_points(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("size_points", value);
        self
    }
    pub fn foreground_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("foreground_rgba", value);
        self
    }
    pub fn alignment(&mut self, value: pango::Alignment) -> &mut Self {
        self.obj.set_property("alignment", value);
        self
    }
    pub fn rise_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("rise_set", value);
        self
    }
    pub fn foreground(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("foreground", value);
        self
    }
    pub fn language(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("language", value);
        self
    }
    pub fn ypad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("ypad", value);
        self
    }
    pub fn background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("background_rgba", value);
        self
    }
    pub fn style(&mut self, value: pango::Style) -> &mut Self {
        self.obj.set_property("style", value);
        self
    }
    pub fn wrap_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("wrap_width", value);
        self
    }
    pub fn has_entry(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_entry", value);
        self
    }
    pub fn background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("background_set", value);
        self
    }
    pub fn wrap_mode(&mut self, value: pango::WrapMode) -> &mut Self {
        self.obj.set_property("wrap_mode", value);
        self
    }
    pub fn underline_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("underline_set", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn font_desc(&mut self, value: &pango::FontDescription) -> &mut Self {
        self.obj.set_property("font_desc", value);
        self
    }
    pub fn text_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("text_column", value);
        self
    }
    pub fn max_width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_width_chars", value);
        self
    }
    pub fn xpad(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("xpad", value);
        self
    }
    pub fn strikethrough(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("strikethrough", value);
        self
    }
    pub fn underline(&mut self, value: pango::Underline) -> &mut Self {
        self.obj.set_property("underline", value);
        self
    }
    pub fn stretch_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("stretch_set", value);
        self
    }
    pub fn strikethrough_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("strikethrough_set", value);
        self
    }
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("font", value);
        self
    }
    pub fn cell_background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cell_background_set", value);
        self
    }
    pub fn ellipsize(&mut self, value: pango::EllipsizeMode) -> &mut Self {
        self.obj.set_property("ellipsize", value);
        self
    }
    pub fn weight(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("weight", value);
        self
    }
    pub fn is_expander(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_expander", value);
        self
    }
    pub fn foreground_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("foreground_set", value);
        self
    }
    pub fn scale_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("scale_set", value);
        self
    }
    pub fn family_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("family_set", value);
        self
    }
    pub fn language_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("language_set", value);
        self
    }
    pub fn markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("markup", value);
        self
    }
    pub fn rise(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("rise", value);
        self
    }
    pub fn stretch(&mut self, value: pango::Stretch) -> &mut Self {
        self.obj.set_property("stretch", value);
        self
    }
    pub fn bind(&mut self) -> CellRendererComboBinder {
        CellRendererComboBinder { builder: self }
    }
    pub fn connect(&mut self) -> CellRendererComboSignals {
        CellRendererComboSignals { builder: self }
    }
}
impl crate::prelude::Builder for CellRendererComboBuilder {
    type Target = CellRendererCombo;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for CellRendererComboBuilder {
    type Target = CellRendererCombo;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct CellRendererComboBinder<'builder> {
    builder: &'builder mut CellRendererComboBuilder,
}
impl<'builder> CellRendererComboBinder<'builder> {
    pub fn attributes(
        &mut self,
        value: &(impl Prop<pango::AttrList> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("attributes", val));
        value.connect_update(move |value| obj.set_property("attributes", value));
        self.builder
    }
    pub fn model<T: IsA<TreeModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background", val));
        value.connect_update(move |value| obj.set_property("background", value));
        self.builder
    }
    pub fn cell_background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background", val));
        value.connect_update(move |value| obj.set_property("cell_background", value));
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn height(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height", val));
        value.connect_update(move |value| obj.set_property("height", value));
        self.builder
    }
    pub fn variant(
        &mut self,
        value: &(impl Prop<pango::Variant> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("variant", val));
        value.connect_update(move |value| obj.set_property("variant", value));
        self.builder
    }
    pub fn align_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("align_set", val));
        value.connect_update(move |value| obj.set_property("align_set", value));
        self.builder
    }
    pub fn scale(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("scale", val));
        value.connect_update(move |value| obj.set_property("scale", value));
        self.builder
    }
    pub fn size(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size", val));
        value.connect_update(move |value| obj.set_property("size", value));
        self.builder
    }
    pub fn style_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("style_set", val));
        value.connect_update(move |value| obj.set_property("style_set", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn mode(
        &mut self,
        value: &(impl Prop<CellRendererMode> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mode", val));
        value.connect_update(move |value| obj.set_property("mode", value));
        self.builder
    }
    pub fn editable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable", val));
        value.connect_update(move |value| obj.set_property("editable", value));
        self.builder
    }
    pub fn width_chars(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_chars", val));
        value.connect_update(move |value| obj.set_property("width_chars", value));
        self.builder
    }
    pub fn width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width", val));
        value.connect_update(move |value| obj.set_property("width", value));
        self.builder
    }
    pub fn cell_background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_rgba", val));
        value.connect_update(move |value| obj.set_property("cell_background_rgba", value));
        self.builder
    }
    pub fn ellipsize_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ellipsize_set", val));
        value.connect_update(move |value| obj.set_property("ellipsize_set", value));
        self.builder
    }
    pub fn is_expanded(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expanded", val));
        value.connect_update(move |value| obj.set_property("is_expanded", value));
        self.builder
    }
    pub fn single_paragraph_mode(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("single_paragraph_mode", val));
        value.connect_update(move |value| obj.set_property("single_paragraph_mode", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn family(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("family", val));
        value.connect_update(move |value| obj.set_property("family", value));
        self.builder
    }
    pub fn yalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("yalign", val));
        value.connect_update(move |value| obj.set_property("yalign", value));
        self.builder
    }
    pub fn placeholder_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("placeholder_text", val));
        value.connect_update(move |value| obj.set_property("placeholder_text", value));
        self.builder
    }
    pub fn editable_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable_set", val));
        value.connect_update(move |value| obj.set_property("editable_set", value));
        self.builder
    }
    pub fn variant_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("variant_set", val));
        value.connect_update(move |value| obj.set_property("variant_set", value));
        self.builder
    }
    pub fn size_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size_set", val));
        value.connect_update(move |value| obj.set_property("size_set", value));
        self.builder
    }
    pub fn weight_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("weight_set", val));
        value.connect_update(move |value| obj.set_property("weight_set", value));
        self.builder
    }
    pub fn size_points(
        &mut self,
        value: &(impl Prop<f64> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size_points", val));
        value.connect_update(move |value| obj.set_property("size_points", value));
        self.builder
    }
    pub fn foreground_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground_rgba", val));
        value.connect_update(move |value| obj.set_property("foreground_rgba", value));
        self.builder
    }
    pub fn alignment(
        &mut self,
        value: &(impl Prop<pango::Alignment> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("alignment", val));
        value.connect_update(move |value| obj.set_property("alignment", value));
        self.builder
    }
    pub fn rise_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rise_set", val));
        value.connect_update(move |value| obj.set_property("rise_set", value));
        self.builder
    }
    pub fn foreground(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground", val));
        value.connect_update(move |value| obj.set_property("foreground", value));
        self.builder
    }
    pub fn language(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("language", val));
        value.connect_update(move |value| obj.set_property("language", value));
        self.builder
    }
    pub fn ypad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ypad", val));
        value.connect_update(move |value| obj.set_property("ypad", value));
        self.builder
    }
    pub fn background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_rgba", val));
        value.connect_update(move |value| obj.set_property("background_rgba", value));
        self.builder
    }
    pub fn style(
        &mut self,
        value: &(impl Prop<pango::Style> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("style", val));
        value.connect_update(move |value| obj.set_property("style", value));
        self.builder
    }
    pub fn wrap_width(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_width", val));
        value.connect_update(move |value| obj.set_property("wrap_width", value));
        self.builder
    }
    pub fn has_entry(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_entry", val));
        value.connect_update(move |value| obj.set_property("has_entry", value));
        self.builder
    }
    pub fn background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_set", val));
        value.connect_update(move |value| obj.set_property("background_set", value));
        self.builder
    }
    pub fn wrap_mode(
        &mut self,
        value: &(impl Prop<pango::WrapMode> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_mode", val));
        value.connect_update(move |value| obj.set_property("wrap_mode", value));
        self.builder
    }
    pub fn underline_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline_set", val));
        value.connect_update(move |value| obj.set_property("underline_set", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn font_desc(
        &mut self,
        value: &(impl Prop<pango::FontDescription> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font_desc", val));
        value.connect_update(move |value| obj.set_property("font_desc", value));
        self.builder
    }
    pub fn text_column(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_column", val));
        value.connect_update(move |value| obj.set_property("text_column", value));
        self.builder
    }
    pub fn max_width_chars(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width_chars", val));
        value.connect_update(move |value| obj.set_property("max_width_chars", value));
        self.builder
    }
    pub fn xpad(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xpad", val));
        value.connect_update(move |value| obj.set_property("xpad", value));
        self.builder
    }
    pub fn strikethrough(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough", val));
        value.connect_update(move |value| obj.set_property("strikethrough", value));
        self.builder
    }
    pub fn underline(
        &mut self,
        value: &(impl Prop<pango::Underline> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline", val));
        value.connect_update(move |value| obj.set_property("underline", value));
        self.builder
    }
    pub fn stretch_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("stretch_set", val));
        value.connect_update(move |value| obj.set_property("stretch_set", value));
        self.builder
    }
    pub fn strikethrough_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough_set", val));
        value.connect_update(move |value| obj.set_property("strikethrough_set", value));
        self.builder
    }
    pub fn font(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font", val));
        value.connect_update(move |value| obj.set_property("font", value));
        self.builder
    }
    pub fn cell_background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_background_set", val));
        value.connect_update(move |value| obj.set_property("cell_background_set", value));
        self.builder
    }
    pub fn ellipsize(
        &mut self,
        value: &(impl Prop<pango::EllipsizeMode> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ellipsize", val));
        value.connect_update(move |value| obj.set_property("ellipsize", value));
        self.builder
    }
    pub fn weight(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("weight", val));
        value.connect_update(move |value| obj.set_property("weight", value));
        self.builder
    }
    pub fn is_expander(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_expander", val));
        value.connect_update(move |value| obj.set_property("is_expander", value));
        self.builder
    }
    pub fn foreground_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground_set", val));
        value.connect_update(move |value| obj.set_property("foreground_set", value));
        self.builder
    }
    pub fn scale_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("scale_set", val));
        value.connect_update(move |value| obj.set_property("scale_set", value));
        self.builder
    }
    pub fn family_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("family_set", val));
        value.connect_update(move |value| obj.set_property("family_set", value));
        self.builder
    }
    pub fn language_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("language_set", val));
        value.connect_update(move |value| obj.set_property("language_set", value));
        self.builder
    }
    pub fn markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("markup", val));
        value.connect_update(move |value| obj.set_property("markup", value));
        self.builder
    }
    pub fn rise(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rise", val));
        value.connect_update(move |value| obj.set_property("rise", value));
        self.builder
    }
    pub fn stretch(
        &mut self,
        value: &(impl Prop<pango::Stretch> + ?Sized),
    ) -> &mut CellRendererComboBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("stretch", val));
        value.connect_update(move |value| obj.set_property("stretch", value));
        self.builder
    }
}
pub struct CellRendererComboSignals<'builder> {
    builder: &'builder mut CellRendererComboBuilder,
}
impl<'builder> CellRendererComboSignals<'builder> {}
impl ForteExt for CellRendererCombo {
    type Builder = CellRendererComboBuilder;
}
#[macro_export]
macro_rules ! CellRendererCombo { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: CellRendererCombo :: forte ()) $ ($ rest) * } } }
