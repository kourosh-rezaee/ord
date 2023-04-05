use super::*;

#[derive(Copy, Clone)]
pub enum Blocktime {
  Confirmed(DateTime<Utc>),
  Expected(DateTime<Utc>),
}

impl Blocktime {
  pub fn confirmed(seconds: u32) -> Self {
    Self::Confirmed(timestamp(seconds))
  }

  pub fn timestamp(self) -> DateTime<Utc> {
    match self {
      Self::Confirmed(timestamp) | Self::Expected(timestamp) => timestamp,
    }
  }

  pub fn suffix(self) -> &'static str {
    match self {
      Self::Confirmed(_) => "",
      Self::Expected(_) => " (expected)",
    }
  }
}
