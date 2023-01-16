#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, CellAreaBox, *};
use gtkrs::{Buildable, CellArea, CellLayout, CellRenderer, Orientable, Orientation};
#[derive(Clone)]
pub struct CellAreaBoxBuilder {
    obj: CellAreaBox,
}
impl Default for CellAreaBoxBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl CellAreaBoxBuilder {
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn spacing(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("spacing", value);
        self
    }
    pub fn focus_cell(&mut self, value: &impl IsA<CellRenderer>) -> &mut Self {
        self.obj.set_property("focus_cell", value);
        self
    }
    pub fn bind(&mut self) -> CellAreaBoxBinder {
        CellAreaBoxBinder { builder: self }
    }
    pub fn connect(&mut self) -> CellAreaBoxSignals {
        CellAreaBoxSignals { builder: self }
    }
}
impl crate::prelude::Builder for CellAreaBoxBuilder {
    type Target = CellAreaBox;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for CellAreaBoxBuilder {
    type Target = CellAreaBox;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct CellAreaBoxBinder<'builder> {
    builder: &'builder mut CellAreaBoxBuilder,
}
impl<'builder> CellAreaBoxBinder<'builder> {
    pub fn orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut CellAreaBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn spacing(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut CellAreaBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("spacing", val));
        value.connect_update(move |value| obj.set_property("spacing", value));
        self.builder
    }
    pub fn focus_cell<T: IsA<CellRenderer>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut CellAreaBoxBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_cell", val));
        value.connect_update(move |value| obj.set_property("focus_cell", value));
        self.builder
    }
}
pub struct CellAreaBoxSignals<'builder> {
    builder: &'builder mut CellAreaBoxBuilder,
}
impl<'builder> CellAreaBoxSignals<'builder> {}
impl ForteExt for CellAreaBox {
    type Builder = CellAreaBoxBuilder;
}
#[macro_export]
macro_rules ! CellAreaBox { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: CellAreaBox :: forte ()) $ ($ rest) * } } }
