use struct_convert::Convert;

#[derive(Debug, Default, Convert, PartialEq)]
#[convert_into(from = "AInner")]
struct BInner {
    name: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert_into(from = "A")]
struct B {

    #[convert_field(rename = "id")]
    bid: i64,

    #[convert_field(to_string)]
    num: String,

    #[convert_field(unwrap)]
    name: String,

    inner: BInner,

    #[convert_field(wrap)]
    opt_str: Option<String>,

    opt_str2: Option<String>
}

#[derive(Debug, Default,  PartialEq)]
struct A {
    ignore_f: i64,
    id: i64,
    num: i64,
    name: Option<String>,
    inner: AInner,
    opt_str: String,
    opt_str2: Option<String>,
}


#[derive(Debug, Default, PartialEq)]
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
