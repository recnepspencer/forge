use worth_store_physical_backend::MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES;

use super::{OperationalControlEncodingDenial, PersistedControlRecordDecodeDenial};

pub(super) struct ControlRecordEncoder {
    bytes: Vec<u8>,
}

impl ControlRecordEncoder {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    pub(super) fn bytes(&mut self, value: &[u8]) -> Result<(), OperationalControlEncodingDenial> {
        let total = self
            .bytes
            .len()
            .checked_add(value.len())
            .ok_or(OperationalControlEncodingDenial::RecordTooLarge)?;
        if total > MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES {
            return Err(OperationalControlEncodingDenial::RecordTooLarge);
        }
        self.bytes
            .try_reserve(value.len())
            .map_err(|_| OperationalControlEncodingDenial::AllocationFailed)?;
        self.bytes.extend_from_slice(value);
        Ok(())
    }
    pub(super) fn u8(&mut self, value: u8) -> Result<(), OperationalControlEncodingDenial> {
        self.bytes(&[value])
    }
    pub(super) fn u32(&mut self, value: u32) -> Result<(), OperationalControlEncodingDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn u64(&mut self, value: u64) -> Result<(), OperationalControlEncodingDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn string(&mut self, value: &str) -> Result<(), OperationalControlEncodingDenial> {
        self.length_prefixed_bytes(value.as_bytes())
    }
    pub(super) fn length_prefixed_bytes(
        &mut self,
        value: &[u8],
    ) -> Result<(), OperationalControlEncodingDenial> {
        self.u32(
            u32::try_from(value.len())
                .map_err(|_| OperationalControlEncodingDenial::RecordTooLarge)?,
        )?;
        self.bytes(value)
    }
    pub(super) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

pub(super) struct ControlRecordDecoder<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ControlRecordDecoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    pub(super) fn array<const N: usize>(
        &mut self,
    ) -> Result<[u8; N], PersistedControlRecordDecodeDenial> {
        let end = self
            .position
            .checked_add(N)
            .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        let slice = self
            .bytes
            .get(self.position..end)
            .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        self.position = end;
        Ok(slice.try_into().expect("control record fixed-width slice"))
    }
    pub(super) fn u8(&mut self) -> Result<u8, PersistedControlRecordDecodeDenial> {
        Ok(self.array::<1>()?[0])
    }
    fn u32(&mut self) -> Result<u32, PersistedControlRecordDecodeDenial> {
        Ok(u32::from_le_bytes(self.array()?))
    }
    pub(super) fn u64(&mut self) -> Result<u64, PersistedControlRecordDecodeDenial> {
        Ok(u64::from_le_bytes(self.array()?))
    }
    pub(super) fn string(&mut self) -> Result<String, PersistedControlRecordDecodeDenial> {
        String::from_utf8(self.length_prefixed_bytes()?)
            .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)
    }
    pub(super) fn length_prefixed_bytes(
        &mut self,
    ) -> Result<Vec<u8>, PersistedControlRecordDecodeDenial> {
        let length = usize::try_from(self.u32()?)
            .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        let end = self
            .position
            .checked_add(length)
            .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        let source = self
            .bytes
            .get(self.position..end)
            .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(length)
            .map_err(|_| PersistedControlRecordDecodeDenial::AllocationFailed)?;
        owned.extend_from_slice(source);
        self.position = end;
        Ok(owned)
    }
    pub(super) fn require_eof(self) -> Result<(), PersistedControlRecordDecodeDenial> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err(PersistedControlRecordDecodeDenial::InvalidEncoding)
        }
    }
}
