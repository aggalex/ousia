#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, PrintOperation, *};
use gtkrs::{
    PageSetup, PrintContext, PrintOperationAction, PrintOperationPreview, PrintOperationResult,
    PrintSettings, PrintStatus, Unit, Widget, Window,
};
#[derive(Clone)]
pub struct PrintOperationBuilder {
    obj: PrintOperation,
}
impl Default for PrintOperationBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl PrintOperationBuilder {
    pub fn print_settings(&mut self, value: &PrintSettings) -> &mut Self {
        self.obj.set_property("print_settings", value);
        self
    }
    pub fn use_full_page(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("use_full_page", value);
        self
    }
    pub fn show_progress(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_progress", value);
        self
    }
    pub fn default_page_setup(&mut self, value: &PageSetup) -> &mut Self {
        self.obj.set_property("default_page_setup", value);
        self
    }
    pub fn has_selection(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_selection", value);
        self
    }
    pub fn support_selection(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("support_selection", value);
        self
    }
    pub fn allow_async(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("allow_async", value);
        self
    }
    pub fn track_print_status(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("track_print_status", value);
        self
    }
    pub fn unit(&mut self, value: Unit) -> &mut Self {
        self.obj.set_property("unit", value);
        self
    }
    pub fn job_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("job_name", value);
        self
    }
    pub fn n_pages(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("n_pages", value);
        self
    }
    pub fn embed_page_setup(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("embed_page_setup", value);
        self
    }
    pub fn custom_tab_label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("custom_tab_label", value);
        self
    }
    pub fn current_page(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("current_page", value);
        self
    }
    pub fn export_filename(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("export_filename", value);
        self
    }
    pub fn bind(&mut self) -> PrintOperationBinder {
        PrintOperationBinder { builder: self }
    }
    pub fn connect(&mut self) -> PrintOperationSignals {
        PrintOperationSignals { builder: self }
    }
}
impl crate::prelude::Builder for PrintOperationBuilder {
    type Target = PrintOperation;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for PrintOperationBuilder {
    type Target = PrintOperation;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct PrintOperationBinder<'builder> {
    builder: &'builder mut PrintOperationBuilder,
}
impl<'builder> PrintOperationBinder<'builder> {
    pub fn print_settings(
        &mut self,
        value: &(impl Prop<PrintSettings> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("print_settings", val));
        value.connect_update(move |value| obj.set_property("print_settings", value));
        self.builder
    }
    pub fn use_full_page(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_full_page", val));
        value.connect_update(move |value| obj.set_property("use_full_page", value));
        self.builder
    }
    pub fn show_progress(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_progress", val));
        value.connect_update(move |value| obj.set_property("show_progress", value));
        self.builder
    }
    pub fn default_page_setup(
        &mut self,
        value: &(impl Prop<PageSetup> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_page_setup", val));
        value.connect_update(move |value| obj.set_property("default_page_setup", value));
        self.builder
    }
    pub fn has_selection(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_selection", val));
        value.connect_update(move |value| obj.set_property("has_selection", value));
        self.builder
    }
    pub fn support_selection(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("support_selection", val));
        value.connect_update(move |value| obj.set_property("support_selection", value));
        self.builder
    }
    pub fn allow_async(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("allow_async", val));
        value.connect_update(move |value| obj.set_property("allow_async", value));
        self.builder
    }
    pub fn track_print_status(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("track_print_status", val));
        value.connect_update(move |value| obj.set_property("track_print_status", value));
        self.builder
    }
    pub fn unit(&mut self, value: &(impl Prop<Unit> + ?Sized)) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("unit", val));
        value.connect_update(move |value| obj.set_property("unit", value));
        self.builder
    }
    pub fn job_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("job_name", val));
        value.connect_update(move |value| obj.set_property("job_name", value));
        self.builder
    }
    pub fn n_pages(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("n_pages", val));
        value.connect_update(move |value| obj.set_property("n_pages", value));
        self.builder
    }
    pub fn embed_page_setup(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("embed_page_setup", val));
        value.connect_update(move |value| obj.set_property("embed_page_setup", value));
        self.builder
    }
    pub fn custom_tab_label(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("custom_tab_label", val));
        value.connect_update(move |value| obj.set_property("custom_tab_label", value));
        self.builder
    }
    pub fn current_page(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("current_page", val));
        value.connect_update(move |value| obj.set_property("current_page", value));
        self.builder
    }
    pub fn export_filename(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("export_filename", val));
        value.connect_update(move |value| obj.set_property("export_filename", value));
        self.builder
    }
}
pub struct PrintOperationSignals<'builder> {
    builder: &'builder mut PrintOperationBuilder,
}
impl<'builder> PrintOperationSignals<'builder> {
    pub fn n_pages_to_print_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_n_pages_to_print_notify(f);
        &mut self.builder
    }
    pub fn has_selection_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_selection_notify(f);
        &mut self.builder
    }
    pub fn draw_page(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintContext, i32) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_draw_page(f);
        &mut self.builder
    }
    pub fn export_filename_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_export_filename_notify(f);
        &mut self.builder
    }
    pub fn begin_print(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintContext) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_begin_print(f);
        &mut self.builder
    }
    pub fn request_page_setup(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintContext, i32, &PageSetup) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_request_page_setup(f);
        &mut self.builder
    }
    pub fn unit_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_unit_notify(f);
        &mut self.builder
    }
    pub fn embed_page_setup_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_embed_page_setup_notify(f);
        &mut self.builder
    }
    pub fn custom_widget_apply(
        &mut self,
        f: impl Fn(&PrintOperation, &Widget) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_custom_widget_apply(f);
        &mut self.builder
    }
    pub fn n_pages_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_n_pages_notify(f);
        &mut self.builder
    }
    pub fn use_full_page_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_use_full_page_notify(f);
        &mut self.builder
    }
    pub fn print_settings_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_print_settings_notify(f);
        &mut self.builder
    }
    pub fn allow_async_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_allow_async_notify(f);
        &mut self.builder
    }
    pub fn current_page_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_current_page_notify(f);
        &mut self.builder
    }
    pub fn support_selection_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_support_selection_notify(f);
        &mut self.builder
    }
    pub fn job_name_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_job_name_notify(f);
        &mut self.builder
    }
    pub fn track_print_status_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_track_print_status_notify(f);
        &mut self.builder
    }
    pub fn paginate(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintContext) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paginate(f);
        &mut self.builder
    }
    pub fn done(
        &mut self,
        f: impl Fn(&PrintOperation, PrintOperationResult) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_done(f);
        &mut self.builder
    }
    pub fn end_print(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintContext) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_end_print(f);
        &mut self.builder
    }
    pub fn create_custom_widget(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_create_custom_widget(f);
        &mut self.builder
    }
    pub fn update_custom_widget(
        &mut self,
        f: impl Fn(&PrintOperation, &Widget, &PageSetup, &PrintSettings) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_update_custom_widget(f);
        &mut self.builder
    }
    pub fn default_page_setup_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_default_page_setup_notify(f);
        &mut self.builder
    }
    pub fn status_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_status_notify(f);
        &mut self.builder
    }
    pub fn preview(
        &mut self,
        f: impl Fn(&PrintOperation, &PrintOperationPreview, &PrintContext, Option<&Window>) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_preview(f);
        &mut self.builder
    }
    pub fn status_string_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_status_string_notify(f);
        &mut self.builder
    }
    pub fn status_changed(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_status_changed(f);
        &mut self.builder
    }
    pub fn show_progress_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_show_progress_notify(f);
        &mut self.builder
    }
    pub fn custom_tab_label_notify(
        &mut self,
        f: impl Fn(&PrintOperation) + 'static,
    ) -> &mut PrintOperationBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_custom_tab_label_notify(f);
        &mut self.builder
    }
}
impl ForteExt for PrintOperation {
    type Builder = PrintOperationBuilder;
}
#[macro_export]
macro_rules ! PrintOperation { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: PrintOperation :: forte ()) $ ($ rest) * } } }
