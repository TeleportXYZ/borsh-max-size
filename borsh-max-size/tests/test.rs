#![allow(dead_code)]
use borsh_max_size::MaxSize;

#[derive(MaxSize, Clone, Copy)]
struct Foo8 {
    a: i32,
    b: i32,
}

#[derive(MaxSize, Clone, Copy)]
struct Foo4 {
    a: i32,
}

#[test]
fn test_struct_derive() {
    assert_eq!(Foo8::max_size(), 8);
    assert_eq!(Foo4::max_size(), 4);
}

#[derive(MaxSize)]
enum Foos {
    Foo8(Foo8),
    Foo4(Foo4),
}

#[test]
fn test_enum_derive() {
    assert_eq!(Foos::max_size(), 8);
}

#[derive(MaxSize)]
union FooUnion {
    a: Foo8,
    b: Foo4,
}

#[test]
fn test_union_derive() {
    assert_eq!(FooUnion::max_size(), 8);
}
