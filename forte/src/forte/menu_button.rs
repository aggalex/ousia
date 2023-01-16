#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, MenuButton, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, ArrowType, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Popover, Widget,
};
#[derive(Clone)]
pub struct MenuButtonBuilder {
    obj: MenuButton,
}
impl Default for MenuButtonBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl MenuButtonBuilder {
    pub fn direction(&mut self, value: ArrowType) -> &mut Self {
        self.obj.set_property("direction", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn has_frame(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_frame", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn menu_model(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.obj.set_property("menu_model", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    pub fn always_show_arrow(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("always_show_arrow", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("label", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("child", value);
        self
    }
    pub fn use_underline(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("use_underline", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn icon_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("icon_name", value);
        self
    }
    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    pub fn primary(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("primary", value);
        self
    }
    pub fn popover(&mut self, value: &impl IsA<Popover>) -> &mut Self {
        self.obj.set_property("popover", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn bind(&mut self) -> MenuButtonBinder {
        MenuButtonBinder { builder: self }
    }
    pub fn connect(&mut self) -> MenuButtonSignals {
        MenuButtonSignals { builder: self }
    }
}
impl crate::prelude::Builder for MenuButtonBuilder {
    type Target = MenuButton;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for MenuButtonBuilder {
    type Target = MenuButton;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct MenuButtonBinder<'builder> {
    builder: &'builder mut MenuButtonBuilder,
}
impl<'builder> MenuButtonBinder<'builder> {
    pub fn direction(&mut self, value: &(impl Prop<ArrowType> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("direction", val));
        value.connect_update(move |value| obj.set_property("direction", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn has_frame(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_frame", val));
        value.connect_update(move |value| obj.set_property("has_frame", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn menu_model<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("menu_model", val));
        value.connect_update(move |value| obj.set_property("menu_model", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    pub fn always_show_arrow(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("always_show_arrow", val));
        value.connect_update(move |value| obj.set_property("always_show_arrow", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn label(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("label", val));
        value.connect_update(move |value| obj.set_property("label", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("child", val));
        value.connect_update(move |value| obj.set_property("child", value));
        self.builder
    }
    pub fn use_underline(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_underline", val));
        value.connect_update(move |value| obj.set_property("use_underline", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn icon_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("icon_name", val));
        value.connect_update(move |value| obj.set_property("icon_name", value));
        self.builder
    }
    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    pub fn primary(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("primary", val));
        value.connect_update(move |value| obj.set_property("primary", value));
        self.builder
    }
    pub fn popover<T: IsA<Popover>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("popover", val));
        value.connect_update(move |value| obj.set_property("popover", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut MenuButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
}
pub struct MenuButtonSignals<'builder> {
    builder: &'builder mut MenuButtonBuilder,
}
impl<'builder> MenuButtonSignals<'builder> {}
impl ForteExt for MenuButton {
    type Builder = MenuButtonBuilder;
}
#[macro_export]
macro_rules ! MenuButton { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: MenuButton :: forte ()) $ ($ rest) * } } }
