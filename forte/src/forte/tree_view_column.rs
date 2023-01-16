#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, TreeViewColumn, *};
use gtkrs::{
    Buildable, CellArea, CellLayout, CellRenderer, SortType, TreeIter, TreeModel,
    TreeViewColumnSizing, Widget,
};
#[derive(Clone)]
pub struct TreeViewColumnBuilder {
    obj: TreeViewColumn,
}
impl Default for TreeViewColumnBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TreeViewColumnBuilder {
    pub fn max_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("max_width", value);
        self
    }
    pub fn sort_column_id(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("sort_column_id", value);
        self
    }
    pub fn sort_order(&mut self, value: SortType) -> &mut Self {
        self.obj.set_property("sort_order", value);
        self
    }
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("resizable", value);
        self
    }
    pub fn clickable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("clickable", value);
        self
    }
    pub fn sizing(&mut self, value: TreeViewColumnSizing) -> &mut Self {
        self.obj.set_property("sizing", value);
        self
    }
    pub fn widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("widget", value);
        self
    }
    pub fn spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("spacing", value);
        self
    }
    pub fn cell_area(&mut self, value: &impl IsA<CellArea>) -> &mut Self {
        self.obj.set_property("cell_area", value);
        self
    }
    pub fn alignment(&mut self, value: f32) -> &mut Self {
        self.obj.set_property("alignment", value);
        self
    }
    pub fn expand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("expand", value);
        self
    }
    pub fn reorderable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("reorderable", value);
        self
    }
    pub fn sort_indicator(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sort_indicator", value);
        self
    }
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn min_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("min_width", value);
        self
    }
    pub fn fixed_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("fixed_width", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn bind(&mut self) -> TreeViewColumnBinder {
        TreeViewColumnBinder { builder: self }
    }
    pub fn connect(&mut self) -> TreeViewColumnSignals {
        TreeViewColumnSignals { builder: self }
    }
}
impl crate::prelude::Builder for TreeViewColumnBuilder {
    type Target = TreeViewColumn;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TreeViewColumnBuilder {
    type Target = TreeViewColumn;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TreeViewColumnBinder<'builder> {
    builder: &'builder mut TreeViewColumnBuilder,
}
impl<'builder> TreeViewColumnBinder<'builder> {
    pub fn max_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width", val));
        value.connect_update(move |value| obj.set_property("max_width", value));
        self.builder
    }
    pub fn sort_column_id(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sort_column_id", val));
        value.connect_update(move |value| obj.set_property("sort_column_id", value));
        self.builder
    }
    pub fn sort_order(
        &mut self,
        value: &(impl Prop<SortType> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sort_order", val));
        value.connect_update(move |value| obj.set_property("sort_order", value));
        self.builder
    }
    pub fn resizable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("resizable", val));
        value.connect_update(move |value| obj.set_property("resizable", value));
        self.builder
    }
    pub fn clickable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("clickable", val));
        value.connect_update(move |value| obj.set_property("clickable", value));
        self.builder
    }
    pub fn sizing(
        &mut self,
        value: &(impl Prop<TreeViewColumnSizing> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sizing", val));
        value.connect_update(move |value| obj.set_property("sizing", value));
        self.builder
    }
    pub fn widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("widget", val));
        value.connect_update(move |value| obj.set_property("widget", value));
        self.builder
    }
    pub fn spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("spacing", val));
        value.connect_update(move |value| obj.set_property("spacing", value));
        self.builder
    }
    pub fn cell_area<T: IsA<CellArea>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cell_area", val));
        value.connect_update(move |value| obj.set_property("cell_area", value));
        self.builder
    }
    pub fn alignment(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("alignment", val));
        value.connect_update(move |value| obj.set_property("alignment", value));
        self.builder
    }
    pub fn expand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("expand", val));
        value.connect_update(move |value| obj.set_property("expand", value));
        self.builder
    }
    pub fn reorderable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("reorderable", val));
        value.connect_update(move |value| obj.set_property("reorderable", value));
        self.builder
    }
    pub fn sort_indicator(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sort_indicator", val));
        value.connect_update(move |value| obj.set_property("sort_indicator", value));
        self.builder
    }
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
    pub fn min_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("min_width", val));
        value.connect_update(move |value| obj.set_property("min_width", value));
        self.builder
    }
    pub fn fixed_width(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fixed_width", val));
        value.connect_update(move |value| obj.set_property("fixed_width", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TreeViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
}
pub struct TreeViewColumnSignals<'builder> {
    builder: &'builder mut TreeViewColumnBuilder,
}
impl<'builder> TreeViewColumnSignals<'builder> {}
impl ForteExt for TreeViewColumn {
    type Builder = TreeViewColumnBuilder;
}
#[macro_export]
macro_rules ! TreeViewColumn { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TreeViewColumn :: forte ()) $ ($ rest) * } } }
