extern crate iron;

use iron::prelude::*;

fn main() {
    let chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:8556").unwrap();
}

//TODO: Remove this and put the actual web implementation in.
fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}
