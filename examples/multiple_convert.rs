use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct B {
    bid: String,
    num: String,
    name: String,
}

#[derive(Debug, Default, PartialEq)]
struct C {
    cid: Option<String>,
    num: String,
    name: String,
}

#[derive(Debug, Default, Clone, Convert, PartialEq)]
#[convert(into = "B")]
#[convert(into = "C")]
struct A {
    #[convert_field(class = "B", rename = "bid", to_string)]
    #[convert_field(class = "C", rename = "cid", custom_fn = "wrap_id")]
    id: i64,

    #[convert_field(to_string)]
    num: i64,

    #[convert_field(unwrap)]
    name: Option<String>,
}

fn wrap_id(a: &A) -> Option<String> {
    Some(a.id.to_string())
}

fn main() {
    let a = A {
        id: 2,
        num: 1,
        name: Some("Jack".to_string()),
    };
    let b: B = a.clone().into();
    debug_assert_eq!(
        B {
            num: "1".to_string(),
            bid: 2.to_string(),
            name: "Jack".to_string(),
        },
        b
    );

    let c: C = a.into();
    debug_assert_eq!(
        C {
            num: "1".to_string(),
            cid: Some("2".into()),
            name: "Jack".to_string(),
        },
        c
    );
}
