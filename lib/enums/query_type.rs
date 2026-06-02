use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
pub enum QueryType {
  UNKNOWN(u16),
  CNAME,
  AAAA,
  MX,
  NS,
  #[default]
  A,
}

impl Display for QueryType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::A => write!(f, "A"),
      Self::NS => write!(f, "NS"),
      Self::CNAME => write!(f, "CNAME"),
      Self::MX => write!(f, "MX"),
      Self::AAAA => write!(f, "AAAA"),
      Self::UNKNOWN(num) => write!(f, "UNKNOWN({num:#06X})"),
    }
  }
}

impl From<u16> for QueryType {
  fn from(value: u16) -> Self {
    match value {
      0x01 => Self::A,
      0x02 => Self::NS,
      0x05 => Self::CNAME,
      0x0F => Self::MX,
      0x1C => Self::AAAA,
      _ => Self::UNKNOWN(value),
    }
  }
}

impl From<QueryType> for u16 {
  fn from(value: QueryType) -> Self {
    match value {
      QueryType::A => 0x01,
      QueryType::NS => 0x02,
      QueryType::CNAME => 0x05,
      QueryType::MX => 0x0F,
      QueryType::AAAA => 0x1C,
      QueryType::UNKNOWN(num) => num,
    }
  }
}
