#[macro_export]
macro_rules! app{
    () => {
        fn main() {
            astronaut::to_the_moon();
        }
    };
}