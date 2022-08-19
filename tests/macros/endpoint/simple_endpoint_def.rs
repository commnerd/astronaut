#[macro_use] extern crate astronaut;
#[macro_use] extern crate rocket;

#[get("/")]
fn something() -> &'static str {
    "Wow, cool!";
}

fn main() {
    endpoint!("/foo", self::something);
}