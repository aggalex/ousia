#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::Window;
use gtkrs::{prelude::*, AlertDialog, *};
#[derive(Clone)]
pub struct AlertDialogBuilder {
    obj: AlertDialog,
}
impl Default for AlertDialogBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl AlertDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn message(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("message", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn buttons(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("buttons", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("modal", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn cancel_button(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("cancel_button", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn default_button(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("default_button", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn detail(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("detail", value);
        self
    }
    pub fn bind(&mut self) -> AlertDialogBinder {
        AlertDialogBinder { builder: self }
    }
    pub fn connect(&mut self) -> AlertDialogSignals {
        AlertDialogSignals { builder: self }
    }
}
impl crate::prelude::Builder for AlertDialogBuilder {
    type Target = AlertDialog;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for AlertDialogBuilder {
    type Target = AlertDialog;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct AlertDialogBinder<'builder> {
    builder: &'builder mut AlertDialogBuilder,
}
impl<'builder> AlertDialogBinder<'builder> {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn message(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("message", val));
        value.connect_update(move |value| obj.set_property("message", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn buttons(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("buttons", val));
        value.connect_update(move |value| obj.set_property("buttons", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn cancel_button(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cancel_button", val));
        value.connect_update(move |value| obj.set_property("cancel_button", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn default_button(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_button", val));
        value.connect_update(move |value| obj.set_property("default_button", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn detail(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut AlertDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("detail", val));
        value.connect_update(move |value| obj.set_property("detail", value));
        self.builder
    }
}
pub struct AlertDialogSignals<'builder> {
    builder: &'builder mut AlertDialogBuilder,
}
impl<'builder> AlertDialogSignals<'builder> {}
impl ForteExt for AlertDialog {
    type Builder = AlertDialogBuilder;
}
#[macro_export]
macro_rules ! AlertDialog { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: AlertDialog :: forte ()) $ ($ rest) * } } }
