#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use gtkrs::NaturalWrapMode;
use gtkrs::{prelude::*, Label, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Justification, LayoutManager,
    MovementStep, Overflow, Widget,
};
#[derive(Default)]
pub struct LabelBuilder {
    builder: gtkrs::builders::LabelBuilder,
    on_build: Vec<std::boxed::Box<dyn FnOnce(&gtkrs::Label) + 'static>>,
    object: Option<gtkrs::Label>,
}
impl LabelBuilder {
    pub fn mnemonic_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).mnemonic_widget(value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand_set(value);
        self
    }
    pub fn justify(&mut self, value: Justification) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).justify(value);
        self
    }
    pub fn max_width_chars(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).max_width_chars(value);
        self
    }
    pub fn ellipsize(&mut self, value: pango::EllipsizeMode) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).ellipsize(value);
        self
    }
    pub fn width_chars(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).width_chars(value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).accessible_role(value);
        self
    }
    pub fn extra_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).extra_menu(value);
        self
    }
    pub fn use_markup(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).use_markup(value);
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
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).receives_default(value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).valign(value);
        self
    }
    pub fn attributes(&mut self, value: &pango::AttrList) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).attributes(value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_top(value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn natural_wrap_mode(&mut self, value: NaturalWrapMode) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).natural_wrap_mode(value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).visible(value);
        self
    }
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tabs(value);
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
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand_set(value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).sensitive(value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).has_tooltip(value);
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
    pub fn wrap_mode(&mut self, value: pango::WrapMode) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).wrap_mode(value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).width_request(value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_text(value);
        self
    }
    pub fn selectable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).selectable(value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).can_focus(value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).vexpand(value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_end(value);
        self
    }
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).label(value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).tooltip_markup(value);
        self
    }
    pub fn single_line_mode(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).single_line_mode(value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).xalign(value);
        self
    }
    pub fn lines(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).lines(value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).name(value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).focusable(value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_start(value);
        self
    }
    pub fn wrap(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).wrap(value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).css_name(value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).yalign(value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).margin_bottom(value);
        self
    }
    pub fn use_underline(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).use_underline(value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).cursor(value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).hexpand(value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).layout_manager(value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.builder = std::mem::take(&mut self.builder).halign(value);
        self
    }
    pub fn bind(&mut self) -> LabelBinder {
        LabelBinder { builder: self }
    }
    pub fn connect(&mut self) -> LabelSignals {
        LabelSignals { builder: self }
    }
    pub fn on_build(&mut self, f: impl FnOnce(&gtkrs::Label) + 'static) -> &mut Self {
        self.on_build.push(std::boxed::Box::new(f));
        self
    }
}
impl crate::prelude::Builder for LabelBuilder {
    type Target = Label;
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
pub struct LabelBinder<'builder> {
    builder: &'builder mut LabelBuilder,
}
impl<'builder> LabelBinder<'builder> {
    pub fn mnemonic_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).mnemonic_widget(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("mnemonic_widget", value));
        });
        self.builder
    }
    pub fn vexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn justify(
        &mut self,
        value: &(impl Prop<Justification> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).justify(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("justify", value));
        });
        self.builder
    }
    pub fn max_width_chars(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).max_width_chars(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("max_width_chars", value));
        });
        self.builder
    }
    pub fn ellipsize(
        &mut self,
        value: &(impl Prop<pango::EllipsizeMode> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).ellipsize(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("ellipsize", value));
        });
        self.builder
    }
    pub fn width_chars(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).width_chars(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("width_chars", value));
        });
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn extra_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).extra_menu(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("extra_menu", value));
        });
        self.builder
    }
    pub fn use_markup(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).use_markup(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("use_markup", value));
        });
        self.builder
    }
    pub fn can_target(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn attributes(
        &mut self,
        value: &(impl Prop<pango::AttrList> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).attributes(val)
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("attributes", value));
        });
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn natural_wrap_mode(
        &mut self,
        value: &(impl Prop<NaturalWrapMode> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).natural_wrap_mode(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("natural_wrap_mode", value));
        });
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn tabs(
        &mut self,
        value: &(impl Prop<pango::TabArray> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value
            .with(|val| self.builder.builder = std::mem::take(&mut self.builder.builder).tabs(val));
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("tabs", value));
        });
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    ) -> &mut LabelBuilder {
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
    pub fn hexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn has_tooltip(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    ) -> &mut LabelBuilder {
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
    pub fn wrap_mode(
        &mut self,
        value: &(impl Prop<pango::WrapMode> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).wrap_mode(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("wrap_mode", value));
        });
        self.builder
    }
    pub fn width_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn selectable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).selectable(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("selectable", value));
        });
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn label(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn single_line_mode(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder =
                std::mem::take(&mut self.builder.builder).single_line_mode(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("single_line_mode", value));
        });
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized + 'static)) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).xalign(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("xalign", value));
        });
        self.builder
    }
    pub fn lines(&mut self, value: &(impl Prop<i32> + ?Sized + 'static)) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).lines(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("lines", value));
        });
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut LabelBuilder {
        value
            .with(|val| self.builder.builder = std::mem::take(&mut self.builder.builder).name(val));
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("name", value));
        });
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn margin_start(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn wrap(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).wrap(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("wrap", value));
        });
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn yalign(&mut self, value: &(impl Prop<f32> + ?Sized + 'static)) -> &mut LabelBuilder {
        value.with(|val| {
            self.builder.builder = std::mem::take(&mut self.builder.builder).yalign(val.clone())
        });
        let value = value.clone();
        self.builder.on_build(move |obj| {
            let obj = obj.clone();
            value.connect_update(move |value| obj.set_property("yalign", value));
        });
        self.builder
    }
    pub fn margin_bottom(
        &mut self,
        value: &(impl Prop<i32> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn use_underline(
        &mut self,
        value: &(impl Prop<bool> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized + 'static)) -> &mut LabelBuilder {
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
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized + 'static),
    ) -> &mut LabelBuilder {
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
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized + 'static)) -> &mut LabelBuilder {
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
}
pub struct LabelSignals<'builder> {
    builder: &'builder mut LabelBuilder,
}
impl<'builder> LabelSignals<'builder> {}
impl ForteExt for Label {
    type Builder = LabelBuilder;
}
#[macro_export]
macro_rules ! Label { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Label :: forte ()) $ ($ rest) * } } }
