use struct_convert::Convert;

use crate::some_mod::Remote;

#[derive(Convert)]
#[convert(from = "Remote")]
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

mod some_mod {
    pub struct Remote {
        pub value: i64,
    }
}

fn main() {
}

#[test]
fn test_proxy() {
    let c = C { value: 8 };
    let a: A = c.into();
    let b: B = a.into();
    debug_assert_eq!(8, b.value);

    let r = Remote{value: 7};
    let a2: A = r.into();
    debug_assert_eq!(7, a2.value);
}