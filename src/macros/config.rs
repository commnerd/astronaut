#[macro_export]
macro_rules! config {
    ($($tt:tt)*) => {
        use astronaut::serde_json::json;
        // astronaut::app::get_app().config = json!();
    };
}