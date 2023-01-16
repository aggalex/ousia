#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, FileDialog, *};
use gtkrs::{FileFilter, Window};
#[derive(Clone)]
pub struct FileDialogBuilder {
    obj: FileDialog,
}
impl Default for FileDialogBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl FileDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("initial_name", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn shortcut_folders(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("shortcut_folders", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn filters(&mut self, value: &impl IsA<gio::ListModel>) -> &mut Self {
        self.obj.set_property("filters", value);
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
    pub fn default_filter(&mut self, value: &FileFilter) -> &mut Self {
        self.obj.set_property("default_filter", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_file(&mut self, value: &impl IsA<gio::File>) -> &mut Self {
        self.obj.set_property("initial_file", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_folder(&mut self, value: &impl IsA<gio::File>) -> &mut Self {
        self.obj.set_property("initial_folder", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn accept_label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("accept_label", value);
        self
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn bind(&mut self) -> FileDialogBinder {
        FileDialogBinder { builder: self }
    }
    pub fn connect(&mut self) -> FileDialogSignals {
        FileDialogSignals { builder: self }
    }
}
impl crate::prelude::Builder for FileDialogBuilder {
    type Target = FileDialog;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for FileDialogBuilder {
    type Target = FileDialog;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct FileDialogBinder<'builder> {
    builder: &'builder mut FileDialogBuilder,
}
impl<'builder> FileDialogBinder<'builder> {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("initial_name", val));
        value.connect_update(move |value| obj.set_property("initial_name", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn shortcut_folders<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("shortcut_folders", val));
        value.connect_update(move |value| obj.set_property("shortcut_folders", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn filters<T: IsA<gio::ListModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("filters", val));
        value.connect_update(move |value| obj.set_property("filters", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn default_filter(
        &mut self,
        value: &(impl Prop<FileFilter> + ?Sized),
    ) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_filter", val));
        value.connect_update(move |value| obj.set_property("default_filter", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_file<T: IsA<gio::File>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("initial_file", val));
        value.connect_update(move |value| obj.set_property("initial_file", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_folder<T: IsA<gio::File>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("initial_folder", val));
        value.connect_update(move |value| obj.set_property("initial_folder", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn accept_label(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accept_label", val));
        value.connect_update(move |value| obj.set_property("accept_label", value));
        self.builder
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut FileDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
}
pub struct FileDialogSignals<'builder> {
    builder: &'builder mut FileDialogBuilder,
}
impl<'builder> FileDialogSignals<'builder> {}
impl ForteExt for FileDialog {
    type Builder = FileDialogBuilder;
}
#[macro_export]
macro_rules ! FileDialog { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: FileDialog :: forte ()) $ ($ rest) * } } }
