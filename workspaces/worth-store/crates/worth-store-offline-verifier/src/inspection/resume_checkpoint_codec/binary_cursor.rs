use super::{
    charge_owned_allocation, to_usize, OfflineInspectionCheckpointCodecDenial, MAX_CHECKPOINT_BYTES,
};

pub(super) struct CheckpointEncoder {
    bytes: Vec<u8>,
}

impl CheckpointEncoder {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    pub(super) fn bytes(
        &mut self,
        value: &[u8],
    ) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        let total = self
            .bytes
            .len()
            .checked_add(value.len())
            .ok_or(OfflineInspectionCheckpointCodecDenial::SizeLimitExceeded)?;
        if total > MAX_CHECKPOINT_BYTES {
            return Err(OfflineInspectionCheckpointCodecDenial::SizeLimitExceeded);
        }
        self.bytes
            .try_reserve(value.len())
            .map_err(|_| OfflineInspectionCheckpointCodecDenial::AllocationFailed)?;
        self.bytes.extend_from_slice(value);
        Ok(())
    }
    pub(super) fn u8(&mut self, value: u8) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        self.bytes(&[value])
    }
    pub(super) fn u32(&mut self, value: u32) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn u64(&mut self, value: u64) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn string(
        &mut self,
        value: &str,
    ) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        self.u32(
            u32::try_from(value.len())
                .map_err(|_| OfflineInspectionCheckpointCodecDenial::SizeLimitExceeded)?,
        )?;
        self.bytes(value.as_bytes())
    }
    pub(super) fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
    pub(super) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

pub(super) struct CheckpointDecoder<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> CheckpointDecoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    pub(super) fn array<const N: usize>(
        &mut self,
    ) -> Result<[u8; N], OfflineInspectionCheckpointCodecDenial> {
        let end = self
            .position
            .checked_add(N)
            .ok_or(OfflineInspectionCheckpointCodecDenial::InvalidEncoding)?;
        let source = self
            .bytes
            .get(self.position..end)
            .ok_or(OfflineInspectionCheckpointCodecDenial::InvalidEncoding)?;
        self.position = end;
        Ok(source.try_into().expect("fixed-width checkpoint field"))
    }
    pub(super) fn u8(&mut self) -> Result<u8, OfflineInspectionCheckpointCodecDenial> {
        Ok(self.array::<1>()?[0])
    }
    pub(super) fn u32(&mut self) -> Result<u32, OfflineInspectionCheckpointCodecDenial> {
        Ok(u32::from_le_bytes(self.array()?))
    }
    pub(super) fn u64(&mut self) -> Result<u64, OfflineInspectionCheckpointCodecDenial> {
        Ok(u64::from_le_bytes(self.array()?))
    }
    pub(super) fn string(
        &mut self,
        maximum: usize,
        owned_allocation_bytes: &mut u64,
        maximum_owned_allocation_bytes: u64,
    ) -> Result<String, OfflineInspectionCheckpointCodecDenial> {
        let length = to_usize(u64::from(self.u32()?))?;
        if length > maximum {
            return Err(OfflineInspectionCheckpointCodecDenial::SizeLimitExceeded);
        }
        let end = self
            .position
            .checked_add(length)
            .ok_or(OfflineInspectionCheckpointCodecDenial::InvalidEncoding)?;
        let source = self
            .bytes
            .get(self.position..end)
            .ok_or(OfflineInspectionCheckpointCodecDenial::InvalidEncoding)?;
        charge_owned_allocation(
            owned_allocation_bytes,
            u64::try_from(length)
                .map_err(|_| OfflineInspectionCheckpointCodecDenial::SizeLimitExceeded)?,
            maximum_owned_allocation_bytes,
        )?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(length)
            .map_err(|_| OfflineInspectionCheckpointCodecDenial::AllocationFailed)?;
        owned.extend_from_slice(source);
        self.position = end;
        String::from_utf8(owned)
            .map_err(|_| OfflineInspectionCheckpointCodecDenial::InvalidEncoding)
    }
    pub(super) fn require_eof(self) -> Result<(), OfflineInspectionCheckpointCodecDenial> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err(OfflineInspectionCheckpointCodecDenial::InvalidEncoding)
        }
    }
}
