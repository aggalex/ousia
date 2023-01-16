#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, IconView, *};
use gtkrs::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, CellArea, CellLayout, CellRenderer,
    ConstraintTarget, IconViewDropPosition, LayoutManager, MovementStep, Orientation, Overflow,
    Scrollable, ScrollablePolicy, SelectionMode, Tooltip, TreeIter, TreeModel, TreePath, Widget,
};
#[derive(Clone)]
pub struct IconViewBuilder {
    obj: IconView,
}
impl Default for IconViewBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl IconViewBuilder {
    pub fn column_spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("column_spacing", value);
        self
    }
    pub fn row_spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("row_spacing", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn selection_mode(&mut self, value: SelectionMode) -> &mut Self {
        self.obj.set_property("selection_mode", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn cell_area(&mut self, value: &impl IsA<CellArea>) -> &mut Self {
        self.obj.set_property("cell_area", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn markup_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("markup_column", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<TreeModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn pixbuf_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("pixbuf_column", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn margin(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn reorderable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("reorderable", value);
        self
    }
    pub fn activate_on_single_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("activate_on_single_click", value);
        self
    }
    pub fn spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("spacing", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn hadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("hadjustment", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn item_padding(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("item_padding", value);
        self
    }
    pub fn hscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("hscroll_policy", value);
        self
    }
    pub fn vadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("vadjustment", value);
        self
    }
    pub fn columns(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("columns", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn vscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("vscroll_policy", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn text_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("text_column", value);
        self
    }
    pub fn tooltip_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("tooltip_column", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn item_orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("item_orientation", value);
        self
    }
    pub fn item_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("item_width", value);
        self
    }
    pub fn bind(&mut self) -> IconViewBinder {
        IconViewBinder { builder: self }
    }
    pub fn connect(&mut self) -> IconViewSignals {
        IconViewSignals { builder: self }
    }
}
impl crate::prelude::Builder for IconViewBuilder {
    type Target = IconView;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for IconViewBuilder {
    type Target = IconView;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct IconViewBinder<'builder> {
    builder: &'builder mut IconViewBuilder,
}
impl<'builder> IconViewBinder<'builder> {
    pub fn column_spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("column_spacing", val));
        value.connect_update(move |value| obj.set_property("column_spacing", value));
        self.builder
    }
    pub fn row_spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("row_spacing", val));
        value.connect_update(move |value| obj.set_property("row_spacing", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn selection_mode(
        &mut self,
        value: &(impl Prop<SelectionMode> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("selection_mode", val));
        value.connect_update(move |value| obj.set_property("selection_mode", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn cell_area<T: IsA<CellArea>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_area", val));
        value.connect_update(move |value| obj.set_property("cell_area", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn markup_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("markup_column", val));
        value.connect_update(move |value| obj.set_property("markup_column", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn model<T: IsA<TreeModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn pixbuf_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pixbuf_column", val));
        value.connect_update(move |value| obj.set_property("pixbuf_column", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn margin(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin", val));
        value.connect_update(move |value| obj.set_property("margin", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn reorderable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("reorderable", val));
        value.connect_update(move |value| obj.set_property("reorderable", value));
        self.builder
    }
    pub fn activate_on_single_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("activate_on_single_click", val));
        value.connect_update(move |value| obj.set_property("activate_on_single_click", value));
        self.builder
    }
    pub fn spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("spacing", val));
        value.connect_update(move |value| obj.set_property("spacing", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn hadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hadjustment", val));
        value.connect_update(move |value| obj.set_property("hadjustment", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn item_padding(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("item_padding", val));
        value.connect_update(move |value| obj.set_property("item_padding", value));
        self.builder
    }
    pub fn hscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hscroll_policy", val));
        value.connect_update(move |value| obj.set_property("hscroll_policy", value));
        self.builder
    }
    pub fn vadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vadjustment", val));
        value.connect_update(move |value| obj.set_property("vadjustment", value));
        self.builder
    }
    pub fn columns(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("columns", val));
        value.connect_update(move |value| obj.set_property("columns", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn vscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vscroll_policy", val));
        value.connect_update(move |value| obj.set_property("vscroll_policy", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn text_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text_column", val));
        value.connect_update(move |value| obj.set_property("text_column", value));
        self.builder
    }
    pub fn tooltip_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_column", val));
        value.connect_update(move |value| obj.set_property("tooltip_column", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn item_orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("item_orientation", val));
        value.connect_update(move |value| obj.set_property("item_orientation", value));
        self.builder
    }
    pub fn item_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut IconViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("item_width", val));
        value.connect_update(move |value| obj.set_property("item_width", value));
        self.builder
    }
}
pub struct IconViewSignals<'builder> {
    builder: &'builder mut IconViewBuilder,
}
impl<'builder> IconViewSignals<'builder> {}
impl ForteExt for IconView {
    type Builder = IconViewBuilder;
}
#[macro_export]
macro_rules ! IconView { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: IconView :: forte ()) $ ($ rest) * } } }
