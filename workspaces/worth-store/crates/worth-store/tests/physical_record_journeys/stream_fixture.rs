use worth_store::physical_runtime::{RecordWriteSource, RecordWriteSourceError};

pub(super) struct PatternSource {
    declared: u64,
    available: u64,
    produced: u64,
    extra: bool,
}

pub(super) struct RepeatedByteSource {
    declared: u64,
    produced: u64,
    byte: u8,
}

impl RepeatedByteSource {
    pub(super) const fn new(declared: u64, byte: u8) -> Self {
        Self {
            declared,
            produced: 0,
            byte,
        }
    }
}

impl RecordWriteSource for RepeatedByteSource {
    fn declared_length(&self) -> u64 {
        self.declared
    }

    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        let count = target.len().min((self.declared - self.produced) as usize);
        target[..count].fill(self.byte);
        self.produced += count as u64;
        Ok(count)
    }
}
impl PatternSource {
    pub(super) const fn exact(bytes: u64) -> Self {
        Self {
            declared: bytes,
            available: bytes,
            produced: 0,
            extra: false,
        }
    }
    pub(super) const fn truncated(declared: u64, available: u64) -> Self {
        Self {
            declared,
            available,
            produced: 0,
            extra: false,
        }
    }
    pub(super) const fn overlong(declared: u64) -> Self {
        Self {
            declared,
            available: declared,
            produced: 0,
            extra: true,
        }
    }
}
impl RecordWriteSource for PatternSource {
    fn declared_length(&self) -> u64 {
        self.declared
    }
    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        if self.produced == self.available {
            if self.extra {
                self.extra = false;
                target[0] = 0xa5;
                return Ok(1);
            }
            return Ok(0);
        }
        let count = target.len().min((self.available - self.produced) as usize);
        for (index, byte) in target[..count].iter_mut().enumerate() {
            *byte = pattern(self.produced + index as u64);
        }
        self.produced += count as u64;
        Ok(count)
    }
}

fn pattern(offset: u64) -> u8 {
    ((offset.wrapping_mul(31).wrapping_add(17)) & 0xff) as u8
}
pub(super) fn pattern_digest(length: u64) -> u64 {
    (0..length).fold(0_u64, |digest, offset| {
        digest.rotate_left(5) ^ u64::from(pattern(offset))
    })
}
pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
