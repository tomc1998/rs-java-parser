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
            // This is a comment
            System.out.println("Hello, world!"); // This is a comment at the end of a line
            System.out.println("3 + 0.2 = " + c);
            /* This is a nested comment
             * With some code() inside
            System.out.println("Hello");
             */
        }
    }
    "#;
    let ret = lex_str(java_code);
    for t in ret {
        println!("{:?}", t);
    }
}
