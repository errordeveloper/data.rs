#![feature(macro_rules, phase)]

#[phase(syntax)]
extern crate thrustmacro;
extern crate thrust;

fn main() {
    thrust! {
        namespace
    };
}
