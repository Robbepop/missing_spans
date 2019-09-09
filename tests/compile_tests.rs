#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    // t.pass("tests/ui/pass/03-flipper-contract.rs");
    t.compile_fail("tests/ui/inline_mod.rs");
    t.compile_fail("tests/ui/inline_mod_with_attr.rs");
    t.compile_fail("tests/ui/struct.rs");
}
