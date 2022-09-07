use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, Default, PartialEq)]
struct B {
    bid: i64,
    num: String,
    name: String,
    inner: BInner,

    opt_str: Option<String>,
    opt_str2: Option<String>
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "B")]
struct A {

    #[convert_field(ignore)]
    ignore_f: i64,

    #[convert_field(rename = "bid")]
    id: i64,

    #[convert_field(to_string)]
    num: i64,

    #[convert_field(unwrap)]
    name: Option<String>,

    inner: AInner,

    #[convert_field(option)]
    opt_str: String,

    #[convert_field(option)]
    opt_str2: Option<String>,

    
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "BInner")]
struct AInner {
    name: String,
}

fn main() {
    let a = A {
        id: 2,
        num: 1,
        name: Some("Jack".to_string()),
        inner: AInner {
            name: String::from("AInner"),
        },
        opt_str: String::from("str"),
        opt_str2: Some(String::from("Option")),
        ignore_f: 1,
    };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            num: "1".to_string(),
            bid: 2,
            name: "Jack".to_string(),
            inner: BInner {
                name: String::from("AInner")
            },
            opt_str: Some(String::from("str")),
            opt_str2: Some(String::from("Option"))
        },
        b
    );
}
