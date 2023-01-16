#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Application, *};
use gtkrs::{ApplicationInhibitFlags, Window};
#[derive(Clone, Default)]
pub struct ApplicationBuilder {
    builder: gtkrs::ApplicationBuilder,
    on_build: Vec<Box<dyn FnOnce(&gtkrs::Application) + 'static>>,
    object: Option<gtkrs::Application>,
}
impl ApplicationBuilder {
    pub fn register_session(&mut self, value: bool) -> &mut Self {
        self.builder.register_session(value);
        self
    }
    pub fn inactivity_timeout(&mut self, value: u32) -> &mut Self {
        self.builder.inactivity_timeout(value);
        self
    }
    pub fn menubar(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.builder.menubar(value);
        self
    }
    pub fn action_group(&mut self, value: &impl IsA<gio::ActionGroup>) -> &mut Self {
        self.builder.action_group(value);
        self
    }
    pub fn application_id(&mut self, value: &str) -> &mut Self {
        self.builder.application_id(value);
        self
    }
    pub fn flags(&mut self, value: gio::ApplicationFlags) -> &mut Self {
        self.builder.flags(value);
        self
    }
    pub fn resource_base_path(&mut self, value: &str) -> &mut Self {
        self.builder.resource_base_path(value);
        self
    }
    pub fn bind(&mut self) -> ApplicationBinder {
        ApplicationBinder { builder: self }
    }
    pub fn connect(&mut self) -> ApplicationSignals {
        ApplicationSignals { builder: self }
    }
}
impl crate::prelude::Builder for ApplicationBuilder {
    type Target = Application;
    fn build(&mut self, func: impl Fn(Self::Target)) -> &mut Self {
        func(self.create());
        self
    }
    fn create(&mut self) -> Self::Target {
        self.obj = self.obj.or_else(|| {
            let obj = self.builder.build();
            self.on_build.iter().for_each(|f| f(&obj));
            Some(obj)
        });
        self.obj.unwrap().clone()
    }
}
impl std::ops::Deref for ApplicationBuilder {
    type Target = Application;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ApplicationBinder<'builder> {
    builder: &'builder mut ApplicationBuilder,
}
impl<'builder> ApplicationBinder<'builder> {
    pub fn register_session(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("register_session", val));
        value.connect_update(move |value| obj.set_property("register_session", value));
        self.builder
    }
    pub fn inactivity_timeout(
        &mut self,
        value: &(impl Prop<u32> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inactivity_timeout", val));
        value.connect_update(move |value| obj.set_property("inactivity_timeout", value));
        self.builder
    }
    pub fn menubar<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("menubar", val));
        value.connect_update(move |value| obj.set_property("menubar", value));
        self.builder
    }
    pub fn action_group<T: IsA<gio::ActionGroup>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action_group", val));
        value.connect_update(move |value| obj.set_property("action_group", value));
        self.builder
    }
    pub fn application_id(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("application_id", val));
        value.connect_update(move |value| obj.set_property("application_id", value));
        self.builder
    }
    pub fn flags(
        &mut self,
        value: &(impl Prop<gio::ApplicationFlags> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("flags", val));
        value.connect_update(move |value| obj.set_property("flags", value));
        self.builder
    }
    pub fn resource_base_path(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("resource_base_path", val));
        value.connect_update(move |value| obj.set_property("resource_base_path", value));
        self.builder
    }
}
pub struct ApplicationSignals<'builder> {
    builder: &'builder mut ApplicationBuilder,
}
impl<'builder> ApplicationSignals<'builder> {
    pub fn window_added(
        &mut self,
        f: impl Fn(&Application, &Window) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_window_added(f);
        &mut self.builder
    }
    pub fn menubar_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_menubar_notify(f);
        &mut self.builder
    }
    pub fn screensaver_active_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_screensaver_active_notify(f);
        &mut self.builder
    }
    pub fn register_session_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_register_session_notify(f);
        &mut self.builder
    }
    pub fn query_end(&mut self, f: impl Fn(&Application) + 'static) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_query_end(f);
        &mut self.builder
    }
    pub fn active_window_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_active_window_notify(f);
        &mut self.builder
    }
    pub fn window_removed(
        &mut self,
        f: impl Fn(&Application, &Window) + 'static,
    ) -> &mut ApplicationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_window_removed(f);
        &mut self.builder
    }
}
impl ForteExt for Application {
    type Builder = ApplicationBuilder;
}
#[macro_export]
macro_rules ! Application { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Application :: forte ()) $ ($ rest) * } } }
