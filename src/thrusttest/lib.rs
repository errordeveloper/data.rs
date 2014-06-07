#![crate_id = "thrusttest"]
#![feature(phase)]
//! Thrust test.

#[phase(syntax)]
extern crate thrustmacro;
extern crate thrust;

#[cfg(test)]
mod test {

  use thrust::Server;
  use thrust::Thrust;

  #[test]
  fn empty() {
    thrust!();
  }

  #[test]
  fn thrust() {
    let thrust: Thrust = thrust!();
  }

}
