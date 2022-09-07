use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct B {
    num: String,
    name: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(ignore)]
    id: i64,

    #[convert_field(to_string)]
    num: i64,

    #[convert_field(unwrap)]
    name: Option<String>,
}

fn main() {
    let a = A {
        id: 2,
        num: 1,
        name: Some("Jack".to_string()),
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            num: "1".to_string(),
            name: "Jack".to_string(),
        },
        b
    );
}
