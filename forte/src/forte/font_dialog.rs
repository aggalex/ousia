#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, FontDialog, *};
use gtkrs::{Filter, Window};
#[derive(Clone)]
pub struct FontDialogBuilder {
    obj: FontDialog,
}
impl Default for FontDialogBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl FontDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("modal", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn language(&mut self, value: &pango::Language) -> &mut Self {
        self.obj.set_property("language", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn font_map(&mut self, value: &impl IsA<pango::FontMap>) -> &mut Self {
        self.obj.set_property("font_map", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn filter(&mut self, value: &impl IsA<Filter>) -> &mut Self {
        self.obj.set_property("filter", value);
        self
    }
    pub fn bind(&mut self) -> FontDialogBinder {
        FontDialogBinder { builder: self }
    }
    pub fn connect(&mut self) -> FontDialogSignals {
        FontDialogSignals { builder: self }
    }
}
impl crate::prelude::Builder for FontDialogBuilder {
    type Target = FontDialog;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for FontDialogBuilder {
    type Target = FontDialog;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct FontDialogBinder<'builder> {
    builder: &'builder mut FontDialogBuilder,
}
impl<'builder> FontDialogBinder<'builder> {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut FontDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn language(
        &mut self,
        value: &(impl Prop<pango::Language> + ?Sized),
    ) -> &mut FontDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("language", val));
        value.connect_update(move |value| obj.set_property("language", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn font_map<T: IsA<pango::FontMap>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FontDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("font_map", val));
        value.connect_update(move |value| obj.set_property("font_map", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut FontDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn filter<T: IsA<Filter>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FontDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("filter", val));
        value.connect_update(move |value| obj.set_property("filter", value));
        self.builder
    }
}
pub struct FontDialogSignals<'builder> {
    builder: &'builder mut FontDialogBuilder,
}
impl<'builder> FontDialogSignals<'builder> {}
impl ForteExt for FontDialog {
    type Builder = FontDialogBuilder;
}
#[macro_export]
macro_rules ! FontDialog { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: FontDialog :: forte ()) $ ($ rest) * } } }
