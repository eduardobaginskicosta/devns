use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum ResultCode {
  #[default]
  NoError = 0x0,
  FormError = 0x1,
  ServerFail = 0x2,
  NxDomain = 0x3,
  NoTimp = 0x4,
  Refused = 0x5,
}

impl From<u8> for ResultCode {
  fn from(value: u8) -> Self {
    match value {
      0x1 => Self::FormError,
      0x2 => Self::ServerFail,
      0x3 => Self::NxDomain,
      0x4 => Self::NoTimp,
      0x5 => Self::Refused,
      _ => Self::NoError,
    }
  }
}

impl Display for ResultCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NoError => write!(f, "NOERROR"),
      Self::FormError => write!(f, "FORMERROR"),
      Self::ServerFail => write!(f, "SERVERFAIL"),
      Self::NxDomain => write!(f, "NXDOMAIN"),
      Self::NoTimp => write!(f, "NOTIMP"),
      Self::Refused => write!(f, "REFUSED"),
    }
  }
}
