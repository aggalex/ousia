#![feature(let_else)]

#[cfg(test)]
mod tests;
pub mod class;
pub mod prop;
pub mod generate;
pub mod module;
pub mod signal;

extern crate syn;
extern crate quote;
extern crate rust_format;
extern crate convert_case;


