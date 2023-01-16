#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, TextTag, *};
use gtkrs::{Justification, TextDirection, WrapMode};
#[derive(Clone)]
pub struct TextTagBuilder {
    obj: TextTag,
}
impl Default for TextTagBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TextTagBuilder {
    pub fn size(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("size", value);
        self
    }
    pub fn show_spaces_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_spaces_set", value);
        self
    }
    pub fn variant(&mut self, value: pango::Variant) -> &mut Self {
        self.obj.set_property("variant", value);
        self
    }
    pub fn underline_rgba_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("underline_rgba_set", value);
        self
    }
    pub fn fallback(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("fallback", value);
        self
    }
    pub fn background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("background_rgba", value);
        self
    }
    pub fn strikethrough_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("strikethrough_set", value);
        self
    }
    pub fn insert_hyphens(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("insert_hyphens", value);
        self
    }
    pub fn overline_rgba_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("overline_rgba_set", value);
        self
    }
    pub fn scale(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("scale", value);
        self
    }
    pub fn invisible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("invisible", value);
        self
    }
    pub fn foreground_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("foreground_rgba", value);
        self
    }
    pub fn overline_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("overline_rgba", value);
        self
    }
    pub fn paragraph_background_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("paragraph_background_rgba", value);
        self
    }
    pub fn fallback_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("fallback_set", value);
        self
    }
    pub fn invisible_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("invisible_set", value);
        self
    }
    pub fn pixels_inside_wrap(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_inside_wrap", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn sentence_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sentence_set", value);
        self
    }
    pub fn justification_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("justification_set", value);
        self
    }
    pub fn strikethrough_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("strikethrough_rgba", value);
        self
    }
    pub fn family(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("family", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn word(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("word", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn word_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("word_set", value);
        self
    }
    pub fn justification(&mut self, value: Justification) -> &mut Self {
        self.obj.set_property("justification", value);
        self
    }
    pub fn letter_spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("letter_spacing", value);
        self
    }
    pub fn right_margin_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("right_margin_set", value);
        self
    }
    pub fn variant_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("variant_set", value);
        self
    }
    pub fn wrap_mode(&mut self, value: WrapMode) -> &mut Self {
        self.obj.set_property("wrap_mode", value);
        self
    }
    pub fn wrap_mode_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("wrap_mode_set", value);
        self
    }
    pub fn direction(&mut self, value: TextDirection) -> &mut Self {
        self.obj.set_property("direction", value);
        self
    }
    pub fn font_desc(&mut self, value: &pango::FontDescription) -> &mut Self {
        self.obj.set_property("font_desc", value);
        self
    }
    pub fn size_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("size_set", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn foreground_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("foreground_set", value);
        self
    }
    pub fn stretch_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("stretch_set", value);
        self
    }
    pub fn weight(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("weight", value);
        self
    }
    pub fn right_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("right_margin", value);
        self
    }
    pub fn paragraph_background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("paragraph_background_set", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn line_height(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("line_height", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn text_transform(&mut self, value: pango::TextTransform) -> &mut Self {
        self.obj.set_property("text_transform", value);
        self
    }
    pub fn background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("background", value);
        self
    }
    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable", value);
        self
    }
    pub fn show_spaces(&mut self, value: pango::ShowFlags) -> &mut Self {
        self.obj.set_property("show_spaces", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn line_height_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("line_height_set", value);
        self
    }
    pub fn left_margin_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("left_margin_set", value);
        self
    }
    pub fn pixels_below_lines_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("pixels_below_lines_set", value);
        self
    }
    pub fn pixels_below_lines(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_below_lines", value);
        self
    }
    pub fn pixels_inside_wrap_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("pixels_inside_wrap_set", value);
        self
    }
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.obj.set_property("tabs", value);
        self
    }
    pub fn pixels_above_lines_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("pixels_above_lines_set", value);
        self
    }
    pub fn language_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("language_set", value);
        self
    }
    pub fn stretch(&mut self, value: pango::Stretch) -> &mut Self {
        self.obj.set_property("stretch", value);
        self
    }
    pub fn strikethrough(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("strikethrough", value);
        self
    }
    pub fn pixels_above_lines(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_above_lines", value);
        self
    }
    pub fn paragraph_background(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("paragraph_background", value);
        self
    }
    pub fn size_points(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("size_points", value);
        self
    }
    pub fn editable_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable_set", value);
        self
    }
    pub fn left_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("left_margin", value);
        self
    }
    pub fn letter_spacing_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("letter_spacing_set", value);
        self
    }
    pub fn rise_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("rise_set", value);
        self
    }
    pub fn accumulative_margin(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("accumulative_margin", value);
        self
    }
    pub fn underline_rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("underline_rgba", value);
        self
    }
    pub fn indent_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("indent_set", value);
        self
    }
    pub fn underline(&mut self, value: pango::Underline) -> &mut Self {
        self.obj.set_property("underline", value);
        self
    }
    pub fn weight_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("weight_set", value);
        self
    }
    pub fn background_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("background_set", value);
        self
    }
    pub fn tabs_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("tabs_set", value);
        self
    }
    pub fn insert_hyphens_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("insert_hyphens_set", value);
        self
    }
    pub fn overline_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("overline_set", value);
        self
    }
    pub fn language(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("language", value);
        self
    }
    pub fn strikethrough_rgba_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("strikethrough_rgba_set", value);
        self
    }
    pub fn style_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("style_set", value);
        self
    }
    pub fn allow_breaks_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("allow_breaks_set", value);
        self
    }
    pub fn background_full_height(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("background_full_height", value);
        self
    }
    pub fn font_features_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("font_features_set", value);
        self
    }
    pub fn overline(&mut self, value: pango::Overline) -> &mut Self {
        self.obj.set_property("overline", value);
        self
    }
    pub fn foreground(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("foreground", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn sentence(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sentence", value);
        self
    }
    pub fn rise(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("rise", value);
        self
    }
    pub fn family_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("family_set", value);
        self
    }
    pub fn style(&mut self, value: pango::Style) -> &mut Self {
        self.obj.set_property("style", value);
        self
    }
    pub fn underline_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("underline_set", value);
        self
    }
    pub fn background_full_height_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("background_full_height_set", value);
        self
    }
    pub fn font_features(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("font_features", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn text_transform_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("text_transform_set", value);
        self
    }
    pub fn allow_breaks(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("allow_breaks", value);
        self
    }
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("font", value);
        self
    }
    pub fn indent(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("indent", value);
        self
    }
    pub fn scale_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("scale_set", value);
        self
    }
    pub fn bind(&mut self) -> TextTagBinder {
        TextTagBinder { builder: self }
    }
    pub fn connect(&mut self) -> TextTagSignals {
        TextTagSignals { builder: self }
    }
}
impl crate::prelude::Builder for TextTagBuilder {
    type Target = TextTag;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TextTagBuilder {
    type Target = TextTag;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TextTagBinder<'builder> {
    builder: &'builder mut TextTagBuilder,
}
impl<'builder> TextTagBinder<'builder> {
    pub fn size(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size", val));
        value.connect_update(move |value| obj.set_property("size", value));
        self.builder
    }
    pub fn show_spaces_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_spaces_set", val));
        value.connect_update(move |value| obj.set_property("show_spaces_set", value));
        self.builder
    }
    pub fn variant(&mut self, value: &(impl Prop<pango::Variant> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("variant", val));
        value.connect_update(move |value| obj.set_property("variant", value));
        self.builder
    }
    pub fn underline_rgba_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline_rgba_set", val));
        value.connect_update(move |value| obj.set_property("underline_rgba_set", value));
        self.builder
    }
    pub fn fallback(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fallback", val));
        value.connect_update(move |value| obj.set_property("fallback", value));
        self.builder
    }
    pub fn background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_rgba", val));
        value.connect_update(move |value| obj.set_property("background_rgba", value));
        self.builder
    }
    pub fn strikethrough_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough_set", val));
        value.connect_update(move |value| obj.set_property("strikethrough_set", value));
        self.builder
    }
    pub fn insert_hyphens(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("insert_hyphens", val));
        value.connect_update(move |value| obj.set_property("insert_hyphens", value));
        self.builder
    }
    pub fn overline_rgba_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overline_rgba_set", val));
        value.connect_update(move |value| obj.set_property("overline_rgba_set", value));
        self.builder
    }
    pub fn scale(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("scale", val));
        value.connect_update(move |value| obj.set_property("scale", value));
        self.builder
    }
    pub fn invisible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible", val));
        value.connect_update(move |value| obj.set_property("invisible", value));
        self.builder
    }
    pub fn foreground_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground_rgba", val));
        value.connect_update(move |value| obj.set_property("foreground_rgba", value));
        self.builder
    }
    pub fn overline_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overline_rgba", val));
        value.connect_update(move |value| obj.set_property("overline_rgba", value));
        self.builder
    }
    pub fn paragraph_background_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("paragraph_background_rgba", val));
        value.connect_update(move |value| obj.set_property("paragraph_background_rgba", value));
        self.builder
    }
    pub fn fallback_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fallback_set", val));
        value.connect_update(move |value| obj.set_property("fallback_set", value));
        self.builder
    }
    pub fn invisible_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible_set", val));
        value.connect_update(move |value| obj.set_property("invisible_set", value));
        self.builder
    }
    pub fn pixels_inside_wrap(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_inside_wrap", val));
        value.connect_update(move |value| obj.set_property("pixels_inside_wrap", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn sentence_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sentence_set", val));
        value.connect_update(move |value| obj.set_property("sentence_set", value));
        self.builder
    }
    pub fn justification_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("justification_set", val));
        value.connect_update(move |value| obj.set_property("justification_set", value));
        self.builder
    }
    pub fn strikethrough_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough_rgba", val));
        value.connect_update(move |value| obj.set_property("strikethrough_rgba", value));
        self.builder
    }
    pub fn family(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("family", val));
        value.connect_update(move |value| obj.set_property("family", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn word(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("word", val));
        value.connect_update(move |value| obj.set_property("word", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn word_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("word_set", val));
        value.connect_update(move |value| obj.set_property("word_set", value));
        self.builder
    }
    pub fn justification(
        &mut self,
        value: &(impl Prop<Justification> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("justification", val));
        value.connect_update(move |value| obj.set_property("justification", value));
        self.builder
    }
    pub fn letter_spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("letter_spacing", val));
        value.connect_update(move |value| obj.set_property("letter_spacing", value));
        self.builder
    }
    pub fn right_margin_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("right_margin_set", val));
        value.connect_update(move |value| obj.set_property("right_margin_set", value));
        self.builder
    }
    pub fn variant_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("variant_set", val));
        value.connect_update(move |value| obj.set_property("variant_set", value));
        self.builder
    }
    pub fn wrap_mode(&mut self, value: &(impl Prop<WrapMode> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_mode", val));
        value.connect_update(move |value| obj.set_property("wrap_mode", value));
        self.builder
    }
    pub fn wrap_mode_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_mode_set", val));
        value.connect_update(move |value| obj.set_property("wrap_mode_set", value));
        self.builder
    }
    pub fn direction(
        &mut self,
        value: &(impl Prop<TextDirection> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("direction", val));
        value.connect_update(move |value| obj.set_property("direction", value));
        self.builder
    }
    pub fn font_desc(
        &mut self,
        value: &(impl Prop<pango::FontDescription> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font_desc", val));
        value.connect_update(move |value| obj.set_property("font_desc", value));
        self.builder
    }
    pub fn size_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size_set", val));
        value.connect_update(move |value| obj.set_property("size_set", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn foreground_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground_set", val));
        value.connect_update(move |value| obj.set_property("foreground_set", value));
        self.builder
    }
    pub fn stretch_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("stretch_set", val));
        value.connect_update(move |value| obj.set_property("stretch_set", value));
        self.builder
    }
    pub fn weight(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("weight", val));
        value.connect_update(move |value| obj.set_property("weight", value));
        self.builder
    }
    pub fn right_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("right_margin", val));
        value.connect_update(move |value| obj.set_property("right_margin", value));
        self.builder
    }
    pub fn paragraph_background_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("paragraph_background_set", val));
        value.connect_update(move |value| obj.set_property("paragraph_background_set", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn line_height(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("line_height", val));
        value.connect_update(move |value| obj.set_property("line_height", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn text_transform(
        &mut self,
        value: &(impl Prop<pango::TextTransform> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_transform", val));
        value.connect_update(move |value| obj.set_property("text_transform", value));
        self.builder
    }
    pub fn background(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background", val));
        value.connect_update(move |value| obj.set_property("background", value));
        self.builder
    }
    pub fn editable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable", val));
        value.connect_update(move |value| obj.set_property("editable", value));
        self.builder
    }
    pub fn show_spaces(
        &mut self,
        value: &(impl Prop<pango::ShowFlags> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_spaces", val));
        value.connect_update(move |value| obj.set_property("show_spaces", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn line_height_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("line_height_set", val));
        value.connect_update(move |value| obj.set_property("line_height_set", value));
        self.builder
    }
    pub fn left_margin_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("left_margin_set", val));
        value.connect_update(move |value| obj.set_property("left_margin_set", value));
        self.builder
    }
    pub fn pixels_below_lines_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_below_lines_set", val));
        value.connect_update(move |value| obj.set_property("pixels_below_lines_set", value));
        self.builder
    }
    pub fn pixels_below_lines(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_below_lines", val));
        value.connect_update(move |value| obj.set_property("pixels_below_lines", value));
        self.builder
    }
    pub fn pixels_inside_wrap_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_inside_wrap_set", val));
        value.connect_update(move |value| obj.set_property("pixels_inside_wrap_set", value));
        self.builder
    }
    pub fn tabs(&mut self, value: &(impl Prop<pango::TabArray> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs", val));
        value.connect_update(move |value| obj.set_property("tabs", value));
        self.builder
    }
    pub fn pixels_above_lines_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_above_lines_set", val));
        value.connect_update(move |value| obj.set_property("pixels_above_lines_set", value));
        self.builder
    }
    pub fn language_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("language_set", val));
        value.connect_update(move |value| obj.set_property("language_set", value));
        self.builder
    }
    pub fn stretch(&mut self, value: &(impl Prop<pango::Stretch> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("stretch", val));
        value.connect_update(move |value| obj.set_property("stretch", value));
        self.builder
    }
    pub fn strikethrough(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough", val));
        value.connect_update(move |value| obj.set_property("strikethrough", value));
        self.builder
    }
    pub fn pixels_above_lines(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_above_lines", val));
        value.connect_update(move |value| obj.set_property("pixels_above_lines", value));
        self.builder
    }
    pub fn paragraph_background(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("paragraph_background", val));
        value.connect_update(move |value| obj.set_property("paragraph_background", value));
        self.builder
    }
    pub fn size_points(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("size_points", val));
        value.connect_update(move |value| obj.set_property("size_points", value));
        self.builder
    }
    pub fn editable_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable_set", val));
        value.connect_update(move |value| obj.set_property("editable_set", value));
        self.builder
    }
    pub fn left_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("left_margin", val));
        value.connect_update(move |value| obj.set_property("left_margin", value));
        self.builder
    }
    pub fn letter_spacing_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("letter_spacing_set", val));
        value.connect_update(move |value| obj.set_property("letter_spacing_set", value));
        self.builder
    }
    pub fn rise_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rise_set", val));
        value.connect_update(move |value| obj.set_property("rise_set", value));
        self.builder
    }
    pub fn accumulative_margin(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accumulative_margin", val));
        value.connect_update(move |value| obj.set_property("accumulative_margin", value));
        self.builder
    }
    pub fn underline_rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline_rgba", val));
        value.connect_update(move |value| obj.set_property("underline_rgba", value));
        self.builder
    }
    pub fn indent_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("indent_set", val));
        value.connect_update(move |value| obj.set_property("indent_set", value));
        self.builder
    }
    pub fn underline(
        &mut self,
        value: &(impl Prop<pango::Underline> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline", val));
        value.connect_update(move |value| obj.set_property("underline", value));
        self.builder
    }
    pub fn weight_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("weight_set", val));
        value.connect_update(move |value| obj.set_property("weight_set", value));
        self.builder
    }
    pub fn background_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_set", val));
        value.connect_update(move |value| obj.set_property("background_set", value));
        self.builder
    }
    pub fn tabs_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs_set", val));
        value.connect_update(move |value| obj.set_property("tabs_set", value));
        self.builder
    }
    pub fn insert_hyphens_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("insert_hyphens_set", val));
        value.connect_update(move |value| obj.set_property("insert_hyphens_set", value));
        self.builder
    }
    pub fn overline_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overline_set", val));
        value.connect_update(move |value| obj.set_property("overline_set", value));
        self.builder
    }
    pub fn language(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("language", val));
        value.connect_update(move |value| obj.set_property("language", value));
        self.builder
    }
    pub fn strikethrough_rgba_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("strikethrough_rgba_set", val));
        value.connect_update(move |value| obj.set_property("strikethrough_rgba_set", value));
        self.builder
    }
    pub fn style_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("style_set", val));
        value.connect_update(move |value| obj.set_property("style_set", value));
        self.builder
    }
    pub fn allow_breaks_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("allow_breaks_set", val));
        value.connect_update(move |value| obj.set_property("allow_breaks_set", value));
        self.builder
    }
    pub fn background_full_height(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_full_height", val));
        value.connect_update(move |value| obj.set_property("background_full_height", value));
        self.builder
    }
    pub fn font_features_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font_features_set", val));
        value.connect_update(move |value| obj.set_property("font_features_set", value));
        self.builder
    }
    pub fn overline(
        &mut self,
        value: &(impl Prop<pango::Overline> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overline", val));
        value.connect_update(move |value| obj.set_property("overline", value));
        self.builder
    }
    pub fn foreground(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("foreground", val));
        value.connect_update(move |value| obj.set_property("foreground", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn sentence(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sentence", val));
        value.connect_update(move |value| obj.set_property("sentence", value));
        self.builder
    }
    pub fn rise(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rise", val));
        value.connect_update(move |value| obj.set_property("rise", value));
        self.builder
    }
    pub fn family_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("family_set", val));
        value.connect_update(move |value| obj.set_property("family_set", value));
        self.builder
    }
    pub fn style(&mut self, value: &(impl Prop<pango::Style> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("style", val));
        value.connect_update(move |value| obj.set_property("style", value));
        self.builder
    }
    pub fn underline_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("underline_set", val));
        value.connect_update(move |value| obj.set_property("underline_set", value));
        self.builder
    }
    pub fn background_full_height_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("background_full_height_set", val));
        value.connect_update(move |value| obj.set_property("background_full_height_set", value));
        self.builder
    }
    pub fn font_features(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font_features", val));
        value.connect_update(move |value| obj.set_property("font_features", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn text_transform_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_transform_set", val));
        value.connect_update(move |value| obj.set_property("text_transform_set", value));
        self.builder
    }
    pub fn allow_breaks(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("allow_breaks", val));
        value.connect_update(move |value| obj.set_property("allow_breaks", value));
        self.builder
    }
    pub fn font(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font", val));
        value.connect_update(move |value| obj.set_property("font", value));
        self.builder
    }
    pub fn indent(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("indent", val));
        value.connect_update(move |value| obj.set_property("indent", value));
        self.builder
    }
    pub fn scale_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("scale_set", val));
        value.connect_update(move |value| obj.set_property("scale_set", value));
        self.builder
    }
}
pub struct TextTagSignals<'builder> {
    builder: &'builder mut TextTagBuilder,
}
impl<'builder> TextTagSignals<'builder> {
    pub fn stretch_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_stretch_set_notify(f);
        &mut self.builder
    }
    pub fn stretch_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_stretch_notify(f);
        &mut self.builder
    }
    pub fn pixels_above_lines_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_above_lines_set_notify(f);
        &mut self.builder
    }
    pub fn background_full_height_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_background_full_height_notify(f);
        &mut self.builder
    }
    pub fn fallback_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_fallback_set_notify(f);
        &mut self.builder
    }
    pub fn indent_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_indent_set_notify(f);
        &mut self.builder
    }
    pub fn overline_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overline_notify(f);
        &mut self.builder
    }
    pub fn justification_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_justification_set_notify(f);
        &mut self.builder
    }
    pub fn sentence_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_sentence_set_notify(f);
        &mut self.builder
    }
    pub fn tabs_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_tabs_notify(f);
        &mut self.builder
    }
    pub fn direction_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_direction_notify(f);
        &mut self.builder
    }
    pub fn paragraph_background_rgba_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paragraph_background_rgba_notify(f);
        &mut self.builder
    }
    pub fn word_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_word_set_notify(f);
        &mut self.builder
    }
    pub fn word_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_word_notify(f);
        &mut self.builder
    }
    pub fn text_transform_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_text_transform_notify(f);
        &mut self.builder
    }
    pub fn left_margin_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_left_margin_set_notify(f);
        &mut self.builder
    }
    pub fn strikethrough_rgba_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_strikethrough_rgba_set_notify(f);
        &mut self.builder
    }
    pub fn underline_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_underline_set_notify(f);
        &mut self.builder
    }
    pub fn letter_spacing_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_letter_spacing_notify(f);
        &mut self.builder
    }
    pub fn insert_hyphens_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_hyphens_notify(f);
        &mut self.builder
    }
    pub fn line_height_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_line_height_set_notify(f);
        &mut self.builder
    }
    pub fn invisible_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_invisible_notify(f);
        &mut self.builder
    }
    pub fn language_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_language_notify(f);
        &mut self.builder
    }
    pub fn tabs_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_tabs_set_notify(f);
        &mut self.builder
    }
    pub fn foreground_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_foreground_notify(f);
        &mut self.builder
    }
    pub fn font_desc_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_font_desc_notify(f);
        &mut self.builder
    }
    pub fn foreground_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_foreground_set_notify(f);
        &mut self.builder
    }
    pub fn indent_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_indent_notify(f);
        &mut self.builder
    }
    pub fn language_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_language_set_notify(f);
        &mut self.builder
    }
    pub fn paragraph_background_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paragraph_background_set_notify(f);
        &mut self.builder
    }
    pub fn pixels_inside_wrap_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_inside_wrap_set_notify(f);
        &mut self.builder
    }
    pub fn rise_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_rise_notify(f);
        &mut self.builder
    }
    pub fn editable_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_editable_set_notify(f);
        &mut self.builder
    }
    pub fn font_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_font_notify(f);
        &mut self.builder
    }
    pub fn show_spaces_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_show_spaces_set_notify(f);
        &mut self.builder
    }
    pub fn background_rgba_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_background_rgba_notify(f);
        &mut self.builder
    }
    pub fn foreground_rgba_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_foreground_rgba_notify(f);
        &mut self.builder
    }
    pub fn show_spaces_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_show_spaces_notify(f);
        &mut self.builder
    }
    pub fn invisible_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_invisible_set_notify(f);
        &mut self.builder
    }
    pub fn weight_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_weight_set_notify(f);
        &mut self.builder
    }
    pub fn variant_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_variant_set_notify(f);
        &mut self.builder
    }
    pub fn font_features_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_font_features_notify(f);
        &mut self.builder
    }
    pub fn rise_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_rise_set_notify(f);
        &mut self.builder
    }
    pub fn underline_rgba_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_underline_rgba_notify(f);
        &mut self.builder
    }
    pub fn pixels_below_lines_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_below_lines_notify(f);
        &mut self.builder
    }
    pub fn line_height_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_line_height_notify(f);
        &mut self.builder
    }
    pub fn family_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_family_set_notify(f);
        &mut self.builder
    }
    pub fn underline_rgba_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_underline_rgba_set_notify(f);
        &mut self.builder
    }
    pub fn wrap_mode_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_wrap_mode_notify(f);
        &mut self.builder
    }
    pub fn editable_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_editable_notify(f);
        &mut self.builder
    }
    pub fn pixels_above_lines_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_above_lines_notify(f);
        &mut self.builder
    }
    pub fn overline_rgba_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overline_rgba_set_notify(f);
        &mut self.builder
    }
    pub fn family_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_family_notify(f);
        &mut self.builder
    }
    pub fn scale_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_scale_notify(f);
        &mut self.builder
    }
    pub fn sentence_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_sentence_notify(f);
        &mut self.builder
    }
    pub fn wrap_mode_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_wrap_mode_set_notify(f);
        &mut self.builder
    }
    pub fn justification_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_justification_notify(f);
        &mut self.builder
    }
    pub fn accumulative_margin_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_accumulative_margin_notify(f);
        &mut self.builder
    }
    pub fn background_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_background_notify(f);
        &mut self.builder
    }
    pub fn pixels_below_lines_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_below_lines_set_notify(f);
        &mut self.builder
    }
    pub fn paragraph_background_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paragraph_background_notify(f);
        &mut self.builder
    }
    pub fn pixels_inside_wrap_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_inside_wrap_notify(f);
        &mut self.builder
    }
    pub fn right_margin_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_right_margin_set_notify(f);
        &mut self.builder
    }
    pub fn strikethrough_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_strikethrough_notify(f);
        &mut self.builder
    }
    pub fn weight_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_weight_notify(f);
        &mut self.builder
    }
    pub fn left_margin_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_left_margin_notify(f);
        &mut self.builder
    }
    pub fn background_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_background_set_notify(f);
        &mut self.builder
    }
    pub fn underline_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_underline_notify(f);
        &mut self.builder
    }
    pub fn font_features_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_font_features_set_notify(f);
        &mut self.builder
    }
    pub fn scale_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_scale_set_notify(f);
        &mut self.builder
    }
    pub fn letter_spacing_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_letter_spacing_set_notify(f);
        &mut self.builder
    }
    pub fn right_margin_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_right_margin_notify(f);
        &mut self.builder
    }
    pub fn variant_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_variant_notify(f);
        &mut self.builder
    }
    pub fn size_points_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_size_points_notify(f);
        &mut self.builder
    }
    pub fn size_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_size_set_notify(f);
        &mut self.builder
    }
    pub fn background_full_height_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_background_full_height_set_notify(f);
        &mut self.builder
    }
    pub fn style_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_style_notify(f);
        &mut self.builder
    }
    pub fn text_transform_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_text_transform_set_notify(f);
        &mut self.builder
    }
    pub fn allow_breaks_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_allow_breaks_notify(f);
        &mut self.builder
    }
    pub fn insert_hyphens_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_hyphens_set_notify(f);
        &mut self.builder
    }
    pub fn strikethrough_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_strikethrough_set_notify(f);
        &mut self.builder
    }
    pub fn size_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_size_notify(f);
        &mut self.builder
    }
    pub fn style_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_style_set_notify(f);
        &mut self.builder
    }
    pub fn allow_breaks_set_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_allow_breaks_set_notify(f);
        &mut self.builder
    }
    pub fn strikethrough_rgba_notify(
        &mut self,
        f: impl Fn(&TextTag) + 'static,
    ) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_strikethrough_rgba_notify(f);
        &mut self.builder
    }
    pub fn overline_rgba_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overline_rgba_notify(f);
        &mut self.builder
    }
    pub fn fallback_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_fallback_notify(f);
        &mut self.builder
    }
    pub fn overline_set_notify(&mut self, f: impl Fn(&TextTag) + 'static) -> &mut TextTagBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overline_set_notify(f);
        &mut self.builder
    }
}
impl ForteExt for TextTag {
    type Builder = TextTagBuilder;
}
#[macro_export]
macro_rules ! TextTag { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TextTag :: forte ()) $ ($ rest) * } } }
