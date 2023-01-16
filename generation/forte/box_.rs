#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Box, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, BaselinePosition, Buildable, ConstraintTarget,
    LayoutManager, Orientable, Orientation, Overflow, Widget,
};
#[derive(Clone, Default)]
pub struct BoxBuilder {
    builder: gtkrs::BoxBuilder,
    on_build: Vec<Box<dyn FnOnce(&gtkrs::Box) + 'static>>,
    object: Option<gtkrs::Box>,
}
impl BoxBuilder {
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.builder.layout_manager(value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.builder.css_classes(value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.builder.name(value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.builder.opacity(value);
        self
    }
    pub fn baseline_position(&mut self, value: BaselinePosition) -> &mut Self {
        self.builder.baseline_position(value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.builder.sensitive(value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.builder.accessible_role(value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.builder.halign(value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder.hexpand_set(value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.builder.tooltip_text(value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.builder.can_target(value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.builder.focusable(value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.builder.margin_start(value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.builder.overflow(value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.builder.margin_end(value);
        self
    }
    pub fn spacing(&mut self, value: i32) -> &mut Self {
        self.builder.spacing(value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.builder.css_name(value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.builder.margin_bottom(value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.builder.margin_top(value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder.vexpand_set(value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.builder.width_request(value);
        self
    }
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.builder.orientation(value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.builder.can_focus(value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.builder.height_request(value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.builder.tooltip_markup(value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.builder.hexpand(value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.builder.receives_default(value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.builder.cursor(value);
        self
    }
    pub fn homogeneous(&mut self, value: bool) -> &mut Self {
        self.builder.homogeneous(value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.builder.visible(value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.builder.vexpand(value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.builder.valign(value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.builder.has_tooltip(value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.builder.focus_on_click(value);
        self
    }
    pub fn bind(&mut self) -> BoxBinder {
        BoxBinder { builder: self }
    }
    pub fn connect(&mut self) -> BoxSignals {
        BoxSignals { builder: self }
    }
}
impl crate::prelude::Builder for BoxBuilder {
    type Target = Box;
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
impl std::ops::Deref for BoxBuilder {
    type Target = Box;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct BoxBinder<'builder> {
    builder: &'builder mut BoxBuilder,
}
impl<'builder> BoxBinder<'builder> {
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn css_classes(&mut self, value: &(impl Prop<Vec<String>> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn baseline_position(
        &mut self,
        value: &(impl Prop<BaselinePosition> + ?Sized),
    ) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("baseline_position", val));
        value.connect_update(move |value| obj.set_property("baseline_position", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("spacing", val));
        value.connect_update(move |value| obj.set_property("spacing", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn orientation(&mut self, value: &(impl Prop<Orientation> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn homogeneous(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("homogeneous", val));
        value.connect_update(move |value| obj.set_property("homogeneous", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
}
pub struct BoxSignals<'builder> {
    builder: &'builder mut BoxBuilder,
}
impl<'builder> BoxSignals<'builder> {
    pub fn homogeneous_notify(&mut self, f: impl Fn(&Box) + 'static) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_homogeneous_notify(f);
        &mut self.builder
    }
    pub fn spacing_notify(&mut self, f: impl Fn(&Box) + 'static) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_spacing_notify(f);
        &mut self.builder
    }
    pub fn baseline_position_notify(&mut self, f: impl Fn(&Box) + 'static) -> &mut BoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_baseline_position_notify(f);
        &mut self.builder
    }
}
impl ForteExt for Box {
    type Builder = BoxBuilder;
}
#[macro_export]
macro_rules ! Box { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Box :: forte ()) $ ($ rest) * } } }
