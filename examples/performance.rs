use std::time::Instant;

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
    innerList: Vec<BInner>,

    opt_str: Option<String>,
    opt_str2: Option<String>,
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

    innerList: Vec<AInner>,

    #[convert_field(option)]
    opt_str: String,

    #[convert_field(option)]
    opt_str2: Option<String>,
}

#[derive(Debug, Default, Clone, Convert, PartialEq)]
#[convert(into = "BInner")]
struct AInner {
    name: String,
}

// impl From<A> for B {
//     fn from(s: A) -> Self {
//        B {
//         bid: s.id.into(),
//         num: s.num.to_string(),
//         name: s.name.unwrap_or_default(),
//         inner: s.inner.into(),
//         innerList: s.innerList.into_iter().map(|a| a.into()).collect(),
//         // innerList: s.innerList.into(), //err
//         // innerList: to_list(&s), //err
//         ..B::default()
//        } 
//     }
// }



fn to_list(a: &A) -> Vec<BInner> {
  return vec![]
}

fn main() {
    let size = 1_000_000;
    let mut list_a: Vec<A> = Vec::with_capacity(size);
    for _i in 1..size {
        let a = A {
            id: 2,
            num: 1,
            name: Some("Jack".to_string()),
            inner: AInner {
                name: String::from("AInner"),
            },
            innerList: vec![AInner {
                name: String::from("AInner"),
            }],
            opt_str: String::from("str"),
            opt_str2: Some(String::from("Option")),
            ignore_f: 1,
        };
        list_a.push(a);
    }

    let now = Instant::now();
    let list_b = list_a.into_iter().map(|a| a.into()).collect::<Vec<B>>();
    println!("{:?}", now.elapsed().as_millis());
    println!("{:?}", list_b.get(0)); // 704ms with AMD 4800H
}
