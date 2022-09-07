use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, Default, PartialEq)]
struct B {
    bid: i64,
    inner: BInner,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid")]
    id: i64,

    inner: AInner,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "BInner")]
struct AInner {
    name: String,
}

fn main() {
    let a = A {
        id: 2,
        inner: AInner {
            name: String::from("AInner"),
        },
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            bid: 2,
            inner: BInner {
                name: String::from("AInner")
            },
        },
        b
    );
}
