#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ColumnViewColumn, *};
use gtkrs::{ColumnView, ListItemFactory, Sorter};
#[derive(Clone)]
pub struct ColumnViewColumnBuilder {
    obj: ColumnViewColumn,
}
impl Default for ColumnViewColumnBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ColumnViewColumnBuilder {
    pub fn factory(&mut self, value: &impl IsA<ListItemFactory>) -> &mut Self {
        self.obj.set_property("factory", value);
        self
    }
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("resizable", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn id(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("id", value);
        self
    }
    pub fn fixed_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("fixed_width", value);
        self
    }
    pub fn sorter(&mut self, value: &impl IsA<Sorter>) -> &mut Self {
        self.obj.set_property("sorter", value);
        self
    }
    pub fn header_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.obj.set_property("header_menu", value);
        self
    }
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn expand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("expand", value);
        self
    }
    pub fn bind(&mut self) -> ColumnViewColumnBinder {
        ColumnViewColumnBinder { builder: self }
    }
    pub fn connect(&mut self) -> ColumnViewColumnSignals {
        ColumnViewColumnSignals { builder: self }
    }
}
impl crate::prelude::Builder for ColumnViewColumnBuilder {
    type Target = ColumnViewColumn;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ColumnViewColumnBuilder {
    type Target = ColumnViewColumn;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ColumnViewColumnBinder<'builder> {
    builder: &'builder mut ColumnViewColumnBuilder,
}
impl<'builder> ColumnViewColumnBinder<'builder> {
    pub fn factory<T: IsA<ListItemFactory>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("factory", val));
        value.connect_update(move |value| obj.set_property("factory", value));
        self.builder
    }
    pub fn resizable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("resizable", val));
        value.connect_update(move |value| obj.set_property("resizable", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn id(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("id", val));
        value.connect_update(move |value| obj.set_property("id", value));
        self.builder
    }
    pub fn fixed_width(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fixed_width", val));
        value.connect_update(move |value| obj.set_property("fixed_width", value));
        self.builder
    }
    pub fn sorter<T: IsA<Sorter>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sorter", val));
        value.connect_update(move |value| obj.set_property("sorter", value));
        self.builder
    }
    pub fn header_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("header_menu", val));
        value.connect_update(move |value| obj.set_property("header_menu", value));
        self.builder
    }
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn expand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ColumnViewColumnBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("expand", val));
        value.connect_update(move |value| obj.set_property("expand", value));
        self.builder
    }
}
pub struct ColumnViewColumnSignals<'builder> {
    builder: &'builder mut ColumnViewColumnBuilder,
}
impl<'builder> ColumnViewColumnSignals<'builder> {}
impl ForteExt for ColumnViewColumn {
    type Builder = ColumnViewColumnBuilder;
}
#[macro_export]
macro_rules ! ColumnViewColumn { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ColumnViewColumn :: forte ()) $ ($ rest) * } } }
