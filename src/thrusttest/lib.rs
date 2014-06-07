#![crate_id = "thrusttest"]
#![feature(phase)]
//! Thrust test.

#[phase(syntax)]
extern crate thrustmacro;
extern crate thrust;

#[cfg(test)]
mod test {

  #[test]
  fn empty() {
    thrust!();
  }

}
