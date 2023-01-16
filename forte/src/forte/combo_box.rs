#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ComboBox, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, CellEditable, CellLayout, ConstraintTarget,
    LayoutManager, Overflow, ScrollType, SensitivityType, TreeIter, TreeModel, Widget,
};
#[derive(Clone)]
pub struct ComboBoxBuilder {
    obj: ComboBox,
}
impl Default for ComboBoxBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ComboBoxBuilder {
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn entry_text_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("entry_text_column", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn has_frame(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_frame", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("child", value);
        self
    }
    pub fn active(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("active", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn active_id(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("active_id", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn editing_canceled(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("editing_canceled", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<TreeModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn has_entry(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_entry", value);
        self
    }
    pub fn id_column(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("id_column", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn popup_fixed_width(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("popup_fixed_width", value);
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
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn button_sensitivity(&mut self, value: SensitivityType) -> &mut Self {
        self.obj.set_property("button_sensitivity", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn bind(&mut self) -> ComboBoxBinder {
        ComboBoxBinder { builder: self }
    }
    pub fn connect(&mut self) -> ComboBoxSignals {
        ComboBoxSignals { builder: self }
    }
}
impl crate::prelude::Builder for ComboBoxBuilder {
    type Target = ComboBox;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ComboBoxBuilder {
    type Target = ComboBox;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ComboBoxBinder<'builder> {
    builder: &'builder mut ComboBoxBuilder,
}
impl<'builder> ComboBoxBinder<'builder> {
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn entry_text_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("entry_text_column", val));
        value.connect_update(move |value| obj.set_property("entry_text_column", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn has_frame(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_frame", val));
        value.connect_update(move |value| obj.set_property("has_frame", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("child", val));
        value.connect_update(move |value| obj.set_property("child", value));
        self.builder
    }
    pub fn active(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("active", val));
        value.connect_update(move |value| obj.set_property("active", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn active_id(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("active_id", val));
        value.connect_update(move |value| obj.set_property("active_id", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn editing_canceled(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("editing_canceled", val));
        value.connect_update(move |value| obj.set_property("editing_canceled", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn model<T: IsA<TreeModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn has_entry(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_entry", val));
        value.connect_update(move |value| obj.set_property("has_entry", value));
        self.builder
    }
    pub fn id_column(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("id_column", val));
        value.connect_update(move |value| obj.set_property("id_column", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn popup_fixed_width(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("popup_fixed_width", val));
        value.connect_update(move |value| obj.set_property("popup_fixed_width", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn button_sensitivity(
        &mut self,
        value: &(impl Prop<SensitivityType> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("button_sensitivity", val));
        value.connect_update(move |value| obj.set_property("button_sensitivity", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
}
pub struct ComboBoxSignals<'builder> {
    builder: &'builder mut ComboBoxBuilder,
}
impl<'builder> ComboBoxSignals<'builder> {
    pub fn active_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_active_notify(f);
        &mut self.builder
    }
    pub fn active_id_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_active_id_notify(f);
        &mut self.builder
    }
    pub fn format_entry_text(
        &mut self,
        f: impl Fn(&ComboBox, &str) + 'static,
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_format_entry_text(f);
        &mut self.builder
    }
    pub fn id_column_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_id_column_notify(f);
        &mut self.builder
    }
    pub fn button_sensitivity_notify(
        &mut self,
        f: impl Fn(&ComboBox) + 'static,
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_button_sensitivity_notify(f);
        &mut self.builder
    }
    pub fn child_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_child_notify(f);
        &mut self.builder
    }
    pub fn entry_text_column_notify(
        &mut self,
        f: impl Fn(&ComboBox) + 'static,
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_entry_text_column_notify(f);
        &mut self.builder
    }
    pub fn has_frame_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_frame_notify(f);
        &mut self.builder
    }
    pub fn popdown(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_popdown(f);
        &mut self.builder
    }
    pub fn activate(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_activate(f);
        &mut self.builder
    }
    pub fn changed(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_changed(f);
        &mut self.builder
    }
    pub fn model_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_model_notify(f);
        &mut self.builder
    }
    pub fn popup_fixed_width_notify(
        &mut self,
        f: impl Fn(&ComboBox) + 'static,
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_popup_fixed_width_notify(f);
        &mut self.builder
    }
    pub fn popup_shown_notify(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_popup_shown_notify(f);
        &mut self.builder
    }
    pub fn move_active(
        &mut self,
        f: impl Fn(&ComboBox, ScrollType) + 'static,
    ) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_move_active(f);
        &mut self.builder
    }
    pub fn popup(&mut self, f: impl Fn(&ComboBox) + 'static) -> &mut ComboBoxBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_popup(f);
        &mut self.builder
    }
}
impl ForteExt for ComboBox {
    type Builder = ComboBoxBuilder;
}
#[macro_export]
macro_rules ! ComboBox { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ComboBox :: forte ()) $ ($ rest) * } } }
