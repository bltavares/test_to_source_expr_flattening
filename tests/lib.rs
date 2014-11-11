#![feature(phase)]

#[phase(plugin)]
extern crate test_to_source_expr_flattening;

#[test]
fn foo() {
    assert_eq!(test1!(), test2!());
}

