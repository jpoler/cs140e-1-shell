// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
  fn default() -> Builder {
    Builder{string: None, number: None}
  }

  fn string<S: AsRef<str>>(self, s: S) -> Builder {
    Builder{
      string: Some(s.as_ref().to_string()),
      number: self.number,
    }
  }

  fn number(self, n: usize) -> Builder {
    Builder{
      string: self.string,
      number: Some(n),
    }
  }

  fn to_string(self) -> String {
    match (self.string, self.number) {
      (None, None) => String::new(),
      (Some(string), None) => string,
      (None, Some(number)) => format!("{}", number),
      (Some(string), Some(number)) => format!("{} {}", string, number),
    }
  }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
