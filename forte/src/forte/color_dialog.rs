#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::Window;
use gtkrs::{prelude::*, ColorDialog, *};
#[derive(Clone)]
pub struct ColorDialogBuilder {
    obj: ColorDialog,
}
impl Default for ColorDialogBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ColorDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("modal", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn with_alpha(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("with_alpha", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn bind(&mut self) -> ColorDialogBinder {
        ColorDialogBinder { builder: self }
    }
    pub fn connect(&mut self) -> ColorDialogSignals {
        ColorDialogSignals { builder: self }
    }
}
impl crate::prelude::Builder for ColorDialogBuilder {
    type Target = ColorDialog;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ColorDialogBuilder {
    type Target = ColorDialog;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ColorDialogBinder<'builder> {
    builder: &'builder mut ColorDialogBuilder,
}
impl<'builder> ColorDialogBinder<'builder> {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ColorDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn with_alpha(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ColorDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("with_alpha", val));
        value.connect_update(move |value| obj.set_property("with_alpha", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ColorDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
}
pub struct ColorDialogSignals<'builder> {
    builder: &'builder mut ColorDialogBuilder,
}
impl<'builder> ColorDialogSignals<'builder> {}
impl ForteExt for ColorDialog {
    type Builder = ColorDialogBuilder;
}
#[macro_export]
macro_rules ! ColorDialog { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ColorDialog :: forte ()) $ ($ rest) * } } }
