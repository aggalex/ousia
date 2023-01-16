#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Button, *};
use gtkrs::{
    Accessible, AccessibleRole, Actionable, Align, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Widget,
};
#[derive(Default)]
pub struct ButtonBuilder {
    builder: gtkrs::builders::ButtonBuilder,
    on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::Button) + 'static>>,
    object: Option<gtkrs::Button>,
}
impl ButtonBuilder {
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).cursor(value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).receives_default(value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).valign(value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand_set(value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).halign(value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand(value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).visible(value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand(value);
        self
    }
    pub fn action_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).action_name(value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_top(value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).layout_manager(value);
        self
    }
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).label(value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).accessible_role(value);
        self
    }
    pub fn action_target(&mut self, value: &glib::Variant) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).action_target(value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_markup(value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focusable(value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand_set(value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).can_focus(value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_start(value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_bottom(value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).css_name(value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_text(value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_end(value);
        self
    }
    pub fn icon_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).icon_name(value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).css_classes(value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).height_request(value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).width_request(value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).name(value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).child(value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).has_tooltip(value);
        self
    }
    pub fn has_frame(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).has_frame(value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).can_target(value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).opacity(value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).sensitive(value);
        self
    }
    pub fn use_underline(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).use_underline(value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focus_on_click(value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).overflow(value);
        self
    }
    pub fn bind(&mut self) -> ButtonBinder {
        ButtonBinder { builder: self }
    }
    pub fn connect(&mut self) -> ButtonSignals {
        ButtonSignals { builder: self }
    }
    pub fn on_build(&mut self, f: impl FnOnce(&gtkrs::Button) + 'static) -> &mut Self {
        self.on_build.push(std::boxed::Box::new(f));
        self
    }
}
impl crate::prelude::Builder for ButtonBuilder {
    type Target = Button;
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
pub struct ButtonBinder<'builder> {
    builder: &'builder mut ButtonBuilder,
}
impl<'builder> ButtonBinder<'builder> {
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn vexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn action_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).action_name(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("action_name", value));
        });
        self.builder
    }
    pub fn margin_top(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn label(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut ButtonBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).label(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("label", value));
        });
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn action_target(
        &mut self,
        value: &(impl Prop<glib::Variant> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).action_target(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("action_target", value));
        });
        self.builder
    }
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn focusable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn hexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn can_focus(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn margin_start(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn margin_bottom(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn margin_end(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn icon_name(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn height_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn width_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut ButtonBuilder {
        value
            .with(|val| self.builder.builder = std::mem::take(&mut self.builder.builder).name(val));
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("name", value));
        });
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn has_tooltip(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn has_frame(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).has_frame(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("has_frame", value));
        });
        self.builder
    }
    pub fn can_target(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized + 'static)) -> &mut ButtonBuilder {
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
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn use_underline(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).use_underline(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("use_underline", value));
        });
        self.builder
    }
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
    pub fn overflow(
        &mut self,
        value: &(impl Prop<Overflow> + ?Sized + 'static),
    ) -> &mut ButtonBuilder {
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
}
pub struct ButtonSignals<'builder> {
    builder: &'builder mut ButtonBuilder,
}
impl<'builder> ButtonSignals<'builder> {
    pub fn clicked(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_clicked(f);
        });
        &mut self.builder
    }
    pub fn activate(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_activate(f);
        });
        &mut self.builder
    }
    pub fn child_notify(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_child_notify(f);
        });
        &mut self.builder
    }
    pub fn icon_name_notify(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_icon_name_notify(f);
        });
        &mut self.builder
    }
    pub fn label_notify(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_label_notify(f);
        });
        &mut self.builder
    }
    pub fn use_underline_notify(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_use_underline_notify(f);
        });
        &mut self.builder
    }
    pub fn has_frame_notify(&mut self, f: impl Fn(&Button) + 'static) -> &mut ButtonBuilder {
        self.builder.on_build(move |obj| {
            obj.connect_has_frame_notify(f);
        });
        &mut self.builder
    }
}
impl ForteExt for Button {
    type Builder = ButtonBuilder;
}
#[macro_export]
macro_rules ! Button { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Button :: forte ()) $ ($ rest) * } } }
