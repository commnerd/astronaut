extern crate rocket;
pub extern crate serde_json;

pub mod app;
pub mod macros;

use app::App;

pub fn to_the_moon() -> App {
    app::get_app().to_the_moon()
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_the_moon() -> App {
        super::to_the_moon()
    }
}