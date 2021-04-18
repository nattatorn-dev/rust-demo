trait Show {
  fn show(&self) -> String;
}

impl Show for i32 {
  fn show(&self) -> String {
      format!("four-byte signed {}", self)
  }
}

impl Show for f64 {
  fn show(&self) -> String {
      format!("eight-byte float {}", self)
  }
}

pub fn demo() {
  let answer = 42;
  let maybe_pi = 3.14;
  let v: Vec<&dyn Show> = vec![&answer,&maybe_pi]; // The dyn Trait feature is the new syntax for using trait objects. In short:
  for d in v.iter() {
      println!("show {}",d.show());
  }
}