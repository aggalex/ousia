#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ApplicationWindow, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Application, Buildable, ConstraintTarget, LayoutManager,
    Native, Overflow, Root, ShortcutManager, ShortcutsWindow, Widget, Window,
};
#[derive(Default)]
pub struct ApplicationWindowBuilder {
    builder: gtkrs::builders::ApplicationWindowBuilder,
    on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::ApplicationWindow) + 'static>>,
    object: Option<gtkrs::ApplicationWindow>,
}
impl ApplicationWindowBuilder {
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_end(value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).width_request(value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).css_classes(value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focus_on_click(value);
        self
    }
    pub fn icon_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).icon_name(value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).has_tooltip(value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_bottom(value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand(value);
        self
    }
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).title(value);
        self
    }
    pub fn deletable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).deletable(value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focusable(value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_top(value);
        self
    }
    pub fn default_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).default_widget(value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand_set(value);
        self
    }
    pub fn application(&mut self, value: &impl IsA<Application>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).application(value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).css_name(value);
        self
    }
    pub fn default_width(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).default_width(value);
        self
    }
    pub fn focus_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focus_widget(value);
        self
    }
    pub fn show_menubar(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).show_menubar(value);
        self
    }
    pub fn hide_on_close(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hide_on_close(value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).child(value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).name(value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).opacity(value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).halign(value);
        self
    }
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).resizable(value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).overflow(value);
        self
    }
    pub fn display(&mut self, value: &impl IsA<gdk::Display>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).display(value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).cursor(value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_markup(value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).accessible_role(value);
        self
    }
    pub fn focus_visible(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focus_visible(value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).can_target(value);
        self
    }
    pub fn default_height(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).default_height(value);
        self
    }
    pub fn fullscreened(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).fullscreened(value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn titlebar(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).titlebar(value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand(value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).height_request(value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).layout_manager(value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).valign(value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand_set(value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_start(value);
        self
    }
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).modal(value);
        self
    }
    pub fn startup_id(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).startup_id(value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).receives_default(value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_text(value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).visible(value);
        self
    }
    pub fn maximized(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).maximized(value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).sensitive(value);
        self
    }
    pub fn destroy_with_parent(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).destroy_with_parent(value);
        self
    }
    pub fn transient_for(&mut self, value: &impl IsA<Window>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).transient_for(value);
        self
    }
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).handle_menubar_accel(value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).can_focus(value);
        self
    }
    pub fn decorated(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).decorated(value);
        self
    }
    pub fn mnemonics_visible(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).mnemonics_visible(value);
        self
    }
    pub fn bind(&mut self) -> ApplicationWindowBinder {
        ApplicationWindowBinder { builder: self }
    }
    pub fn connect(&mut self) -> ApplicationWindowSignals {
        ApplicationWindowSignals { builder: self }
    }
    pub fn on_build(&mut self, f: impl FnOnce(&gtkrs::ApplicationWindow) + 'static) -> &mut Self {
        self.on_build.push(std::boxed::Box::new(f));
        self
    }
}
impl crate::prelude::Builder for ApplicationWindowBuilder {
    type Target = ApplicationWindow;
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
pub struct ApplicationWindowBinder<'builder> {
    builder: &'builder mut ApplicationWindowBuilder,
}
impl<'builder> ApplicationWindowBinder<'builder> {
    pub fn margin_end(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).margin_end(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("margin_end", value));
        });
        self.builder
    }
    pub fn width_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).width_request(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("width_request", value));
        });
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).css_classes(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("css_classes", value));
        });
        self.builder
    }
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).focus_on_click(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("focus_on_click", value));
        });
        self.builder
    }
    pub fn icon_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).icon_name(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("icon_name", value));
        });
        self.builder
    }
    pub fn has_tooltip(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).has_tooltip(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("has_tooltip", value));
        });
        self.builder
    }
    pub fn margin_bottom(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).margin_bottom(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("margin_bottom", value));
        });
        self.builder
    }
    pub fn vexpand(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).vexpand(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("vexpand", value));
        });
        self.builder
    }
    pub fn title(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).title(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("title", value));
        });
        self.builder
    }
    pub fn deletable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).deletable(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("deletable", value));
        });
        self.builder
    }
    pub fn focusable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).focusable(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("focusable", value));
        });
        self.builder
    }
    pub fn margin_top(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).margin_top(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("margin_top", value));
        });
        self.builder
    }
    pub fn default_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).default_widget(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("default_widget", value));
        });
        self.builder
    }
    pub fn vexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).vexpand_set(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("vexpand_set", value));
        });
        self.builder
    }
    pub fn application<T: IsA<Application>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).application(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("application", value));
        });
        self.builder
    }
    pub fn css_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).css_name(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("css_name", value));
        });
        self.builder
    }
    pub fn default_width(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).default_width(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("default_width", value));
        });
        self.builder
    }
    pub fn focus_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).focus_widget(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("focus_widget", value));
        });
        self.builder
    }
    pub fn show_menubar(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).show_menubar(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("show_menubar", value));
        });
        self.builder
    }
    pub fn hide_on_close(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).hide_on_close(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("hide_on_close", value));
        });
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).child(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("child", value));
        });
        self.builder
    }
    pub fn name(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value
            .with(|val| self.builder.builder = std::mem::take(&mut self.builder.builder).name(val));
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("name", value));
        });
        self.builder
    }
    pub fn opacity(
        &mut self,
        value: &(impl Prop<f64> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).opacity(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("opacity", value));
        });
        self.builder
    }
    pub fn halign(
        &mut self,
        value: &(impl Prop<Align> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).halign(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("halign", value));
        });
        self.builder
    }
    pub fn resizable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).resizable(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("resizable", value));
        });
        self.builder
    }
    pub fn overflow(
        &mut self,
        value: &(impl Prop<Overflow> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).overflow(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("overflow", value));
        });
        self.builder
    }
    pub fn display<T: IsA<gdk::Display>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).display(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("display", value));
        });
        self.builder
    }
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).cursor(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("cursor", value));
        });
        self.builder
    }
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).tooltip_markup(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        });
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).accessible_role(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("accessible_role", value));
        });
        self.builder
    }
    pub fn focus_visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).focus_visible(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("focus_visible", value));
        });
        self.builder
    }
    pub fn can_target(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).can_target(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("can_target", value));
        });
        self.builder
    }
    pub fn default_height(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).default_height(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("default_height", value));
        });
        self.builder
    }
    pub fn fullscreened(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).fullscreened(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("fullscreened", value));
        });
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn titlebar<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).titlebar(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("titlebar", value));
        });
        self.builder
    }
    pub fn hexpand(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).hexpand(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("hexpand", value));
        });
        self.builder
    }
    pub fn height_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).height_request(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("height_request", value));
        });
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).layout_manager(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("layout_manager", value));
        });
        self.builder
    }
    pub fn valign(
        &mut self,
        value: &(impl Prop<Align> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).valign(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("valign", value));
        });
        self.builder
    }
    pub fn hexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).hexpand_set(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("hexpand_set", value));
        });
        self.builder
    }
    pub fn margin_start(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).margin_start(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("margin_start", value));
        });
        self.builder
    }
    pub fn modal(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).modal(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("modal", value));
        });
        self.builder
    }
    pub fn startup_id(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).startup_id(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("startup_id", value));
        });
        self.builder
    }
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).receives_default(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("receives_default", value));
        });
        self.builder
    }
    pub fn tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).tooltip_text(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("tooltip_text", value));
        });
        self.builder
    }
    pub fn visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).visible(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("visible", value));
        });
        self.builder
    }
    pub fn maximized(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).maximized(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("maximized", value));
        });
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).sensitive(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("sensitive", value));
        });
        self.builder
    }
    pub fn destroy_with_parent(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).destroy_with_parent(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("destroy_with_parent", value));
        });
        self.builder
    }
    pub fn transient_for<T: IsA<Window>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).transient_for(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("transient_for", value));
        });
        self.builder
    }
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).handle_menubar_accel(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("handle_menubar_accel", value));
        });
        self.builder
    }
    pub fn can_focus(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).can_focus(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("can_focus", value));
        });
        self.builder
    }
    pub fn decorated(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).decorated(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("decorated", value));
        });
        self.builder
    }
    pub fn mnemonics_visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ApplicationWindowBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).mnemonics_visible(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("mnemonics_visible", value));
        });
        self.builder
    }
}
pub struct ApplicationWindowSignals<'builder> {
    builder: &'builder mut ApplicationWindowBuilder,
}
impl<'builder> ApplicationWindowSignals<'builder> {
    pub fn show_menubar_notify(
        &mut self,
        f: impl Fn(&ApplicationWindow) + 'static,
    ) -> &mut ApplicationWindowBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_show_menubar_notify(f);
        });
        &mut self.builder
    }
}
impl ForteExt for ApplicationWindow {
    type Builder = ApplicationWindowBuilder;
}
#[macro_export]
macro_rules ! ApplicationWindow { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ApplicationWindow :: forte ()) $ ($ rest) * } } }
