#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::Window;
use gtkrs::{prelude::*, MountOperation, *};
#[derive(Clone)]
pub struct MountOperationBuilder {
    obj: MountOperation,
}
impl Default for MountOperationBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl MountOperationBuilder {
    pub fn username(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("username", value);
        self
    }
    pub fn anonymous(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("anonymous", value);
        self
    }
    pub fn display(&mut self, value: &impl IsA<gdk::Display>) -> &mut Self {
        self.obj.set_property("display", value);
        self
    }
    pub fn pim(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("pim", value);
        self
    }
    pub fn password(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("password", value);
        self
    }
    pub fn parent(&mut self, value: &impl IsA<Window>) -> &mut Self {
        self.obj.set_property("parent", value);
        self
    }
    pub fn choice(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("choice", value);
        self
    }
    pub fn domain(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("domain", value);
        self
    }
    pub fn is_tcrypt_hidden_volume(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_tcrypt_hidden_volume", value);
        self
    }
    pub fn password_save(&mut self, value: gio::PasswordSave) -> &mut Self {
        self.obj.set_property("password_save", value);
        self
    }
    pub fn is_tcrypt_system_volume(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("is_tcrypt_system_volume", value);
        self
    }
    pub fn bind(&mut self) -> MountOperationBinder {
        MountOperationBinder { builder: self }
    }
    pub fn connect(&mut self) -> MountOperationSignals {
        MountOperationSignals { builder: self }
    }
}
impl crate::prelude::Builder for MountOperationBuilder {
    type Target = MountOperation;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for MountOperationBuilder {
    type Target = MountOperation;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct MountOperationBinder<'builder> {
    builder: &'builder mut MountOperationBuilder,
}
impl<'builder> MountOperationBinder<'builder> {
    pub fn username(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("username", val));
        value.connect_update(move |value| obj.set_property("username", value));
        self.builder
    }
    pub fn anonymous(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("anonymous", val));
        value.connect_update(move |value| obj.set_property("anonymous", value));
        self.builder
    }
    pub fn display<T: IsA<gdk::Display>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("display", val));
        value.connect_update(move |value| obj.set_property("display", value));
        self.builder
    }
    pub fn pim(&mut self, value: &(impl Prop<u32> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pim", val));
        value.connect_update(move |value| obj.set_property("pim", value));
        self.builder
    }
    pub fn password(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("password", val));
        value.connect_update(move |value| obj.set_property("password", value));
        self.builder
    }
    pub fn parent<T: IsA<Window>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("parent", val));
        value.connect_update(move |value| obj.set_property("parent", value));
        self.builder
    }
    pub fn choice(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("choice", val));
        value.connect_update(move |value| obj.set_property("choice", value));
        self.builder
    }
    pub fn domain(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("domain", val));
        value.connect_update(move |value| obj.set_property("domain", value));
        self.builder
    }
    pub fn is_tcrypt_hidden_volume(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_tcrypt_hidden_volume", val));
        value.connect_update(move |value| obj.set_property("is_tcrypt_hidden_volume", value));
        self.builder
    }
    pub fn password_save(
        &mut self,
        value: &(impl Prop<gio::PasswordSave> + ?Sized),
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("password_save", val));
        value.connect_update(move |value| obj.set_property("password_save", value));
        self.builder
    }
    pub fn is_tcrypt_system_volume(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("is_tcrypt_system_volume", val));
        value.connect_update(move |value| obj.set_property("is_tcrypt_system_volume", value));
        self.builder
    }
}
pub struct MountOperationSignals<'builder> {
    builder: &'builder mut MountOperationBuilder,
}
impl<'builder> MountOperationSignals<'builder> {
    pub fn display_notify(
        &mut self,
        f: impl Fn(&MountOperation) + 'static,
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_display_notify(f);
        &mut self.builder
    }
    pub fn is_showing_notify(
        &mut self,
        f: impl Fn(&MountOperation) + 'static,
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_is_showing_notify(f);
        &mut self.builder
    }
    pub fn parent_notify(
        &mut self,
        f: impl Fn(&MountOperation) + 'static,
    ) -> &mut MountOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_parent_notify(f);
        &mut self.builder
    }
}
impl ForteExt for MountOperation {
    type Builder = MountOperationBuilder;
}
#[macro_export]
macro_rules ! MountOperation { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: MountOperation :: forte ()) $ ($ rest) * } } }
