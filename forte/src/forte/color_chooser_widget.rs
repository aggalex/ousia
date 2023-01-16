#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ColorChooserWidget, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, ColorChooser, ConstraintTarget, LayoutManager,
    Overflow, Widget,
};
#[derive(Clone)]
pub struct ColorChooserWidgetBuilder {
    obj: ColorChooserWidget,
}
impl Default for ColorChooserWidgetBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ColorChooserWidgetBuilder {
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn show_editor(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_editor", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn use_alpha(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("use_alpha", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn rgba(&mut self, value: &gdk::RGBA) -> &mut Self {
        self.obj.set_property("rgba", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn bind(&mut self) -> ColorChooserWidgetBinder {
        ColorChooserWidgetBinder { builder: self }
    }
    pub fn connect(&mut self) -> ColorChooserWidgetSignals {
        ColorChooserWidgetSignals { builder: self }
    }
}
impl crate::prelude::Builder for ColorChooserWidgetBuilder {
    type Target = ColorChooserWidget;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ColorChooserWidgetBuilder {
    type Target = ColorChooserWidget;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ColorChooserWidgetBinder<'builder> {
    builder: &'builder mut ColorChooserWidgetBuilder,
}
impl<'builder> ColorChooserWidgetBinder<'builder> {
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn margin_end(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn margin_bottom(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn vexpand(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn show_editor(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_editor", val));
        value.connect_update(move |value| obj.set_property("show_editor", value));
        self.builder
    }
    pub fn margin_start(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn can_target(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn use_alpha(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_alpha", val));
        value.connect_update(move |value| obj.set_property("use_alpha", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn vexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn has_tooltip(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn css_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn margin_top(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn hexpand_set(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn halign(
        &mut self,
        value: &(impl Prop<Align> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn can_focus(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn tooltip_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn width_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn valign(
        &mut self,
        value: &(impl Prop<Align> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn hexpand(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn focusable(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn overflow(
        &mut self,
        value: &(impl Prop<Overflow> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn sensitive(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn rgba(
        &mut self,
        value: &(impl Prop<gdk::RGBA> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("rgba", val));
        value.connect_update(move |value| obj.set_property("rgba", value));
        self.builder
    }
    pub fn height_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ColorChooserWidgetBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
}
pub struct ColorChooserWidgetSignals<'builder> {
    builder: &'builder mut ColorChooserWidgetBuilder,
}
impl<'builder> ColorChooserWidgetSignals<'builder> {}
impl ForteExt for ColorChooserWidget {
    type Builder = ColorChooserWidgetBuilder;
}
#[macro_export]
macro_rules ! ColorChooserWidget { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ColorChooserWidget :: forte ()) $ ($ rest) * } } }
