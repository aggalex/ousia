#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, IconTheme, *};
use gtkrs::{IconLookupFlags, IconPaintable, TextDirection};
#[derive(Clone)]
pub struct IconThemeBuilder {
    obj: IconTheme,
}
impl Default for IconThemeBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl IconThemeBuilder {
    pub fn display(&mut self, value: &impl IsA<gdk::Display>) -> &mut Self {
        self.obj.set_property("display", value);
        self
    }
    pub fn search_path(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("search_path", value);
        self
    }
    pub fn resource_path(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("resource_path", value);
        self
    }
    pub fn theme_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("theme_name", value);
        self
    }
    pub fn bind(&mut self) -> IconThemeBinder {
        IconThemeBinder { builder: self }
    }
    pub fn connect(&mut self) -> IconThemeSignals {
        IconThemeSignals { builder: self }
    }
}
impl crate::prelude::Builder for IconThemeBuilder {
    type Target = IconTheme;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for IconThemeBuilder {
    type Target = IconTheme;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct IconThemeBinder<'builder> {
    builder: &'builder mut IconThemeBuilder,
}
impl<'builder> IconThemeBinder<'builder> {
    pub fn display<T: IsA<gdk::Display>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut IconThemeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("display", val));
        value.connect_update(move |value| obj.set_property("display", value));
        self.builder
    }
    pub fn search_path(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut IconThemeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("search_path", val));
        value.connect_update(move |value| obj.set_property("search_path", value));
        self.builder
    }
    pub fn resource_path(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut IconThemeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("resource_path", val));
        value.connect_update(move |value| obj.set_property("resource_path", value));
        self.builder
    }
    pub fn theme_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut IconThemeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("theme_name", val));
        value.connect_update(move |value| obj.set_property("theme_name", value));
        self.builder
    }
}
pub struct IconThemeSignals<'builder> {
    builder: &'builder mut IconThemeBuilder,
}
impl<'builder> IconThemeSignals<'builder> {}
impl ForteExt for IconTheme {
    type Builder = IconThemeBuilder;
}
#[macro_export]
macro_rules ! IconTheme { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: IconTheme :: forte ()) $ ($ rest) * } } }
