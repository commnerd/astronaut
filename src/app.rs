use rocket::{Rocket, Route, Build, Ignite, Orbit};
use serde_json::Value;

pub enum PhasedRocket {
    Build(Rocket<Build>),
    Ignite(Rocket<Ignite>),
    Orbit(Rocket<Orbit>),
}

pub struct App {
    pub config: Option<Value>,
    pub rocket: PhasedRocket,
}

const APP: Option<App> = None;

impl App {
    pub fn set_endpoint<R>(self, path: &'static str, callback: R) -> Self
    where R: Into<Vec<Route>> {
        App {
            config: None,
            rocket: match self.rocket {
                PhasedRocket::Build(rkt) => PhasedRocket::Build(rkt.mount(path, callback)),
                PhasedRocket::Ignite(rkt) => PhasedRocket::Ignite(rkt),
                PhasedRocket::Orbit(rkt) => PhasedRocket::Orbit(rkt),
            }
        }
        
    }

    pub fn to_the_moon(self) -> App {
        App{
            config: self.config,
            rocket: self.rocket,
        }
    }
}

pub fn get_app() -> App {
    App {
        config: None,
        rocket: match APP {
            Some(a) => a.rocket,
            None => PhasedRocket::Build(rocket::build()),
        }
    }
}