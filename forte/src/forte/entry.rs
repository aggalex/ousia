#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Entry, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, CellEditable, ConstraintTarget, Editable,
    EntryBuffer, EntryCompletion, EntryIconPosition, ImageType, InputHints, InputPurpose,
    LayoutManager, Overflow, Widget,
};
#[derive(Clone)]
pub struct EntryBuilder {
    obj: Entry,
}
impl Default for EntryBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl EntryBuilder {
    pub fn secondary_icon_sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("secondary_icon_sensitive", value);
        self
    }
    pub fn primary_icon_paintable(&mut self, value: &impl IsA<gdk::Paintable>) -> &mut Self {
        self.obj.set_property("primary_icon_paintable", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn secondary_icon_tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj
            .set_property("secondary_icon_tooltip_markup", value);
        self
    }
    pub fn enable_emoji_completion(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_emoji_completion", value);
        self
    }
    pub fn has_frame(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_frame", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn progress_pulse_step(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("progress_pulse_step", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn editing_canceled(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editing_canceled", value);
        self
    }
    pub fn im_module(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("im_module", value);
        self
    }
    pub fn truncate_multiline(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("truncate_multiline", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn secondary_icon_paintable(&mut self, value: &impl IsA<gdk::Paintable>) -> &mut Self {
        self.obj.set_property("secondary_icon_paintable", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn max_width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_width_chars", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn buffer(&mut self, value: &impl IsA<EntryBuffer>) -> &mut Self {
        self.obj.set_property("buffer", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn invisible_char_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("invisible_char_set", value);
        self
    }
    pub fn primary_icon_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("primary_icon_name", value);
        self
    }
    pub fn placeholder_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("placeholder_text", value);
        self
    }
    pub fn progress_fraction(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("progress_fraction", value);
        self
    }
    pub fn enable_undo(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_undo", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn input_purpose(&mut self, value: InputPurpose) -> &mut Self {
        self.obj.set_property("input_purpose", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn primary_icon_tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("primary_icon_tooltip_markup", value);
        self
    }
    pub fn secondary_icon_activatable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("secondary_icon_activatable", value);
        self
    }
    pub fn visibility(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visibility", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable", value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("xalign", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn primary_icon_gicon(&mut self, value: &impl IsA<gio::Icon>) -> &mut Self {
        self.obj.set_property("primary_icon_gicon", value);
        self
    }
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.obj.set_property("tabs", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn show_emoji_icon(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_emoji_icon", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn attributes(&mut self, value: &pango::AttrList) -> &mut Self {
        self.obj.set_property("attributes", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn extra_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.obj.set_property("extra_menu", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn invisible_char(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("invisible_char", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn width_chars(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_chars", value);
        self
    }
    pub fn secondary_icon_gicon(&mut self, value: &impl IsA<gio::Icon>) -> &mut Self {
        self.obj.set_property("secondary_icon_gicon", value);
        self
    }
    pub fn overwrite_mode(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("overwrite_mode", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn completion(&mut self, value: &EntryCompletion) -> &mut Self {
        self.obj.set_property("completion", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn input_hints(&mut self, value: InputHints) -> &mut Self {
        self.obj.set_property("input_hints", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn primary_icon_activatable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("primary_icon_activatable", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn secondary_icon_tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("secondary_icon_tooltip_text", value);
        self
    }
    pub fn secondary_icon_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("secondary_icon_name", value);
        self
    }
    pub fn primary_icon_tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("primary_icon_tooltip_text", value);
        self
    }
    pub fn activates_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("activates_default", value);
        self
    }
    pub fn primary_icon_sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("primary_icon_sensitive", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn max_length(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_length", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn bind(&mut self) -> EntryBinder {
        EntryBinder { builder: self }
    }
    pub fn connect(&mut self) -> EntrySignals {
        EntrySignals { builder: self }
    }
}
impl crate::prelude::Builder for EntryBuilder {
    type Target = Entry;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for EntryBuilder {
    type Target = Entry;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct EntryBinder<'builder> {
    builder: &'builder mut EntryBuilder,
}
impl<'builder> EntryBinder<'builder> {
    pub fn secondary_icon_sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_sensitive", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_sensitive", value));
        self.builder
    }
    pub fn primary_icon_paintable<T: IsA<gdk::Paintable>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_paintable", val));
        value.connect_update(move |value| obj.set_property("primary_icon_paintable", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn secondary_icon_tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_tooltip_markup", value));
        self.builder
    }
    pub fn enable_emoji_completion(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_emoji_completion", val));
        value.connect_update(move |value| obj.set_property("enable_emoji_completion", value));
        self.builder
    }
    pub fn has_frame(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_frame", val));
        value.connect_update(move |value| obj.set_property("has_frame", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn progress_pulse_step(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("progress_pulse_step", val));
        value.connect_update(move |value| obj.set_property("progress_pulse_step", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn editing_canceled(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editing_canceled", val));
        value.connect_update(move |value| obj.set_property("editing_canceled", value));
        self.builder
    }
    pub fn im_module(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("im_module", val));
        value.connect_update(move |value| obj.set_property("im_module", value));
        self.builder
    }
    pub fn truncate_multiline(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("truncate_multiline", val));
        value.connect_update(move |value| obj.set_property("truncate_multiline", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn secondary_icon_paintable<T: IsA<gdk::Paintable>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_paintable", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_paintable", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn max_width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width_chars", val));
        value.connect_update(move |value| obj.set_property("max_width_chars", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn buffer<T: IsA<EntryBuffer>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("buffer", val));
        value.connect_update(move |value| obj.set_property("buffer", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn invisible_char_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible_char_set", val));
        value.connect_update(move |value| obj.set_property("invisible_char_set", value));
        self.builder
    }
    pub fn primary_icon_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_name", val));
        value.connect_update(move |value| obj.set_property("primary_icon_name", value));
        self.builder
    }
    pub fn placeholder_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("placeholder_text", val));
        value.connect_update(move |value| obj.set_property("placeholder_text", value));
        self.builder
    }
    pub fn progress_fraction(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("progress_fraction", val));
        value.connect_update(move |value| obj.set_property("progress_fraction", value));
        self.builder
    }
    pub fn enable_undo(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_undo", val));
        value.connect_update(move |value| obj.set_property("enable_undo", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn input_purpose(
        &mut self,
        value: &(impl Prop<InputPurpose> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_purpose", val));
        value.connect_update(move |value| obj.set_property("input_purpose", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn primary_icon_tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("primary_icon_tooltip_markup", value));
        self.builder
    }
    pub fn secondary_icon_activatable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_activatable", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_activatable", value));
        self.builder
    }
    pub fn visibility(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visibility", val));
        value.connect_update(move |value| obj.set_property("visibility", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn editable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable", val));
        value.connect_update(move |value| obj.set_property("editable", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn primary_icon_gicon<T: IsA<gio::Icon>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_gicon", val));
        value.connect_update(move |value| obj.set_property("primary_icon_gicon", value));
        self.builder
    }
    pub fn tabs(&mut self, value: &(impl Prop<pango::TabArray> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs", val));
        value.connect_update(move |value| obj.set_property("tabs", value));
        self.builder
    }
    pub fn css_classes(&mut self, value: &(impl Prop<Vec<String>> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn show_emoji_icon(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_emoji_icon", val));
        value.connect_update(move |value| obj.set_property("show_emoji_icon", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn attributes(
        &mut self,
        value: &(impl Prop<pango::AttrList> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("attributes", val));
        value.connect_update(move |value| obj.set_property("attributes", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn extra_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("extra_menu", val));
        value.connect_update(move |value| obj.set_property("extra_menu", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn invisible_char(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("invisible_char", val));
        value.connect_update(move |value| obj.set_property("invisible_char", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_chars", val));
        value.connect_update(move |value| obj.set_property("width_chars", value));
        self.builder
    }
    pub fn secondary_icon_gicon<T: IsA<gio::Icon>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_gicon", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_gicon", value));
        self.builder
    }
    pub fn overwrite_mode(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overwrite_mode", val));
        value.connect_update(move |value| obj.set_property("overwrite_mode", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn completion(
        &mut self,
        value: &(impl Prop<EntryCompletion> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("completion", val));
        value.connect_update(move |value| obj.set_property("completion", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn input_hints(&mut self, value: &(impl Prop<InputHints> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_hints", val));
        value.connect_update(move |value| obj.set_property("input_hints", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn primary_icon_activatable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_activatable", val));
        value.connect_update(move |value| obj.set_property("primary_icon_activatable", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn secondary_icon_tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_tooltip_text", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_tooltip_text", value));
        self.builder
    }
    pub fn secondary_icon_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_icon_name", val));
        value.connect_update(move |value| obj.set_property("secondary_icon_name", value));
        self.builder
    }
    pub fn primary_icon_tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_tooltip_text", val));
        value.connect_update(move |value| obj.set_property("primary_icon_tooltip_text", value));
        self.builder
    }
    pub fn activates_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("activates_default", val));
        value.connect_update(move |value| obj.set_property("activates_default", value));
        self.builder
    }
    pub fn primary_icon_sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary_icon_sensitive", val));
        value.connect_update(move |value| obj.set_property("primary_icon_sensitive", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn max_length(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_length", val));
        value.connect_update(move |value| obj.set_property("max_length", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
}
pub struct EntrySignals<'builder> {
    builder: &'builder mut EntryBuilder,
}
impl<'builder> EntrySignals<'builder> {
    pub fn progress_pulse_step_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_progress_pulse_step_notify(f);
        &mut self.builder
    }
    pub fn extra_menu_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_extra_menu_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_activatable_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_activatable_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_name_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_name_notify(f);
        &mut self.builder
    }
    pub fn activates_default_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_activates_default_notify(f);
        &mut self.builder
    }
    pub fn input_hints_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_input_hints_notify(f);
        &mut self.builder
    }
    pub fn placeholder_text_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_placeholder_text_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_tooltip_text_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_tooltip_text_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_tooltip_markup_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_tooltip_markup_notify(f);
        &mut self.builder
    }
    pub fn invisible_char_set_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_invisible_char_set_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_name_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_name_notify(f);
        &mut self.builder
    }
    pub fn truncate_multiline_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_truncate_multiline_notify(f);
        &mut self.builder
    }
    pub fn text_length_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_text_length_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_storage_type_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_storage_type_notify(f);
        &mut self.builder
    }
    pub fn scroll_offset_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_scroll_offset_notify(f);
        &mut self.builder
    }
    pub fn show_emoji_icon_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_show_emoji_icon_notify(f);
        &mut self.builder
    }
    pub fn tabs_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_tabs_notify(f);
        &mut self.builder
    }
    pub fn overwrite_mode_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overwrite_mode_notify(f);
        &mut self.builder
    }
    pub fn visibility_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_visibility_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_gicon_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_gicon_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_sensitive_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_sensitive_notify(f);
        &mut self.builder
    }
    pub fn has_frame_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_frame_notify(f);
        &mut self.builder
    }
    pub fn im_module_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_im_module_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_tooltip_text_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_tooltip_text_notify(f);
        &mut self.builder
    }
    pub fn invisible_char_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_invisible_char_notify(f);
        &mut self.builder
    }
    pub fn enable_emoji_completion_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_enable_emoji_completion_notify(f);
        &mut self.builder
    }
    pub fn completion_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_completion_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_tooltip_markup_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_tooltip_markup_notify(f);
        &mut self.builder
    }
    pub fn input_purpose_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_input_purpose_notify(f);
        &mut self.builder
    }
    pub fn attributes_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_attributes_notify(f);
        &mut self.builder
    }
    pub fn max_length_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_max_length_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_storage_type_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_storage_type_notify(f);
        &mut self.builder
    }
    pub fn primary_icon_paintable_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_primary_icon_paintable_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_paintable_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_paintable_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_activatable_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_activatable_notify(f);
        &mut self.builder
    }
    pub fn icon_press(
        &mut self,
        f: impl Fn(&Entry, EntryIconPosition) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_icon_press(f);
        &mut self.builder
    }
    pub fn buffer_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_buffer_notify(f);
        &mut self.builder
    }
    pub fn activate(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_activate(f);
        &mut self.builder
    }
    pub fn progress_fraction_notify(&mut self, f: impl Fn(&Entry) + 'static) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_progress_fraction_notify(f);
        &mut self.builder
    }
    pub fn icon_release(
        &mut self,
        f: impl Fn(&Entry, EntryIconPosition) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_icon_release(f);
        &mut self.builder
    }
    pub fn secondary_icon_gicon_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_gicon_notify(f);
        &mut self.builder
    }
    pub fn secondary_icon_sensitive_notify(
        &mut self,
        f: impl Fn(&Entry) + 'static,
    ) -> &mut EntryBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_secondary_icon_sensitive_notify(f);
        &mut self.builder
    }
}
impl ForteExt for Entry {
    type Builder = EntryBuilder;
}
#[macro_export]
macro_rules ! Entry { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Entry :: forte ()) $ ($ rest) * } } }
