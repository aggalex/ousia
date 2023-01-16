#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Text, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Editable, EntryBuffer,
    InputHints, InputPurpose, LayoutManager, Overflow, Widget,
};
#[derive(Clone)]
pub struct TextBuilder {
    obj: Text,
}
impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TextBuilder {
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn attributes(&mut self, value: &pango::AttrList) -> &mut Self {
        self.obj.set_property("attributes", value);
        self
    }
    pub fn invisible_char(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("invisible_char", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn visibility(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visibility", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn extra_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.obj.set_property("extra_menu", value);
        self
    }
    pub fn placeholder_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("placeholder_text", value);
        self
    }
    pub fn input_hints(&mut self, value: InputHints) -> &mut Self {
        self.obj.set_property("input_hints", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.obj.set_property("tabs", value);
        self
    }
    pub fn invisible_char_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("invisible_char_set", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn max_width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_width_chars", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn im_module(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("im_module", value);
        self
    }
    pub fn max_length(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_length", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn enable_emoji_completion(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_emoji_completion", value);
        self
    }
    pub fn overwrite_mode(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("overwrite_mode", value);
        self
    }
    pub fn input_purpose(&mut self, value: InputPurpose) -> &mut Self {
        self.obj.set_property("input_purpose", value);
        self
    }
    pub fn activates_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("activates_default", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn buffer(&mut self, value: &impl IsA<EntryBuffer>) -> &mut Self {
        self.obj.set_property("buffer", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn enable_undo(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_undo", value);
        self
    }
    pub fn propagate_text_width(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("propagate_text_width", value);
        self
    }
    pub fn truncate_multiline(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("truncate_multiline", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_chars", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn bind(&mut self) -> TextBinder {
        TextBinder { builder: self }
    }
    pub fn connect(&mut self) -> TextSignals {
        TextSignals { builder: self }
    }
}
impl crate::prelude::Builder for TextBuilder {
    type Target = Text;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TextBuilder {
    type Target = Text;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TextBinder<'builder> {
    builder: &'builder mut TextBuilder,
}
impl<'builder> TextBinder<'builder> {
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn attributes(
        &mut self,
        value: &(impl Prop<pango::AttrList> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("attributes", val));
        value.connect_update(move |value| obj.set_property("attributes", value));
        self.builder
    }
    pub fn invisible_char(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible_char", val));
        value.connect_update(move |value| obj.set_property("invisible_char", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn visibility(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visibility", val));
        value.connect_update(move |value| obj.set_property("visibility", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn editable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable", val));
        value.connect_update(move |value| obj.set_property("editable", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn extra_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("extra_menu", val));
        value.connect_update(move |value| obj.set_property("extra_menu", value));
        self.builder
    }
    pub fn placeholder_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("placeholder_text", val));
        value.connect_update(move |value| obj.set_property("placeholder_text", value));
        self.builder
    }
    pub fn input_hints(&mut self, value: &(impl Prop<InputHints> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_hints", val));
        value.connect_update(move |value| obj.set_property("input_hints", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn tabs(&mut self, value: &(impl Prop<pango::TabArray> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs", val));
        value.connect_update(move |value| obj.set_property("tabs", value));
        self.builder
    }
    pub fn invisible_char_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible_char_set", val));
        value.connect_update(move |value| obj.set_property("invisible_char_set", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn max_width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width_chars", val));
        value.connect_update(move |value| obj.set_property("max_width_chars", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn im_module(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("im_module", val));
        value.connect_update(move |value| obj.set_property("im_module", value));
        self.builder
    }
    pub fn max_length(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_length", val));
        value.connect_update(move |value| obj.set_property("max_length", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn enable_emoji_completion(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_emoji_completion", val));
        value.connect_update(move |value| obj.set_property("enable_emoji_completion", value));
        self.builder
    }
    pub fn overwrite_mode(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overwrite_mode", val));
        value.connect_update(move |value| obj.set_property("overwrite_mode", value));
        self.builder
    }
    pub fn input_purpose(
        &mut self,
        value: &(impl Prop<InputPurpose> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_purpose", val));
        value.connect_update(move |value| obj.set_property("input_purpose", value));
        self.builder
    }
    pub fn activates_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("activates_default", val));
        value.connect_update(move |value| obj.set_property("activates_default", value));
        self.builder
    }
    pub fn css_classes(&mut self, value: &(impl Prop<Vec<String>> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn buffer<T: IsA<EntryBuffer>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("buffer", val));
        value.connect_update(move |value| obj.set_property("buffer", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn enable_undo(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_undo", val));
        value.connect_update(move |value| obj.set_property("enable_undo", value));
        self.builder
    }
    pub fn propagate_text_width(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("propagate_text_width", val));
        value.connect_update(move |value| obj.set_property("propagate_text_width", value));
        self.builder
    }
    pub fn truncate_multiline(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("truncate_multiline", val));
        value.connect_update(move |value| obj.set_property("truncate_multiline", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_chars", val));
        value.connect_update(move |value| obj.set_property("width_chars", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
}
pub struct TextSignals<'builder> {
    builder: &'builder mut TextBuilder,
}
impl<'builder> TextSignals<'builder> {}
impl ForteExt for Text {
    type Builder = TextBuilder;
}
#[macro_export]
macro_rules ! Text { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Text :: forte ()) $ ($ rest) * } } }
