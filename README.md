# Struct Convert

This crate provides a set of alternative customizable #[derive] attributes for Rust. To help you covnert between structs simplify. 

# What it does
```rust
use struct_convert::Convert;

#[derive(Convert)]
#[convert(from = "some_mod::Remote")]
#[convert(from = "C")]
#[convert(into = "B")]
struct A {
    value: i64,
}

struct B {
    value: i64,
}

struct C {
    value: i64,
}

mod some_mod {
    pub struct Remote {
        pub value: i64,
    }
}

fn main() {}

#[test]
fn test_proxy() {
    let c = C { value: 8 };
    let a: A = c.into();
    let b: B = a.into();
    debug_assert_eq!(8, b.value);

    let r = some_mod::Remote { value: 7 };
    let a2: A = r.into();
    debug_assert_eq!(7, a2.value);
}
```

Chech the [examples](https://github.com/Zerounary/struct-convert/tree/main/examples) for more!

# Example

Combination of [Derivative](https://crates.io/crates/derivative) and sturct-convert

```rust
use derivative::Derivative;
use struct_convert::Convert;
use time::OffsetDateTime;

#[derive(Derivative, Convert, Debug)]
#[derivative(Default)]
#[convert(default)]
#[convert(from = "A")]
struct SomeStruct {
    name: String,
    #[derivative(Default(value = "OffsetDateTime::now_utc()"))]
    #[convert_field(from="A", ignore)]
    at: OffsetDateTime,
}

struct A {
    name: String,
}

fn main() {
    let a = A{
        name: "A".to_string()
    };
    let ss: SomeStruct = a.into();
    println!("{:?}", ss);
    // SomeStruct { name: "A", at: 2023-03-03 6:13:32.5684174 +00:00:00 }
}
```


Inner stuct convert.

```rust
use struct_convert::Convert;

#[derive(Debug, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, PartialEq)]
struct B {
    bid: i64,
    inner: BInner,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid")]
    id: i64,

    inner: AInner,
}

#[derive(Debug, Convert, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct B {
    num: String,
    name: String,
}

#[derive(Debug, Convert, PartialEq)]
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

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "AInner")]
struct BInner {
    name: String,
}

#[derive(Debug, Convert, PartialEq)]
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

#[derive(Debug,  PartialEq)]
struct A {
    ignore_f: i64,
    id: i64,
    num: i64,
    name: Option<String>,
    inner: AInner,
    opt_str: String,
    opt_str2: Option<String>,
}


#[derive(Debug, PartialEq)]
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

Convert with custom function.

```rust
use struct_convert::Convert;

#[derive(Debug, PartialEq)]
struct B {
    bid: i64,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid", custom_fn = "str_to_i64")]
    id_str: String,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "B")]
struct C {
    #[convert_field(rename = "bid", custom_fn = "to_point")]
    point: Point,
}

#[derive(Debug, PartialEq)]
struct Point(i64, i64);

fn str_to_i64(a: &A) -> i64 {
    a.id_str.parse().unwrap()
}

fn to_point(b: &B) -> Point {
  Point(b.bid, b.bid)
}

fn main() {
    let a = A { id_str: "4".into() };
    let b: B = a.into();
    debug_assert_eq!(B { bid: 4 }, b);
    let c: C = b.into();
    debug_assert_eq!(C { point: Point(4, 4) }, c);

}

```

More examples look [here](https://github.com/Zerounary/struct-convert/tree/main/examples).

Welcome PR.

