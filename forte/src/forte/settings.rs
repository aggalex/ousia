#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::StyleProvider;
use gtkrs::{prelude::*, Settings, *};
#[derive(Clone)]
pub struct SettingsBuilder {
    obj: Settings,
}
impl Default for SettingsBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl SettingsBuilder {
    pub fn gtk_dnd_drag_threshold(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_dnd_drag_threshold", value);
        self
    }
    pub fn gtk_cursor_blink_time(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_cursor_blink_time", value);
        self
    }
    pub fn gtk_enable_animations(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_enable_animations", value);
        self
    }
    pub fn gtk_xft_dpi(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_xft_dpi", value);
        self
    }
    pub fn gtk_xft_rgba(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_xft_rgba", value);
        self
    }
    pub fn gtk_xft_hintstyle(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_xft_hintstyle", value);
        self
    }
    pub fn gtk_decoration_layout(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_decoration_layout", value);
        self
    }
    pub fn gtk_alternative_sort_arrows(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_alternative_sort_arrows", value);
        self
    }
    pub fn gtk_enable_accels(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_enable_accels", value);
        self
    }
    pub fn gtk_fontconfig_timestamp(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("gtk_fontconfig_timestamp", value);
        self
    }
    pub fn gtk_cursor_theme_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_cursor_theme_name", value);
        self
    }
    pub fn gtk_cursor_aspect_ratio(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("gtk_cursor_aspect_ratio", value);
        self
    }
    pub fn gtk_print_backends(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_print_backends", value);
        self
    }
    pub fn gtk_cursor_theme_size(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_cursor_theme_size", value);
        self
    }
    pub fn gtk_dialogs_use_header(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_dialogs_use_header", value);
        self
    }
    pub fn gtk_keynav_use_caret(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_keynav_use_caret", value);
        self
    }
    pub fn gtk_double_click_time(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_double_click_time", value);
        self
    }
    pub fn gtk_split_cursor(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_split_cursor", value);
        self
    }
    pub fn gtk_theme_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_theme_name", value);
        self
    }
    pub fn gtk_cursor_blink(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_cursor_blink", value);
        self
    }
    pub fn gtk_font_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_font_name", value);
        self
    }
    pub fn gtk_application_prefer_dark_theme(&mut self, value: bool) -> &mut Self {
        self.obj
            .set_property("gtk_application_prefer_dark_theme", value);
        self
    }
    pub fn gtk_enable_primary_paste(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_enable_primary_paste", value);
        self
    }
    pub fn gtk_titlebar_double_click(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_titlebar_double_click", value);
        self
    }
    pub fn gtk_recent_files_max_age(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_recent_files_max_age", value);
        self
    }
    pub fn gtk_titlebar_middle_click(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_titlebar_middle_click", value);
        self
    }
    pub fn gtk_enable_event_sounds(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_enable_event_sounds", value);
        self
    }
    pub fn gtk_double_click_distance(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_double_click_distance", value);
        self
    }
    pub fn gtk_shell_shows_app_menu(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_shell_shows_app_menu", value);
        self
    }
    pub fn gtk_enable_input_feedback_sounds(&mut self, value: bool) -> &mut Self {
        self.obj
            .set_property("gtk_enable_input_feedback_sounds", value);
        self
    }
    pub fn gtk_im_module(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_im_module", value);
        self
    }
    pub fn gtk_sound_theme_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_sound_theme_name", value);
        self
    }
    pub fn gtk_shell_shows_desktop(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_shell_shows_desktop", value);
        self
    }
    pub fn gtk_error_bell(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_error_bell", value);
        self
    }
    pub fn gtk_icon_theme_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_icon_theme_name", value);
        self
    }
    pub fn gtk_titlebar_right_click(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_titlebar_right_click", value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn gtk_hint_font_metrics(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_hint_font_metrics", value);
        self
    }
    pub fn gtk_recent_files_enabled(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_recent_files_enabled", value);
        self
    }
    pub fn gtk_entry_password_hint_timeout(&mut self, value: u32) -> &mut Self {
        self.obj
            .set_property("gtk_entry_password_hint_timeout", value);
        self
    }
    pub fn gtk_alternative_button_order(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_alternative_button_order", value);
        self
    }
    pub fn gtk_entry_select_on_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_entry_select_on_focus", value);
        self
    }
    pub fn gtk_long_press_time(&mut self, value: u32) -> &mut Self {
        self.obj.set_property("gtk_long_press_time", value);
        self
    }
    pub fn gtk_cursor_blink_timeout(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_cursor_blink_timeout", value);
        self
    }
    pub fn gtk_overlay_scrolling(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_overlay_scrolling", value);
        self
    }
    pub fn gtk_primary_button_warps_slider(&mut self, value: bool) -> &mut Self {
        self.obj
            .set_property("gtk_primary_button_warps_slider", value);
        self
    }
    pub fn gtk_print_preview_command(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("gtk_print_preview_command", value);
        self
    }
    pub fn gtk_xft_antialias(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_xft_antialias", value);
        self
    }
    pub fn gtk_label_select_on_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_label_select_on_focus", value);
        self
    }
    pub fn gtk_xft_hinting(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("gtk_xft_hinting", value);
        self
    }
    pub fn gtk_shell_shows_menubar(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("gtk_shell_shows_menubar", value);
        self
    }
    pub fn bind(&mut self) -> SettingsBinder {
        SettingsBinder { builder: self }
    }
    pub fn connect(&mut self) -> SettingsSignals {
        SettingsSignals { builder: self }
    }
}
impl crate::prelude::Builder for SettingsBuilder {
    type Target = Settings;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for SettingsBuilder {
    type Target = Settings;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct SettingsBinder<'builder> {
    builder: &'builder mut SettingsBuilder,
}
impl<'builder> SettingsBinder<'builder> {
    pub fn gtk_dnd_drag_threshold(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_dnd_drag_threshold", val));
        value.connect_update(move |value| obj.set_property("gtk_dnd_drag_threshold", value));
        self.builder
    }
    pub fn gtk_cursor_blink_time(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_blink_time", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_blink_time", value));
        self.builder
    }
    pub fn gtk_enable_animations(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_enable_animations", val));
        value.connect_update(move |value| obj.set_property("gtk_enable_animations", value));
        self.builder
    }
    pub fn gtk_xft_dpi(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_xft_dpi", val));
        value.connect_update(move |value| obj.set_property("gtk_xft_dpi", value));
        self.builder
    }
    pub fn gtk_xft_rgba(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_xft_rgba", val));
        value.connect_update(move |value| obj.set_property("gtk_xft_rgba", value));
        self.builder
    }
    pub fn gtk_xft_hintstyle(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_xft_hintstyle", val));
        value.connect_update(move |value| obj.set_property("gtk_xft_hintstyle", value));
        self.builder
    }
    pub fn gtk_decoration_layout(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_decoration_layout", val));
        value.connect_update(move |value| obj.set_property("gtk_decoration_layout", value));
        self.builder
    }
    pub fn gtk_alternative_sort_arrows(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_alternative_sort_arrows", val));
        value.connect_update(move |value| obj.set_property("gtk_alternative_sort_arrows", value));
        self.builder
    }
    pub fn gtk_enable_accels(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_enable_accels", val));
        value.connect_update(move |value| obj.set_property("gtk_enable_accels", value));
        self.builder
    }
    pub fn gtk_fontconfig_timestamp(
        &mut self,
        value: &(impl Prop<u32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_fontconfig_timestamp", val));
        value.connect_update(move |value| obj.set_property("gtk_fontconfig_timestamp", value));
        self.builder
    }
    pub fn gtk_cursor_theme_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_theme_name", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_theme_name", value));
        self.builder
    }
    pub fn gtk_cursor_aspect_ratio(
        &mut self,
        value: &(impl Prop<f64> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_aspect_ratio", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_aspect_ratio", value));
        self.builder
    }
    pub fn gtk_print_backends(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_print_backends", val));
        value.connect_update(move |value| obj.set_property("gtk_print_backends", value));
        self.builder
    }
    pub fn gtk_cursor_theme_size(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_theme_size", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_theme_size", value));
        self.builder
    }
    pub fn gtk_dialogs_use_header(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_dialogs_use_header", val));
        value.connect_update(move |value| obj.set_property("gtk_dialogs_use_header", value));
        self.builder
    }
    pub fn gtk_keynav_use_caret(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_keynav_use_caret", val));
        value.connect_update(move |value| obj.set_property("gtk_keynav_use_caret", value));
        self.builder
    }
    pub fn gtk_double_click_time(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_double_click_time", val));
        value.connect_update(move |value| obj.set_property("gtk_double_click_time", value));
        self.builder
    }
    pub fn gtk_split_cursor(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_split_cursor", val));
        value.connect_update(move |value| obj.set_property("gtk_split_cursor", value));
        self.builder
    }
    pub fn gtk_theme_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_theme_name", val));
        value.connect_update(move |value| obj.set_property("gtk_theme_name", value));
        self.builder
    }
    pub fn gtk_cursor_blink(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_blink", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_blink", value));
        self.builder
    }
    pub fn gtk_font_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_font_name", val));
        value.connect_update(move |value| obj.set_property("gtk_font_name", value));
        self.builder
    }
    pub fn gtk_application_prefer_dark_theme(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_application_prefer_dark_theme", val));
        value.connect_update(move |value| {
            obj.set_property("gtk_application_prefer_dark_theme", value)
        });
        self.builder
    }
    pub fn gtk_enable_primary_paste(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_enable_primary_paste", val));
        value.connect_update(move |value| obj.set_property("gtk_enable_primary_paste", value));
        self.builder
    }
    pub fn gtk_titlebar_double_click(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_titlebar_double_click", val));
        value.connect_update(move |value| obj.set_property("gtk_titlebar_double_click", value));
        self.builder
    }
    pub fn gtk_recent_files_max_age(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_recent_files_max_age", val));
        value.connect_update(move |value| obj.set_property("gtk_recent_files_max_age", value));
        self.builder
    }
    pub fn gtk_titlebar_middle_click(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_titlebar_middle_click", val));
        value.connect_update(move |value| obj.set_property("gtk_titlebar_middle_click", value));
        self.builder
    }
    pub fn gtk_enable_event_sounds(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_enable_event_sounds", val));
        value.connect_update(move |value| obj.set_property("gtk_enable_event_sounds", value));
        self.builder
    }
    pub fn gtk_double_click_distance(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_double_click_distance", val));
        value.connect_update(move |value| obj.set_property("gtk_double_click_distance", value));
        self.builder
    }
    pub fn gtk_shell_shows_app_menu(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_shell_shows_app_menu", val));
        value.connect_update(move |value| obj.set_property("gtk_shell_shows_app_menu", value));
        self.builder
    }
    pub fn gtk_enable_input_feedback_sounds(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_enable_input_feedback_sounds", val));
        value.connect_update(move |value| {
            obj.set_property("gtk_enable_input_feedback_sounds", value)
        });
        self.builder
    }
    pub fn gtk_im_module(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_im_module", val));
        value.connect_update(move |value| obj.set_property("gtk_im_module", value));
        self.builder
    }
    pub fn gtk_sound_theme_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_sound_theme_name", val));
        value.connect_update(move |value| obj.set_property("gtk_sound_theme_name", value));
        self.builder
    }
    pub fn gtk_shell_shows_desktop(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_shell_shows_desktop", val));
        value.connect_update(move |value| obj.set_property("gtk_shell_shows_desktop", value));
        self.builder
    }
    pub fn gtk_error_bell(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_error_bell", val));
        value.connect_update(move |value| obj.set_property("gtk_error_bell", value));
        self.builder
    }
    pub fn gtk_icon_theme_name(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_icon_theme_name", val));
        value.connect_update(move |value| obj.set_property("gtk_icon_theme_name", value));
        self.builder
    }
    pub fn gtk_titlebar_right_click(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_titlebar_right_click", val));
        value.connect_update(move |value| obj.set_property("gtk_titlebar_right_click", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn gtk_hint_font_metrics(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_hint_font_metrics", val));
        value.connect_update(move |value| obj.set_property("gtk_hint_font_metrics", value));
        self.builder
    }
    pub fn gtk_recent_files_enabled(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_recent_files_enabled", val));
        value.connect_update(move |value| obj.set_property("gtk_recent_files_enabled", value));
        self.builder
    }
    pub fn gtk_entry_password_hint_timeout(
        &mut self,
        value: &(impl Prop<u32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_entry_password_hint_timeout", val));
        value.connect_update(move |value| {
            obj.set_property("gtk_entry_password_hint_timeout", value)
        });
        self.builder
    }
    pub fn gtk_alternative_button_order(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_alternative_button_order", val));
        value.connect_update(move |value| obj.set_property("gtk_alternative_button_order", value));
        self.builder
    }
    pub fn gtk_entry_select_on_focus(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_entry_select_on_focus", val));
        value.connect_update(move |value| obj.set_property("gtk_entry_select_on_focus", value));
        self.builder
    }
    pub fn gtk_long_press_time(
        &mut self,
        value: &(impl Prop<u32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_long_press_time", val));
        value.connect_update(move |value| obj.set_property("gtk_long_press_time", value));
        self.builder
    }
    pub fn gtk_cursor_blink_timeout(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_cursor_blink_timeout", val));
        value.connect_update(move |value| obj.set_property("gtk_cursor_blink_timeout", value));
        self.builder
    }
    pub fn gtk_overlay_scrolling(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_overlay_scrolling", val));
        value.connect_update(move |value| obj.set_property("gtk_overlay_scrolling", value));
        self.builder
    }
    pub fn gtk_primary_button_warps_slider(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_primary_button_warps_slider", val));
        value.connect_update(move |value| {
            obj.set_property("gtk_primary_button_warps_slider", value)
        });
        self.builder
    }
    pub fn gtk_print_preview_command(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_print_preview_command", val));
        value.connect_update(move |value| obj.set_property("gtk_print_preview_command", value));
        self.builder
    }
    pub fn gtk_xft_antialias(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_xft_antialias", val));
        value.connect_update(move |value| obj.set_property("gtk_xft_antialias", value));
        self.builder
    }
    pub fn gtk_label_select_on_focus(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_label_select_on_focus", val));
        value.connect_update(move |value| obj.set_property("gtk_label_select_on_focus", value));
        self.builder
    }
    pub fn gtk_xft_hinting(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_xft_hinting", val));
        value.connect_update(move |value| obj.set_property("gtk_xft_hinting", value));
        self.builder
    }
    pub fn gtk_shell_shows_menubar(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut SettingsBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("gtk_shell_shows_menubar", val));
        value.connect_update(move |value| obj.set_property("gtk_shell_shows_menubar", value));
        self.builder
    }
}
pub struct SettingsSignals<'builder> {
    builder: &'builder mut SettingsBuilder,
}
impl<'builder> SettingsSignals<'builder> {}
impl ForteExt for Settings {
    type Builder = SettingsBuilder;
}
#[macro_export]
macro_rules ! Settings { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Settings :: forte ()) $ ($ rest) * } } }
