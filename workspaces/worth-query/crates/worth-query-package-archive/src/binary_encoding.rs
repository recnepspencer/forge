use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(crate) trait BinaryEncodingSink {
    fn raw_bytes(&mut self, bytes: &[u8]) -> Result<(), Denial>;

    fn claim_nested_entries(&mut self, _count: u32) -> Result<(), Denial> {
        Ok(())
    }

    fn u8(&mut self, value: u8) -> Result<(), Denial> {
        self.raw_bytes(&[value])
    }

    fn u16(&mut self, value: u16) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn u32(&mut self, value: u32) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn u64(&mut self, value: u64) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn i8(&mut self, value: i8) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn i16(&mut self, value: i16) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn i32(&mut self, value: i32) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn i64(&mut self, value: i64) -> Result<(), Denial> {
        self.raw_bytes(&value.to_be_bytes())
    }

    fn text(&mut self, value: &str) -> Result<(), Denial> {
        let length =
            u32::try_from(value.len()).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
        self.u32(length)?;
        self.raw_bytes(value.as_bytes())
    }
}

impl BinaryEncodingSink for BinaryOutput {
    fn raw_bytes(&mut self, bytes: &[u8]) -> Result<(), Denial> {
        BinaryOutput::raw_bytes(self, bytes);
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct BinaryEncodingMeasure {
    bytes: u64,
    nested_entries: u64,
}

impl BinaryEncodingMeasure {
    pub(crate) const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub(crate) const fn nested_entries(&self) -> u64 {
        self.nested_entries
    }
}

impl BinaryEncodingSink for BinaryEncodingMeasure {
    fn raw_bytes(&mut self, bytes: &[u8]) -> Result<(), Denial> {
        self.bytes = self
            .bytes
            .checked_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX))
            .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))?;
        Ok(())
    }

    fn claim_nested_entries(&mut self, count: u32) -> Result<(), Denial> {
        self.nested_entries = self
            .nested_entries
            .checked_add(u64::from(count))
            .ok_or_else(|| Denial::new(Kind::NumericWidthExceeded))?;
        Ok(())
    }
}
