extern crate rocket;
pub extern crate serde_json;

use rocket::Phase;

pub mod app;
pub mod macros;

use app::App;

pub fn to_the_moon() -> App {
    app::get_app().to_the_moon()
}

#[cfg(test)]
mod tests {
    use rocket::Phase;

    #[test]
    fn to_the_moon() {
        super::to_the_moon();
    }
}