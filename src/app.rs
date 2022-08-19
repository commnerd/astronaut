use rocket::{Rocket, Route, Build, Ignite, Orbit};

pub enum PhasedRocket {
    Build(Rocket<Build>),
    Ignite(Rocket<Ignite>),
    Orbit(Rocket<Orbit>),
}

pub struct App {
    pub rocket: PhasedRocket
}

const APP: Option<App> = None;

impl App {
    pub fn set_endpoint<R>(self, path: &'static str, callback: R) -> Self
    where R: Into<Vec<Route>> {
        App {
            rocket: match self.rocket {
                PhasedRocket::Build(rkt) => PhasedRocket::Build(rkt.mount(path, callback)),
                PhasedRocket::Ignite(rkt) => PhasedRocket::Ignite(rkt),
                PhasedRocket::Orbit(rkt) => PhasedRocket::Orbit(rkt),
            }
        }
        
    }

    pub async fn to_the_moon(self) -> App {
        App{
            rocket: self.rocket
        }
        
    }
}

pub fn get_app() -> App {
    App {
        rocket: match APP {
            Some(a) => a.rocket,
            None => PhasedRocket::Build(rocket::build()),
        }
    }
}