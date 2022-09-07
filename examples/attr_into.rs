use struct_convert::{Convert};

#[derive(Debug,Default, PartialEq)]
struct B {
    bid: i64,
    num: String,

}

#[derive(Debug,Default, Convert, PartialEq)]
#[convert_into(into = "B")]
// #[attr_into(into = "a::BVo", include("a as b"))] // 别名
// #[attr_into(into = "a::BVo", include("a.to_string()"))] // 直接手动指名
// #[attr_into(into = "a::BVo", exclude("a.to_string()"))] // 直接手动指名
struct A {
    #[convert_field(rename = "bid")]
    id: i64,
    #[convert_field(to_string)]
    num: i64,
}

fn main() {
    let a = A { num: 1, id: 2 };
    let b: B = a.into();
    debug_assert_eq!(B {num: "1".to_string(), bid: 2 }, b);
}
