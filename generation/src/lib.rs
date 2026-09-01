#![feature(box_patterns)]

#[cfg(test)]
mod tests;
pub mod class;
pub mod prop;
pub mod generate;
pub mod module;
pub mod signal;
pub mod attribute;

extern crate syn;
extern crate quote;
extern crate rust_format;
extern crate convert_case;


