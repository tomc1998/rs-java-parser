#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
extern crate test;
extern crate regex;

pub mod lexer;
pub mod project;

