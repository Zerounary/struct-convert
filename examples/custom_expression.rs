use struct_convert::Convert;

use helpers::{Object, Uuuuuid};

#[derive(Debug, PartialEq)]
struct A {
    id: String,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "A")]
struct B {
    id: String,
    // Call a custom function as any valid expression
    #[convert_field(custom_fn = "Object::custom_new()")]
    obj: Object,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "A")]
struct C {
    id: String,
    // Use `this` to access to the source instance of the structure
    #[convert_field(custom_fn = "Object::new_with_name(&this.id)")]
    obj: Object,
}
// or
#[derive(Debug, Convert, PartialEq)]
#[convert(into = "C")]
struct E {
    id: String,
    // Use `.` to access to the current field
    #[convert_field(custom_fn = "Object::new_with_name(&.name)")]
    obj: Object,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(into = "A")]
struct D {
    // Use `this` to access to the current instance of the structure
    #[convert_field(custom_fn = "this.id.hex()")]
    id: Uuuuuid,
}
// or
#[derive(Debug, Convert, PartialEq)]
#[convert(into = "A")]
struct F {
    // Use `.` to access to the current field
    #[convert_field(custom_fn = ".hex()")]
    id: Uuuuuid,
}

fn main() {}

#[test]
fn test_custom_expression() {
    let a = A { id: "4".into() };
    let b: B = a.into();
    debug_assert_eq!(
        B {
            id: "4".into(),
            obj: Object { name: "new".into() }
        },
        b
    );

    let a = A { id: "2".into() };
    let c: C = a.into();
    debug_assert_eq!(
        C {
            id: "2".into(),
            obj: Object { name: "2".into() }
        },
        c
    );

    let d = D {
        id: Uuuuuid {
            data: "_id_".into(),
        },
    };
    let a = d.into();
    debug_assert_eq!(A { id: "_id_".into() }, a);
}

mod helpers {
    #[derive(Debug, PartialEq)]
    pub struct Object {
        pub name: String,
    }

    impl Object {
        pub fn custom_new() -> Self {
            Self { name: "new".into() }
        }

        pub fn new_with_name(name: &str) -> Self {
            Self { name: name.into() }
        }
    }

    // some Uuuuuid for id store
    #[derive(Debug, PartialEq)]
    pub struct Uuuuuid {
        pub data: String,
    }

    impl Uuuuuid {
        pub fn hex(self) -> String {
            // transform to hex
            self.data
        }
    }
}
