#![crate_id = "thrusttest"]
#![feature(phase)]
//! Thrust test.

#[phase(syntax)]
extern crate thrust;

#[cfg(test)]
mod test {

  #[test]
  fn parse() {
    thrust!(
      namespace foobar ThriftTest
    );
  }

}
