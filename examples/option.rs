use struct_convert::Convert;

#[derive(Debug, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, PartialEq)]
struct B {
    name: String,
    name2: String,
    opt_str: Option<String>,
    opt_str2: Option<String>,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(unwrap)]
    name: Option<String>,

    #[convert_field(unwrap)]
    name2: Option<String>,

    #[convert_field(option)]
    opt_str: String,

    #[convert_field(option)]
    opt_str2: Option<String>,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "BInner")]
struct AInner {
    name: String,
}

fn main() {
}

#[test]
fn test_option() {
    let a = A {
        name: Some("Jack".to_string()),
        opt_str: String::from("str"),
        opt_str2: Some(String::from("Option")),
        name2: None,
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            name: "Jack".to_string(),
            opt_str: Some(String::from("str")),
            opt_str2: Some(String::from("Option")),
            name2: "".to_string()
        },
        b
    );
}
