use gtkrs::glib::IsA;
use gtkrs::prelude::*;
use gtkrs::Widget;

impl crate::ousia::box_::BoxBuilder {
    pub fn append(&mut self, w: &impl IsA<Widget>) -> &mut Self {
        let w = w.clone();
        self.on_build(move |obj| { obj.append(&w); });
        self
    }
}