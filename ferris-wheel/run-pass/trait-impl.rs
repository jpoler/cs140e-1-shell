// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::*;

impl Duration {
  fn to_millis(&self) -> u64 {
    match self {
      &MilliSeconds(millis) => millis,
      &Seconds(secs) => secs as u64 *1000 as u64,
      &Minutes(mins) => mins as u64*60*1000,
    }
  }
}

impl PartialEq for Duration {
  fn eq(&self, other: &Duration) -> bool {
    self.to_millis() == other.to_millis()
  }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
