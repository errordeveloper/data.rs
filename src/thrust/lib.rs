#![crate_id = "thrust"]
#![crate_type = "lib"]

//! Thrust Library

pub struct Server {
  ip: String,
  port: u16
}

pub struct Thrust {
  servers: Vec<Server>
}

impl Thrust {
  pub fn new() -> Thrust {
    Thrust {
      servers: Vec::new()
    }
  }

  pub fn server(&self, ip: &str, port: u16) -> Server {
    Server {
      ip: ip.to_string(),
      port: port
    }
  }
}
