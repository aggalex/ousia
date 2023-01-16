#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Application, *};
use gtkrs::{ApplicationInhibitFlags, Window};
#[derive(Default)]
pub struct ApplicationBuilder {
    builder: gtkrs::builders::ApplicationBuilder,
    on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::Application) + 'static>>,
    object: Option<gtkrs::Application>,
}
impl ApplicationBuilder {
    pub fn menubar(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).menubar(value);
        self
    }
    pub fn flags(&mut self, value: gio::ApplicationFlags) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).flags(value);
        self
    }
    pub fn resource_base_path(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).resource_base_path(value);
        self
    }
    pub fn inactivity_timeout(&mut self, value: u32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).inactivity_timeout(value);
        self
    }
    pub fn application_id(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).application_id(value);
        self
    }
    pub fn register_session(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).register_session(value);
        self
    }
    pub fn action_group(&mut self, value: &impl IsA<gio::ActionGroup>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).action_group(value);
        self
    }
    pub fn bind(&mut self) -> ApplicationBinder {
        ApplicationBinder { builder: self }
    }
    pub fn connect(&mut self) -> ApplicationSignals {
        ApplicationSignals { builder: self }
    }
    pub fn on_build(&mut self, f: impl FnOnce(&gtkrs::Application) + 'static) -> &mut Self {
        self.on_build.push(std::boxed::Box::new(f));
        self
    }
}
impl crate::prelude::Builder for ApplicationBuilder {
    type Target = Application;
    fn build(&mut self, func: impl Fn(Self::Target)) {
        func(self.create());
    }
    fn create(&mut self) -> Self::Target {
        let obj = std::mem::take(&mut self.builder).build();
        std::mem::take(&mut self.on_build)
            .into_iter()
            .for_each(|f| f(&obj));
        obj
    }
}
pub struct ApplicationBinder<'builder> {
    builder: &'builder mut ApplicationBuilder,
}
impl<'builder> ApplicationBinder<'builder> {
    pub fn menubar<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).menubar(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("menubar", value));
        });
        self.builder
    }
    pub fn flags(
        &mut self,
        value: &(impl Prop<gio::ApplicationFlags> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).flags(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("flags", value));
        });
        self.builder
    }
    pub fn resource_base_path(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).resource_base_path(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("resource_base_path", value));
        });
        self.builder
    }
    pub fn inactivity_timeout(
        &mut self,
        value: &(impl Prop<u32> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).inactivity_timeout(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("inactivity_timeout", value));
        });
        self.builder
    }
    pub fn application_id(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).application_id(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("application_id", value));
        });
        self.builder
    }
    pub fn register_session(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).register_session(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("register_session", value));
        });
        self.builder
    }
    pub fn action_group<T: IsA<gio::ActionGroup>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).action_group(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("action_group", value));
        });
        self.builder
    }
}
pub struct ApplicationSignals<'builder> {
    builder: &'builder mut ApplicationBuilder,
}
impl<'builder> ApplicationSignals<'builder> {
    pub fn query_end(&mut self, f: impl Fn(&Application) + 'static) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_query_end(f);
        });
        &mut self.builder
    }
    pub fn active_window_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_active_window_notify(f);
        });
        &mut self.builder
    }
    pub fn screensaver_active_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_screensaver_active_notify(f);
        });
        &mut self.builder
    }
    pub fn window_added(
        &mut self,
        f: impl Fn(&Application, &Window) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_window_added(f);
        });
        &mut self.builder
    }
    pub fn menubar_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_menubar_notify(f);
        });
        &mut self.builder
    }
    pub fn register_session_notify(
        &mut self,
        f: impl Fn(&Application) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_register_session_notify(f);
        });
        &mut self.builder
    }
    pub fn window_removed(
        &mut self,
        f: impl Fn(&Application, &Window) + 'static,
    ) -> &mut ApplicationBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_window_removed(f);
        });
        &mut self.builder
    }
}
impl ForteExt for Application {
    type Builder = ApplicationBuilder;
}
#[macro_export]
macro_rules ! Application { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Application :: forte ()) $ ($ rest) * } } }
