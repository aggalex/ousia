#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, MessageDialog, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Application, Buildable, ButtonsType, ConstraintTarget,
    Dialog, LayoutManager, MessageType, Native, Overflow, Root, ShortcutManager, Widget, Window,
};
#[derive(Clone)]
pub struct MessageDialogBuilder {
    obj: MessageDialog,
}
impl Default for MessageDialogBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl MessageDialogBuilder {
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn buttons(&mut self, value: ButtonsType) -> &mut Self {
        self.obj.set_property("buttons", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn focus_visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_visible", value);
        self
    }
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("handle_menubar_accel", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn default_width(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("default_width", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("resizable", value);
        self
    }
    pub fn default_height(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("default_height", value);
        self
    }
    pub fn transient_for(&mut self, value: &impl IsA<Window>) -> &mut Self {
        self.obj.set_property("transient_for", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn startup_id(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("startup_id", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn hide_on_close(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hide_on_close", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn secondary_use_markup(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("secondary_use_markup", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn use_markup(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("use_markup", value);
        self
    }
    pub fn default_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("default_widget", value);
        self
    }
    pub fn secondary_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("secondary_text", value);
        self
    }
    pub fn icon_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("icon_name", value);
        self
    }
    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("modal", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn title(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("title", value);
        self
    }
    pub fn message_type(&mut self, value: MessageType) -> &mut Self {
        self.obj.set_property("message_type", value);
        self
    }
    pub fn destroy_with_parent(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("destroy_with_parent", value);
        self
    }
    pub fn maximized(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("maximized", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn display(&mut self, value: &impl IsA<gdk::Display>) -> &mut Self {
        self.obj.set_property("display", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn decorated(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("decorated", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn use_header_bar(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("use_header_bar", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn titlebar(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("titlebar", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn fullscreened(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("fullscreened", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("child", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn deletable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("deletable", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn focus_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("focus_widget", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn application(&mut self, value: &impl IsA<Application>) -> &mut Self {
        self.obj.set_property("application", value);
        self
    }
    pub fn mnemonics_visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("mnemonics_visible", value);
        self
    }
    pub fn bind(&mut self) -> MessageDialogBinder {
        MessageDialogBinder { builder: self }
    }
    pub fn connect(&mut self) -> MessageDialogSignals {
        MessageDialogSignals { builder: self }
    }
}
impl crate::prelude::Builder for MessageDialogBuilder {
    type Target = MessageDialog;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for MessageDialogBuilder {
    type Target = MessageDialog;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct MessageDialogBinder<'builder> {
    builder: &'builder mut MessageDialogBuilder,
}
impl<'builder> MessageDialogBinder<'builder> {
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn buttons(
        &mut self,
        value: &(impl Prop<ButtonsType> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("buttons", val));
        value.connect_update(move |value| obj.set_property("buttons", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn focus_visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_visible", val));
        value.connect_update(move |value| obj.set_property("focus_visible", value));
        self.builder
    }
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("handle_menubar_accel", val));
        value.connect_update(move |value| obj.set_property("handle_menubar_accel", value));
        self.builder
    }
    pub fn width_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn overflow(
        &mut self,
        value: &(impl Prop<Overflow> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn default_width(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_width", val));
        value.connect_update(move |value| obj.set_property("default_width", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn resizable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("resizable", val));
        value.connect_update(move |value| obj.set_property("resizable", value));
        self.builder
    }
    pub fn default_height(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_height", val));
        value.connect_update(move |value| obj.set_property("default_height", value));
        self.builder
    }
    pub fn transient_for<T: IsA<Window>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("transient_for", val));
        value.connect_update(move |value| obj.set_property("transient_for", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn startup_id(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("startup_id", val));
        value.connect_update(move |value| obj.set_property("startup_id", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn hide_on_close(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hide_on_close", val));
        value.connect_update(move |value| obj.set_property("hide_on_close", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn secondary_use_markup(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_use_markup", val));
        value.connect_update(move |value| obj.set_property("secondary_use_markup", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn use_markup(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_markup", val));
        value.connect_update(move |value| obj.set_property("use_markup", value));
        self.builder
    }
    pub fn default_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_widget", val));
        value.connect_update(move |value| obj.set_property("default_widget", value));
        self.builder
    }
    pub fn secondary_text(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("secondary_text", val));
        value.connect_update(move |value| obj.set_property("secondary_text", value));
        self.builder
    }
    pub fn icon_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("icon_name", val));
        value.connect_update(move |value| obj.set_property("icon_name", value));
        self.builder
    }
    pub fn modal(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("modal", val));
        value.connect_update(move |value| obj.set_property("modal", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn title(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("title", val));
        value.connect_update(move |value| obj.set_property("title", value));
        self.builder
    }
    pub fn message_type(
        &mut self,
        value: &(impl Prop<MessageType> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("message_type", val));
        value.connect_update(move |value| obj.set_property("message_type", value));
        self.builder
    }
    pub fn destroy_with_parent(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("destroy_with_parent", val));
        value.connect_update(move |value| obj.set_property("destroy_with_parent", value));
        self.builder
    }
    pub fn maximized(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("maximized", val));
        value.connect_update(move |value| obj.set_property("maximized", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn display<T: IsA<gdk::Display>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("display", val));
        value.connect_update(move |value| obj.set_property("display", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn decorated(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("decorated", val));
        value.connect_update(move |value| obj.set_property("decorated", value));
        self.builder
    }
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn use_header_bar(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_header_bar", val));
        value.connect_update(move |value| obj.set_property("use_header_bar", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn titlebar<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("titlebar", val));
        value.connect_update(move |value| obj.set_property("titlebar", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn margin_bottom(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn fullscreened(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fullscreened", val));
        value.connect_update(move |value| obj.set_property("fullscreened", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("child", val));
        value.connect_update(move |value| obj.set_property("child", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn deletable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("deletable", val));
        value.connect_update(move |value| obj.set_property("deletable", value));
        self.builder
    }
    pub fn height_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn focus_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_widget", val));
        value.connect_update(move |value| obj.set_property("focus_widget", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn application<T: IsA<Application>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("application", val));
        value.connect_update(move |value| obj.set_property("application", value));
        self.builder
    }
    pub fn mnemonics_visible(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut MessageDialogBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mnemonics_visible", val));
        value.connect_update(move |value| obj.set_property("mnemonics_visible", value));
        self.builder
    }
}
pub struct MessageDialogSignals<'builder> {
    builder: &'builder mut MessageDialogBuilder,
}
impl<'builder> MessageDialogSignals<'builder> {}
impl ForteExt for MessageDialog {
    type Builder = MessageDialogBuilder;
}
#[macro_export]
macro_rules ! MessageDialog { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: MessageDialog :: forte ()) $ ($ rest) * } } }
