#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, TextBuffer, *};
use gtkrs::{TextChildAnchor, TextIter, TextMark, TextTag, TextTagTable};
#[derive(Clone)]
pub struct TextBufferBuilder {
    obj: TextBuffer,
}
impl Default for TextBufferBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl TextBufferBuilder {
    pub fn enable_undo(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_undo", value);
        self
    }
    pub fn tag_table(&mut self, value: &TextTagTable) -> &mut Self {
        self.obj.set_property("tag_table", value);
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("text", value);
        self
    }
    pub fn bind(&mut self) -> TextBufferBinder {
        TextBufferBinder { builder: self }
    }
    pub fn connect(&mut self) -> TextBufferSignals {
        TextBufferSignals { builder: self }
    }
}
impl crate::prelude::Builder for TextBufferBuilder {
    type Target = TextBuffer;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for TextBufferBuilder {
    type Target = TextBuffer;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct TextBufferBinder<'builder> {
    builder: &'builder mut TextBufferBuilder,
}
impl<'builder> TextBufferBinder<'builder> {
    pub fn enable_undo(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_undo", val));
        value.connect_update(move |value| obj.set_property("enable_undo", value));
        self.builder
    }
    pub fn tag_table(
        &mut self,
        value: &(impl Prop<TextTagTable> + ?Sized),
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tag_table", val));
        value.connect_update(move |value| obj.set_property("tag_table", value));
        self.builder
    }
    pub fn text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("text", val));
        value.connect_update(move |value| obj.set_property("text", value));
        self.builder
    }
}
pub struct TextBufferSignals<'builder> {
    builder: &'builder mut TextBufferBuilder,
}
impl<'builder> TextBufferSignals<'builder> {
    pub fn mark_deleted(
        &mut self,
        f: impl Fn(&TextBuffer, &TextMark) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_mark_deleted(f);
        &mut self.builder
    }
    pub fn enable_undo_notify(
        &mut self,
        f: impl Fn(&TextBuffer) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_enable_undo_notify(f);
        &mut self.builder
    }
    pub fn begin_user_action(
        &mut self,
        f: impl Fn(&TextBuffer) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_begin_user_action(f);
        &mut self.builder
    }
    pub fn apply_tag(
        &mut self,
        f: impl Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_apply_tag(f);
        &mut self.builder
    }
    pub fn delete_range(
        &mut self,
        f: impl Fn(&TextBuffer, &TextIter, &TextIter) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_delete_range(f);
        &mut self.builder
    }
    pub fn end_user_action(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_end_user_action(f);
        &mut self.builder
    }
    pub fn has_selection_notify(
        &mut self,
        f: impl Fn(&TextBuffer) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_selection_notify(f);
        &mut self.builder
    }
    pub fn remove_tag(
        &mut self,
        f: impl Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_remove_tag(f);
        &mut self.builder
    }
    pub fn can_undo_notify(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_can_undo_notify(f);
        &mut self.builder
    }
    pub fn can_redo_notify(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_can_redo_notify(f);
        &mut self.builder
    }
    pub fn cursor_position_notify(
        &mut self,
        f: impl Fn(&TextBuffer) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_cursor_position_notify(f);
        &mut self.builder
    }
    pub fn redo(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_redo(f);
        &mut self.builder
    }
    pub fn undo(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_undo(f);
        &mut self.builder
    }
    pub fn mark_set(
        &mut self,
        f: impl Fn(&TextBuffer, &TextIter, &TextMark) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_mark_set(f);
        &mut self.builder
    }
    pub fn changed(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_changed(f);
        &mut self.builder
    }
    pub fn insert_paintable(
        &mut self,
        f: impl Fn(&TextBuffer, &TextIter, &gdk::Paintable) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_paintable(f);
        &mut self.builder
    }
    pub fn insert_child_anchor(
        &mut self,
        f: impl Fn(&TextBuffer, &TextIter, &TextChildAnchor) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_insert_child_anchor(f);
        &mut self.builder
    }
    pub fn modified_changed(
        &mut self,
        f: impl Fn(&TextBuffer) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_modified_changed(f);
        &mut self.builder
    }
    pub fn paste_done(
        &mut self,
        f: impl Fn(&TextBuffer, &gdk::Clipboard) + 'static,
    ) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_paste_done(f);
        &mut self.builder
    }
    pub fn text_notify(&mut self, f: impl Fn(&TextBuffer) + 'static) -> &mut TextBufferBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_text_notify(f);
        &mut self.builder
    }
}
impl ForteExt for TextBuffer {
    type Builder = TextBufferBuilder;
}
#[macro_export]
macro_rules ! TextBuffer { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: TextBuffer :: forte ()) $ ($ rest) * } } }
