extern crate rocket;
pub extern crate serde_json;

pub mod app;
pub mod build;
pub mod macros;

use app::App;

pub fn to_the_moon() -> App {
    app::get_app().to_the_moon()
}

pub fn build() {
    build::build();
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_the_moon() {
        super::to_the_moon();
    }
}