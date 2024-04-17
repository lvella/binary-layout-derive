use trybuild::TestCases;

#[test]
fn test_layout_as_enum_repr() {
    let t = TestCases::new();

    t.pass("tests/enum_tests/enum_pass.rs");
    t.compile_fail("tests/enum_tests/enum_fail.rs");
}
