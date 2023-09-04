use crate::b::B;

pub mod b {
  use struct_convert::Convert;
  #[derive(Debug, PartialEq)]
  pub struct B {
    pub bid: i64,
    pub num: String,
    pub name: String,
  }
}


pub mod a1 {
  use struct_convert::Convert;
  #[derive(Debug, Convert, PartialEq)]
  #[convert(into = "crate::b::B")]
  pub struct A {
    #[convert_field(into="crate::b::B", rename = "bid")]
    pub id: i64,

    #[convert_field(to_string)]
    pub num: i64,

    #[convert_field(unwrap)]
    pub name: Option<String>,
  }
}

pub mod a2 {
  use struct_convert::Convert;
  #[derive(Debug, Convert, PartialEq)]
  #[convert(into = "crate :: b :: B")]
  pub struct A {
    #[convert_field(into="crate :: b :: B", rename = "bid")]
    pub id: i64,

    #[convert_field(to_string)]
    pub num: i64,

    #[convert_field(unwrap)]
    pub name: Option<String>,
  }
}

pub mod a3 {
  use struct_convert::Convert;
  #[derive(Debug, Convert, PartialEq)]
  #[convert(into = "  crate::b :: B")]
  pub struct A {
    #[convert_field(into="  crate::b :: B", rename = "bid")]
    pub id: i64,

    #[convert_field(to_string)]
    pub num: i64,

    #[convert_field(unwrap)]
    pub name: Option<String>,
  }
}

fn main() {
}

#[test]
fn test_sample_a1() {
  let a = a1::A {
    id: 2,
    num: 1,
    name: Some("Jack".to_string()),
  };

  let bval: b::B = a.into();

  debug_assert_eq!(
    B {
      num: "1".to_string(),
      bid: 2,
      name: "Jack".to_string(),
    },
    bval
  );
}

#[test]
fn test_sample_a2() {
  let a = a2::A {
    id: 2,
    num: 1,
    name: Some("Jack".to_string()),
  };

  let bval: b::B = a.into();

  debug_assert_eq!(
    B {
      num: "1".to_string(),
      bid: 2,
      name: "Jack".to_string(),
    },
    bval
  );
}

#[test]
fn test_sample_a3() {
  let a = a3::A {
    id: 2,
    num: 1,
    name: Some("Jack".to_string()),
  };

  let bval: b::B = a.into();

  debug_assert_eq!(
    B {
      num: "1".to_string(),
      bid: 2,
      name: "Jack".to_string(),
    },
    bval
  );
}
