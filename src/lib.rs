extern crate rocket;
pub mod app;
pub mod macros;

pub fn to_the_moon() {
    app::get_app().to_the_moon();
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_the_moon() {
        super::to_the_moon();
    }
}