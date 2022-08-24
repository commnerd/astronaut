#[macro_export]
macro_rules! config {
    ($($tt:tt)*) => {
        use astronaut::serde_json::{Value, json};

        pub fn get() -> Value {
            serde_json::from_str(r#"{ ... }"#).unwrap()
        }
    };
}