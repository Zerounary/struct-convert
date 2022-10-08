use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct B {
    bid: i64,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid", custom_fn = "str_to_i64")]
    id_str: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(from = "B")]
struct C {
    #[convert_field(rename = "bid", custom_fn = "to_point")]
    point: Point,
}

#[derive(Debug, Default, PartialEq)]
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
