#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, TreeView, *};
use gtkrs::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, CellRenderer, ConstraintTarget,
    Editable, LayoutManager, MovementStep, Overflow, Scrollable, ScrollablePolicy, Tooltip,
    TreeIter, TreeModel, TreePath, TreeSelection, TreeViewColumn, TreeViewDropPosition,
    TreeViewGridLines, Widget,
};
#[derive(Clone)]
pub struct TreeViewBuilder {
    obj: TreeView,
}
impl Default for TreeViewBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TreeViewBuilder {
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn hadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("hadjustment", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn level_indentation(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("level_indentation", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn activate_on_single_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("activate_on_single_click", value);
        self
    }
    pub fn rubber_banding(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("rubber_banding", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn hscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("hscroll_policy", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn vadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("vadjustment", value);
        self
    }
    pub fn expander_column(&mut self, value: &TreeViewColumn) -> &mut Self {
        self.obj.set_property("expander_column", value);
        self
    }
    pub fn enable_grid_lines(&mut self, value: TreeViewGridLines) -> &mut Self {
        self.obj.set_property("enable_grid_lines", value);
        self
    }
    pub fn show_expanders(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_expanders", value);
        self
    }
    pub fn enable_search(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_search", value);
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
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn fixed_height_mode(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("fixed_height_mode", value);
        self
    }
    pub fn hover_selection(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hover_selection", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<TreeModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn headers_visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("headers_visible", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn tooltip_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("tooltip_column", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn hover_expand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hover_expand", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn vscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("vscroll_policy", value);
        self
    }
    pub fn search_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("search_column", value);
        self
    }
    pub fn enable_tree_lines(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_tree_lines", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn reorderable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("reorderable", value);
        self
    }
    pub fn headers_clickable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("headers_clickable", value);
        self
    }
    pub fn bind(&mut self) -> TreeViewBinder {
        TreeViewBinder { builder: self }
    }
    pub fn connect(&mut self) -> TreeViewSignals {
        TreeViewSignals { builder: self }
    }
}
impl crate::prelude::Builder for TreeViewBuilder {
    type Target = TreeView;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TreeViewBuilder {
    type Target = TreeView;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TreeViewBinder<'builder> {
    builder: &'builder mut TreeViewBuilder,
}
impl<'builder> TreeViewBinder<'builder> {
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn hadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hadjustment", val));
        value.connect_update(move |value| obj.set_property("hadjustment", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn level_indentation(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("level_indentation", val));
        value.connect_update(move |value| obj.set_property("level_indentation", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn activate_on_single_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("activate_on_single_click", val));
        value.connect_update(move |value| obj.set_property("activate_on_single_click", value));
        self.builder
    }
    pub fn rubber_banding(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rubber_banding", val));
        value.connect_update(move |value| obj.set_property("rubber_banding", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn hscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hscroll_policy", val));
        value.connect_update(move |value| obj.set_property("hscroll_policy", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn vadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vadjustment", val));
        value.connect_update(move |value| obj.set_property("vadjustment", value));
        self.builder
    }
    pub fn expander_column(
        &mut self,
        value: &(impl Prop<TreeViewColumn> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("expander_column", val));
        value.connect_update(move |value| obj.set_property("expander_column", value));
        self.builder
    }
    pub fn enable_grid_lines(
        &mut self,
        value: &(impl Prop<TreeViewGridLines> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_grid_lines", val));
        value.connect_update(move |value| obj.set_property("enable_grid_lines", value));
        self.builder
    }
    pub fn show_expanders(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_expanders", val));
        value.connect_update(move |value| obj.set_property("show_expanders", value));
        self.builder
    }
    pub fn enable_search(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_search", val));
        value.connect_update(move |value| obj.set_property("enable_search", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn fixed_height_mode(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fixed_height_mode", val));
        value.connect_update(move |value| obj.set_property("fixed_height_mode", value));
        self.builder
    }
    pub fn hover_selection(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hover_selection", val));
        value.connect_update(move |value| obj.set_property("hover_selection", value));
        self.builder
    }
    pub fn model<T: IsA<TreeModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn headers_visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("headers_visible", val));
        value.connect_update(move |value| obj.set_property("headers_visible", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn tooltip_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_column", val));
        value.connect_update(move |value| obj.set_property("tooltip_column", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn hover_expand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hover_expand", val));
        value.connect_update(move |value| obj.set_property("hover_expand", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn vscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vscroll_policy", val));
        value.connect_update(move |value| obj.set_property("vscroll_policy", value));
        self.builder
    }
    pub fn search_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("search_column", val));
        value.connect_update(move |value| obj.set_property("search_column", value));
        self.builder
    }
    pub fn enable_tree_lines(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_tree_lines", val));
        value.connect_update(move |value| obj.set_property("enable_tree_lines", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn reorderable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("reorderable", val));
        value.connect_update(move |value| obj.set_property("reorderable", value));
        self.builder
    }
    pub fn headers_clickable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("headers_clickable", val));
        value.connect_update(move |value| obj.set_property("headers_clickable", value));
        self.builder
    }
}
pub struct TreeViewSignals<'builder> {
    builder: &'builder mut TreeViewBuilder,
}
impl<'builder> TreeViewSignals<'builder> {
    pub fn row_collapsed(
        &mut self,
        f: impl Fn(&TreeView, &TreeIter, &TreePath) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_row_collapsed(f);
        &mut self.builder
    }
    pub fn headers_clickable_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_headers_clickable_notify(f);
        &mut self.builder
    }
    pub fn hover_expand_notify(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_hover_expand_notify(f);
        &mut self.builder
    }
    pub fn enable_grid_lines_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_enable_grid_lines_notify(f);
        &mut self.builder
    }
    pub fn fixed_height_mode_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_fixed_height_mode_notify(f);
        &mut self.builder
    }
    pub fn select_cursor_row(
        &mut self,
        f: impl Fn(&TreeView, bool) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_select_cursor_row(f);
        &mut self.builder
    }
    pub fn cursor_changed(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_cursor_changed(f);
        &mut self.builder
    }
    pub fn expander_column_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_expander_column_notify(f);
        &mut self.builder
    }
    pub fn test_collapse_row(
        &mut self,
        f: impl Fn(&TreeView, &TreeIter, &TreePath) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_test_collapse_row(f);
        &mut self.builder
    }
    pub fn show_expanders_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_show_expanders_notify(f);
        &mut self.builder
    }
    pub fn unselect_all(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_unselect_all(f);
        &mut self.builder
    }
    pub fn enable_tree_lines_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_enable_tree_lines_notify(f);
        &mut self.builder
    }
    pub fn headers_visible_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_headers_visible_notify(f);
        &mut self.builder
    }
    pub fn expand_collapse_cursor_row(
        &mut self,
        f: impl Fn(&TreeView, bool, bool, bool) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_expand_collapse_cursor_row(f);
        &mut self.builder
    }
    pub fn model_notify(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_model_notify(f);
        &mut self.builder
    }
    pub fn hover_selection_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_hover_selection_notify(f);
        &mut self.builder
    }
    pub fn test_expand_row(
        &mut self,
        f: impl Fn(&TreeView, &TreeIter, &TreePath) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_test_expand_row(f);
        &mut self.builder
    }
    pub fn tooltip_column_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_tooltip_column_notify(f);
        &mut self.builder
    }
    pub fn level_indentation_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_level_indentation_notify(f);
        &mut self.builder
    }
    pub fn rubber_banding_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_rubber_banding_notify(f);
        &mut self.builder
    }
    pub fn start_interactive_search(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_start_interactive_search(f);
        &mut self.builder
    }
    pub fn move_cursor(
        &mut self,
        f: impl Fn(&TreeView, MovementStep, i32, bool, bool) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_move_cursor(f);
        &mut self.builder
    }
    pub fn columns_changed(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_columns_changed(f);
        &mut self.builder
    }
    pub fn toggle_cursor_row(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_toggle_cursor_row(f);
        &mut self.builder
    }
    pub fn enable_search_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_enable_search_notify(f);
        &mut self.builder
    }
    pub fn search_column_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_search_column_notify(f);
        &mut self.builder
    }
    pub fn select_all(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_select_all(f);
        &mut self.builder
    }
    pub fn reorderable_notify(&mut self, f: impl Fn(&TreeView) + 'static) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_reorderable_notify(f);
        &mut self.builder
    }
    pub fn row_expanded(
        &mut self,
        f: impl Fn(&TreeView, &TreeIter, &TreePath) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_row_expanded(f);
        &mut self.builder
    }
    pub fn activate_on_single_click_notify(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_activate_on_single_click_notify(f);
        &mut self.builder
    }
    pub fn select_cursor_parent(
        &mut self,
        f: impl Fn(&TreeView) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_select_cursor_parent(f);
        &mut self.builder
    }
    pub fn row_activated(
        &mut self,
        f: impl Fn(&TreeView, &TreePath, Option<&TreeViewColumn>) + 'static,
    ) -> &mut TreeViewBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_row_activated(f);
        &mut self.builder
    }
}
impl ForteExt for TreeView {
    type Builder = TreeViewBuilder;
}
#[macro_export]
macro_rules ! TreeView { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TreeView :: forte ()) $ ($ rest) * } } }
