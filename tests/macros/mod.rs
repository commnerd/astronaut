use trybuild;

#[test]
fn simple_endpoint() {
    let t = trybuild::TestCases::new();
    t.pass("tests/macros/endpoint/simple_endpoint_def.rs");
    t.pass("tests/macros/app/simple_app_init.rs");
}