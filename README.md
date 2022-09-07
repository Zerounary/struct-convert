# Struct Convert

Auto Covnert between structs. 

# Example

A simple struct convert.

```rust
use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct B {
    bid: i64,
    num: String,
    name: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
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
```

Inner stuct convert.

```rust
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
```

Option field convert.

```rust
use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, Default, PartialEq)]
struct B {
    name: String,
    name2: String,
    opt_str: Option<String>,
    opt_str2: Option<String>,
}

#[derive(Debug, Default, Convert, PartialEq)]
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

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "BInner")]
struct AInner {
    name: String,
}

fn main() {
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
```

Ignore Some fileds.

```rust
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
```


convert B from  A
```rust
use struct_convert::Convert;

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(from = "AInner")]
struct BInner {
    name: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(from = "A")]
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
```

Welcome PR.

