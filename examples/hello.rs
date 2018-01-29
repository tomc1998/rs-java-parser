extern crate java_parser;

use java_parser::lexer::lex_str;

fn main() {
    let java_code = r#"
    package com.tom.test;

    public class Main {
        public static void main(String[] args) {
            float a = 3.f;
            float b = .2f;
            float c = a + b;
            System.out.println("Hello, world!");
            System.out.println("3 + 0.2 = " + c);
        }
    }
    "#;
    println!("{:?}", lex_str(java_code));
}
