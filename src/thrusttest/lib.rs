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
        thrust!{};
    }

    #[test]
    fn thrust() {
        let thrust: Thrust = thrust!{
            namespace foobar foo
        };
    }

    #[test]
    fn new_server() {
        let thrust = thrust!{};
        let server: Server = thrust.server("127.0.0.1", 4999);
    }

}
