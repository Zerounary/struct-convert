use struct_convert::Convert;

#[derive(Debug, PartialEq)]
struct B {
    bid: i64,
    num: String,
    name: String,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid")]
    id: i64,

    #[convert_field(to_string)]
    num: i64,

    #[convert_field(unwrap)]
    name: Option<String>,
}

fn main() {
}

#[test]
fn test_sample() {
    let a = A {
        id: 2,
        num: 1,
        name: Some("Jack".to_string()),
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            num: "1".to_string(),
            bid: 2,
            name: "Jack".to_string(),
        },
        b
    );
}
