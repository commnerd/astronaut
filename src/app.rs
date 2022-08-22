use rocket::{Rocket, Route, Build};
use serde_json::Value;

pub struct App {
    pub config: Option<Value>,
    pub rocket: Rocket<Build>,
}

impl App {
    pub fn set_endpoint<R>(self, path: &'static str, callback: R) -> Self
    where R: Into<Vec<Route>> {
        App {
            config: None,
            rocket: self.rocket.mount(path, callback),
        }
        
    }

    pub fn to_the_moon(self) -> App {
        App{
            config: self.config,
            rocket: self.rocket,
        }
    }

    pub fn new() -> App {
        App{
            config: None,
            rocket: rocket::build(),
        }
    }
}

pub fn get_app() -> App {
    App::new()
}