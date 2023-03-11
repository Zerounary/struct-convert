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