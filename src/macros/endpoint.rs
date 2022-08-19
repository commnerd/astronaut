#[macro_export]
macro_rules! endpoint{
    ($path: expr, $func: path) => {
        astronaut::app::get_app().set_endpoint($path, routes![$func])
    };
}