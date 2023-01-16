#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, TextView, *};
use gtkrs::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, DeleteType,
    InputHints, InputPurpose, Justification, LayoutManager, MovementStep, Overflow, ScrollStep,
    Scrollable, ScrollablePolicy, TextBuffer, TextChildAnchor, TextExtendSelection, TextIter,
    TextMark, TextWindowType, Widget, WrapMode,
};
#[derive(Clone)]
pub struct TextViewBuilder {
    obj: TextView,
}
impl Default for TextViewBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TextViewBuilder {
    pub fn im_module(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("im_module", value);
        self
    }
    pub fn hscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("hscroll_policy", value);
        self
    }
    pub fn top_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("top_margin", value);
        self
    }
    pub fn overwrite(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("overwrite", value);
        self
    }
    pub fn pixels_inside_wrap(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_inside_wrap", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn right_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("right_margin", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn bottom_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("bottom_margin", value);
        self
    }
    pub fn pixels_above_lines(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_above_lines", value);
        self
    }
    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editable", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn wrap_mode(&mut self, value: WrapMode) -> &mut Self {
        self.obj.set_property("wrap_mode", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn cursor_visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cursor_visible", value);
        self
    }
    pub fn input_purpose(&mut self, value: InputPurpose) -> &mut Self {
        self.obj.set_property("input_purpose", value);
        self
    }
    pub fn extra_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.obj.set_property("extra_menu", value);
        self
    }
    pub fn indent(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("indent", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn vadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("vadjustment", value);
        self
    }
    pub fn input_hints(&mut self, value: InputHints) -> &mut Self {
        self.obj.set_property("input_hints", value);
        self
    }
    pub fn accepts_tab(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("accepts_tab", value);
        self
    }
    pub fn monospace(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("monospace", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.obj.set_property("tabs", value);
        self
    }
    pub fn vscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("vscroll_policy", value);
        self
    }
    pub fn buffer(&mut self, value: &impl IsA<TextBuffer>) -> &mut Self {
        self.obj.set_property("buffer", value);
        self
    }
    pub fn justification(&mut self, value: Justification) -> &mut Self {
        self.obj.set_property("justification", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn left_margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("left_margin", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn pixels_below_lines(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixels_below_lines", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn hadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("hadjustment", value);
        self
    }
    pub fn bind(&mut self) -> TextViewBinder {
        TextViewBinder { builder: self }
    }
    pub fn connect(&mut self) -> TextViewSignals {
        TextViewSignals { builder: self }
    }
}
impl crate::prelude::Builder for TextViewBuilder {
    type Target = TextView;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TextViewBuilder {
    type Target = TextView;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TextViewBinder<'builder> {
    builder: &'builder mut TextViewBuilder,
}
impl<'builder> TextViewBinder<'builder> {
    pub fn im_module(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("im_module", val));
        value.connect_update(move |value| obj.set_property("im_module", value));
        self.builder
    }
    pub fn hscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hscroll_policy", val));
        value.connect_update(move |value| obj.set_property("hscroll_policy", value));
        self.builder
    }
    pub fn top_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("top_margin", val));
        value.connect_update(move |value| obj.set_property("top_margin", value));
        self.builder
    }
    pub fn overwrite(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overwrite", val));
        value.connect_update(move |value| obj.set_property("overwrite", value));
        self.builder
    }
    pub fn pixels_inside_wrap(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_inside_wrap", val));
        value.connect_update(move |value| obj.set_property("pixels_inside_wrap", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn right_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("right_margin", val));
        value.connect_update(move |value| obj.set_property("right_margin", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn bottom_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("bottom_margin", val));
        value.connect_update(move |value| obj.set_property("bottom_margin", value));
        self.builder
    }
    pub fn pixels_above_lines(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_above_lines", val));
        value.connect_update(move |value| obj.set_property("pixels_above_lines", value));
        self.builder
    }
    pub fn editable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editable", val));
        value.connect_update(move |value| obj.set_property("editable", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn wrap_mode(&mut self, value: &(impl Prop<WrapMode> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_mode", val));
        value.connect_update(move |value| obj.set_property("wrap_mode", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn cursor_visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor_visible", val));
        value.connect_update(move |value| obj.set_property("cursor_visible", value));
        self.builder
    }
    pub fn input_purpose(
        &mut self,
        value: &(impl Prop<InputPurpose> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_purpose", val));
        value.connect_update(move |value| obj.set_property("input_purpose", value));
        self.builder
    }
    pub fn extra_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("extra_menu", val));
        value.connect_update(move |value| obj.set_property("extra_menu", value));
        self.builder
    }
    pub fn indent(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("indent", val));
        value.connect_update(move |value| obj.set_property("indent", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn vadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vadjustment", val));
        value.connect_update(move |value| obj.set_property("vadjustment", value));
        self.builder
    }
    pub fn input_hints(
        &mut self,
        value: &(impl Prop<InputHints> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("input_hints", val));
        value.connect_update(move |value| obj.set_property("input_hints", value));
        self.builder
    }
    pub fn accepts_tab(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accepts_tab", val));
        value.connect_update(move |value| obj.set_property("accepts_tab", value));
        self.builder
    }
    pub fn monospace(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("monospace", val));
        value.connect_update(move |value| obj.set_property("monospace", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn tabs(&mut self, value: &(impl Prop<pango::TabArray> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs", val));
        value.connect_update(move |value| obj.set_property("tabs", value));
        self.builder
    }
    pub fn vscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vscroll_policy", val));
        value.connect_update(move |value| obj.set_property("vscroll_policy", value));
        self.builder
    }
    pub fn buffer<T: IsA<TextBuffer>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("buffer", val));
        value.connect_update(move |value| obj.set_property("buffer", value));
        self.builder
    }
    pub fn justification(
        &mut self,
        value: &(impl Prop<Justification> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("justification", val));
        value.connect_update(move |value| obj.set_property("justification", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn left_margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("left_margin", val));
        value.connect_update(move |value| obj.set_property("left_margin", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn pixels_below_lines(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixels_below_lines", val));
        value.connect_update(move |value| obj.set_property("pixels_below_lines", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn hadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hadjustment", val));
        value.connect_update(move |value| obj.set_property("hadjustment", value));
        self.builder
    }
}
pub struct TextViewSignals<'builder> {
    builder: &'builder mut TextViewBuilder,
}
impl<'builder> TextViewSignals<'builder> {
    pub fn preedit_changed(
        &mut self,
        f: impl Fn(&TextView, &str) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_preedit_changed(f);
        &mut self.builder
    }
    pub fn input_purpose_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_input_purpose_notify(f);
        &mut self.builder
    }
    pub fn im_module_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_im_module_notify(f);
        &mut self.builder
    }
    pub fn toggle_cursor_visible(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_toggle_cursor_visible(f);
        &mut self.builder
    }
    pub fn overwrite_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_overwrite_notify(f);
        &mut self.builder
    }
    pub fn right_margin_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_right_margin_notify(f);
        &mut self.builder
    }
    pub fn delete_from_cursor(
        &mut self,
        f: impl Fn(&TextView, DeleteType, i32) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_delete_from_cursor(f);
        &mut self.builder
    }
    pub fn accepts_tab_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_accepts_tab_notify(f);
        &mut self.builder
    }
    pub fn insert_emoji(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_emoji(f);
        &mut self.builder
    }
    pub fn move_viewport(
        &mut self,
        f: impl Fn(&TextView, ScrollStep, i32) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_move_viewport(f);
        &mut self.builder
    }
    pub fn editable_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_editable_notify(f);
        &mut self.builder
    }
    pub fn extra_menu_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_extra_menu_notify(f);
        &mut self.builder
    }
    pub fn indent_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_indent_notify(f);
        &mut self.builder
    }
    pub fn input_hints_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_input_hints_notify(f);
        &mut self.builder
    }
    pub fn top_margin_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_top_margin_notify(f);
        &mut self.builder
    }
    pub fn left_margin_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_left_margin_notify(f);
        &mut self.builder
    }
    pub fn monospace_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_monospace_notify(f);
        &mut self.builder
    }
    pub fn pixels_above_lines_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_above_lines_notify(f);
        &mut self.builder
    }
    pub fn bottom_margin_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_bottom_margin_notify(f);
        &mut self.builder
    }
    pub fn insert_at_cursor(
        &mut self,
        f: impl Fn(&TextView, &str) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_at_cursor(f);
        &mut self.builder
    }
    pub fn cursor_visible_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_cursor_visible_notify(f);
        &mut self.builder
    }
    pub fn backspace(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_backspace(f);
        &mut self.builder
    }
    pub fn buffer_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_buffer_notify(f);
        &mut self.builder
    }
    pub fn justification_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_justification_notify(f);
        &mut self.builder
    }
    pub fn extend_selection(
        &mut self,
        f: impl Fn(&TextView, TextExtendSelection, &TextIter, &TextIter, &TextIter) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_extend_selection(f);
        &mut self.builder
    }
    pub fn copy_clipboard(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_copy_clipboard(f);
        &mut self.builder
    }
    pub fn tabs_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_tabs_notify(f);
        &mut self.builder
    }
    pub fn move_cursor(
        &mut self,
        f: impl Fn(&TextView, MovementStep, i32, bool) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_move_cursor(f);
        &mut self.builder
    }
    pub fn paste_clipboard(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paste_clipboard(f);
        &mut self.builder
    }
    pub fn pixels_inside_wrap_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_inside_wrap_notify(f);
        &mut self.builder
    }
    pub fn select_all(&mut self, f: impl Fn(&TextView, bool) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_select_all(f);
        &mut self.builder
    }
    pub fn pixels_below_lines_notify(
        &mut self,
        f: impl Fn(&TextView) + 'static,
    ) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pixels_below_lines_notify(f);
        &mut self.builder
    }
    pub fn toggle_overwrite(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_toggle_overwrite(f);
        &mut self.builder
    }
    pub fn wrap_mode_notify(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_wrap_mode_notify(f);
        &mut self.builder
    }
    pub fn set_anchor(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_set_anchor(f);
        &mut self.builder
    }
    pub fn cut_clipboard(&mut self, f: impl Fn(&TextView) + 'static) -> &mut TextViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_cut_clipboard(f);
        &mut self.builder
    }
}
impl ForteExt for TextView {
    type Builder = TextViewBuilder;
}
#[macro_export]
macro_rules ! TextView { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TextView :: forte ()) $ ($ rest) * } } }
