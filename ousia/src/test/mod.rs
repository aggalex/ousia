use crate::ousia;
use crate::prelude::*;

fn function_that_uses_prop(prop: &(impl Prop<str> + ?Sized)) {
    prop.connect_update(|s| println!("Function that uses prop says: {s}"));
}

#[test]
fn value() {
    let value = Reactive::new(5);
    value.with(|v| println!("{}", v));
    value.connect_update(move |n: &i32| println!("{}", n));


    let map = value.map(|v: &i32| "Hello ".repeat(*v as usize));
    function_that_uses_prop(&map);
    map.connect_update(move |s: &String| println!("{s}"));

    value.set(78);
}