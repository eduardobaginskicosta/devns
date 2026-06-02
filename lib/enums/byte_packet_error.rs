use std::{error::Error, fmt::Display, io::Error as IoError};

#[derive(Debug)]
pub enum BytePacketError {
  MaxJumpsExceeded,
  LabelTooLong,
  EndOfBuffer,

  IoError(IoError),
  Error(Box<dyn Error + Send + Sync>),
  Custom(String),

  InvalidQueryType(u16),
  EmptyResponseRecived,
  UnknownRecordError,
  OutOfBounds,
  LookupFailed,
}

impl Display for BytePacketError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::MaxJumpsExceeded => write!(f, "maximum DNS pointer jumps exceeded"),
      Self::LabelTooLong => write!(f, "DNS label exceeds maximum length"),
      Self::EndOfBuffer => write!(f, "unexpected end of packet buffer"),

      Self::IoError(err) => write!(f, "I/O error: {err}"),
      Self::Error(err) => write!(f, "{err}"),
      Self::Custom(msg) => write!(f, "{msg}"),

      Self::InvalidQueryType(qtype) => write!(f, "invalid DNS query type: {qtype}"),
      Self::EmptyResponseRecived => write!(f, "received an empty DNS response"),
      Self::UnknownRecordError => write!(f, "unknown DNS record type"),
      Self::OutOfBounds => write!(f, "buffer access out of bounds"),
      Self::LookupFailed => write!(f, "DNS lookup failed"),
    }
  }
}

impl Error for BytePacketError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::IoError(err) => Some(err),
      Self::Error(err) => Some(err.as_ref()),
      _ => None,
    }
  }
}

impl From<IoError> for BytePacketError {
  fn from(value: IoError) -> Self {
    Self::IoError(value)
  }
}
impl From<Box<dyn Error + Send + Sync>> for BytePacketError {
  fn from(value: Box<dyn Error + Send + Sync>) -> Self {
    Self::Error(value)
  }
}
