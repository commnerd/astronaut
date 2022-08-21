#[macro_export]
macro_rules! app{
    () => {
        mod config;

        fn main() {
            astronaut::to_the_moon();
        }
    };
}