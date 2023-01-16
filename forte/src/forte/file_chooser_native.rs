#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, FileChooserNative, *};
use gtkrs::{FileChooser, FileChooserAction, FileFilter, NativeDialog, Window};
#[derive(Clone)]
pub struct FileChooserNativeBuilder {
    obj: FileChooserNative,
}
impl Default for FileChooserNativeBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl FileChooserNativeBuilder {
    pub fn transient_for(&mut self, value: &impl IsA<Window>) -> &mut Self {
        self.obj.set_property("transient_for", value);
        self
    }
    pub fn cancel_label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("cancel_label", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn create_folders(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("create_folders", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn filter(&mut self, value: &FileFilter) -> &mut Self {
        self.obj.set_property("filter", value);
        self
    }
    pub fn accept_label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("accept_label", value);
        self
    }
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("modal", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn action(&mut self, value: FileChooserAction) -> &mut Self {
        self.obj.set_property("action", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn select_multiple(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("select_multiple", value);
        self
    }
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn bind(&mut self) -> FileChooserNativeBinder {
        FileChooserNativeBinder { builder: self }
    }
    pub fn connect(&mut self) -> FileChooserNativeSignals {
        FileChooserNativeSignals { builder: self }
    }
}
impl crate::prelude::Builder for FileChooserNativeBuilder {
    type Target = FileChooserNative;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for FileChooserNativeBuilder {
    type Target = FileChooserNative;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct FileChooserNativeBinder<'builder> {
    builder: &'builder mut FileChooserNativeBuilder,
}
impl<'builder> FileChooserNativeBinder<'builder> {
    pub fn transient_for<T: IsA<Window>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("transient_for", val));
        value.connect_update(move |value| obj.set_property("transient_for", value));
        self.builder
    }
    pub fn cancel_label(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cancel_label", val));
        value.connect_update(move |value| obj.set_property("cancel_label", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn create_folders(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("create_folders", val));
        value.connect_update(move |value| obj.set_property("create_folders", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn filter(
        &mut self,
        value: &(impl Prop<FileFilter> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("filter", val));
        value.connect_update(move |value| obj.set_property("filter", value));
        self.builder
    }
    pub fn accept_label(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accept_label", val));
        value.connect_update(move |value| obj.set_property("accept_label", value));
        self.builder
    }
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn action(
        &mut self,
        value: &(impl Prop<FileChooserAction> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action", val));
        value.connect_update(move |value| obj.set_property("action", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn select_multiple(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("select_multiple", val));
        value.connect_update(move |value| obj.set_property("select_multiple", value));
        self.builder
    }
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut FileChooserNativeBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
}
pub struct FileChooserNativeSignals<'builder> {
    builder: &'builder mut FileChooserNativeBuilder,
}
impl<'builder> FileChooserNativeSignals<'builder> {}
impl ForteExt for FileChooserNative {
    type Builder = FileChooserNativeBuilder;
}
#[macro_export]
macro_rules ! FileChooserNative { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: FileChooserNative :: forte ()) $ ($ rest) * } } }
