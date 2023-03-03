use struct_convert::Convert;

#[derive(Default, Convert)]
#[convert(from = "C")]
#[convert(into = "B")]
struct A {
    value: i64,
}

struct B {
    value: i64
}

struct C {
    value: i64,
}

fn main() {
    let c = C { value: 8 };
    let a: A = c.into();
    let b: B = a.into();
    debug_assert_eq!(8, b.value);
}