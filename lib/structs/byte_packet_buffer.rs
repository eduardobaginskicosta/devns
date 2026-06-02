use crate::enums::BytePacketError;

pub const PACKET_BUFFER_SIZE: usize = 0x500;
pub struct BytePacketBuffer {
  position: usize,
  buffer: [u8; PACKET_BUFFER_SIZE],
}

impl BytePacketBuffer {
  pub fn new() -> Self {
    Self { position: 0, buffer: [0; PACKET_BUFFER_SIZE] }
  }

  // * members
  pub fn pos(&self) -> usize {
    self.position
  }
  pub fn buffer(&mut self) -> &mut [u8; PACKET_BUFFER_SIZE] {
    &mut self.buffer
  }

  // * actions
  pub fn step(&mut self, steps: usize) -> Result<(), BytePacketError> {
    self.position += steps;
    Ok(())
  }
  pub fn seek(&mut self, pos: usize) -> Result<(), BytePacketError> {
    self.position = pos;
    Ok(())
  }

  // * safe read
  pub fn read(&mut self) -> Result<u8, BytePacketError> {
    let byte: u8 = *self.buffer.get(self.position).ok_or(BytePacketError::EndOfBuffer)?;
    self.position += 1;
    Ok(byte)
  }

  pub fn read_u16(&mut self) -> Result<u16, BytePacketError> {
    Ok((self.read()? as u16) << 0x8 | self.read()? as u16)
  }

  pub fn read_u32(&mut self) -> Result<u32, BytePacketError> {
    Ok(
      (self.read()? as u32) << 0x18
        | (self.read()? as u32) << 0x10
        | (self.read()? as u32) << 0x8
        | (self.read()? as u32),
    )
  }

  // * safe get
  pub fn get(&mut self, pos: usize) -> Result<u8, BytePacketError> {
    self.buffer.get(pos).copied().ok_or(BytePacketError::EndOfBuffer)
  }

  pub fn get_range(
    &mut self,
    start: usize,
    len: usize,
  ) -> Result<&[u8], BytePacketError> {
    self.buffer.get(start..start + len).ok_or(BytePacketError::EndOfBuffer)
  }

  // * safe set
  pub fn set(&mut self, pos: usize, val: u8) -> Result<(), BytePacketError> {
    *self.buffer.get_mut(pos).ok_or(BytePacketError::OutOfBounds)? = val;
    Ok(())
  }

  pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), BytePacketError> {
    self.set(pos, (val >> 0x8) as u8)?;
    self.set(pos + 1, (val & 0xFF) as u8)
  }

  // * safe write
  pub fn write(&mut self, val: u8) -> Result<(), BytePacketError> {
    *self.buffer.get_mut(self.position).ok_or(BytePacketError::EndOfBuffer)? = val;
    self.position += 1;
    Ok(())
  }

  pub fn write_u16(&mut self, val: u16) -> Result<(), BytePacketError> {
    self.write((val >> 0x8) as u8)?;
    self.write((val & 0xFF) as u8)
  }

  pub fn write_u32(&mut self, val: u32) -> Result<(), BytePacketError> {
    self.write(((val >> 0x18) & 0xFF) as u8)?;
    self.write(((val >> 0x10) & 0xFF) as u8)?;
    self.write(((val >> 0x8) & 0xFF) as u8)?;
    self.write((val & 0xFF) as u8)
  }

  // * write bytes
  pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), BytePacketError> {
    bytes.iter().try_for_each(|&byte| self.write(byte))
  }

  // * safe read / write qname
  pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), BytePacketError> {
    const COMPRESSION_POINTER: u8 = 0xC0;

    let mut position: usize = self.pos();
    let mut jumped: bool = false;

    let mut jumps_performed: i32 = 0;
    let mut delim: &'static str = "";
    let max_jumps: i32 = 5;

    loop {
      if jumps_performed > max_jumps {
        return Err(BytePacketError::MaxJumpsExceeded);
      }

      let length: u8 = self.get(position)?;
      if length & COMPRESSION_POINTER == COMPRESSION_POINTER {
        if !jumped {
          self.seek(position + 2)?
        }

        let byte2: u16 = self.get(position)? as u16;
        let offset: u16 = (((length as u16) ^ COMPRESSION_POINTER as u16) << 0x8) | byte2;

        position = offset as usize;
        jumps_performed += 1;
        jumped = true;
        continue;
      }

      position += 1;
      if length == 0 {
        break;
      }

      let label: &[u8] = self.get_range(position, length as usize)?;
      if !label.is_empty() {
        outstr.push_str(delim);
        outstr.push_str(&String::from_utf8_lossy(label).to_lowercase());
      }

      position += length as usize;
      delim = ".";
    }
    if !jumped { self.seek(position) } else { Ok(()) }
  }

  pub fn write_qname(&mut self, qname: &str) -> Result<(), BytePacketError> {
    for label in qname.split('.') {
      if label.len() > 0x3F {
        return Err(BytePacketError::LabelTooLong);
      }
      self.write_bytes(label.as_bytes())?
    }
    self.write(0x0)
  }
}
