use std::io::Read;

use super::BackupBundleFormatDenial;

pub(super) struct FallibleEncoder {
    bytes: Vec<u8>,
}

impl FallibleEncoder {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    pub(super) fn bytes(&mut self, value: &[u8]) -> Result<(), BackupBundleFormatDenial> {
        self.bytes
            .try_reserve(value.len())
            .map_err(|_| BackupBundleFormatDenial::ManifestAllocationFailed)?;
        self.bytes.extend_from_slice(value);
        Ok(())
    }
    pub(super) fn u8(&mut self, value: u8) -> Result<(), BackupBundleFormatDenial> {
        self.bytes(&[value])
    }
    pub(super) fn u16(&mut self, value: u16) -> Result<(), BackupBundleFormatDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn u32(&mut self, value: u32) -> Result<(), BackupBundleFormatDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn u64(&mut self, value: u64) -> Result<(), BackupBundleFormatDenial> {
        self.bytes(&value.to_le_bytes())
    }
    pub(super) fn string(&mut self, value: &str) -> Result<(), BackupBundleFormatDenial> {
        self.u32(
            u32::try_from(value.len()).map_err(|_| BackupBundleFormatDenial::InvalidManifest)?,
        )?;
        self.bytes(value.as_bytes())
    }
    pub(super) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

pub(super) struct ManifestDecoder<R> {
    reader: R,
    remaining_bytes: u64,
    owned_allocation_bytes: u64,
    maximum_owned_allocation_bytes: u64,
}

impl<R: Read> ManifestDecoder<R> {
    pub(super) fn new(reader: R, encoded_bytes: u64, maximum_owned_allocation_bytes: u64) -> Self {
        Self {
            reader,
            remaining_bytes: encoded_bytes,
            owned_allocation_bytes: 0,
            maximum_owned_allocation_bytes,
        }
    }
    pub(super) fn array<const N: usize>(&mut self) -> Result<[u8; N], BackupBundleFormatDenial> {
        let mut bytes = [0; N];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }
    pub(super) fn u8(&mut self) -> Result<u8, BackupBundleFormatDenial> {
        Ok(self.array::<1>()?[0])
    }
    pub(super) fn u16(&mut self) -> Result<u16, BackupBundleFormatDenial> {
        Ok(u16::from_le_bytes(self.array()?))
    }
    pub(super) fn u32(&mut self) -> Result<u32, BackupBundleFormatDenial> {
        Ok(u32::from_le_bytes(self.array()?))
    }
    pub(super) fn u64(&mut self) -> Result<u64, BackupBundleFormatDenial> {
        Ok(u64::from_le_bytes(self.array()?))
    }
    pub(super) fn string(&mut self) -> Result<String, BackupBundleFormatDenial> {
        let length = usize::try_from(self.u32()?)
            .map_err(|_| BackupBundleFormatDenial::ManifestAllocationCountOverflow)?;
        if length as u64 > self.remaining_bytes {
            return Err(BackupBundleFormatDenial::InvalidManifest);
        }
        self.charge_owned_allocation(length as u64)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| BackupBundleFormatDenial::ManifestAllocationFailed)?;
        bytes.resize(length, 0);
        self.read_exact(&mut bytes)?;
        String::from_utf8(bytes).map_err(|_| BackupBundleFormatDenial::InvalidManifest)
    }
    pub(super) fn charge_owned_allocation(
        &mut self,
        requested_bytes: u64,
    ) -> Result<(), BackupBundleFormatDenial> {
        let observed_bytes = self
            .owned_allocation_bytes
            .checked_add(requested_bytes)
            .ok_or(BackupBundleFormatDenial::ManifestAllocationCountOverflow)?;
        if observed_bytes > self.maximum_owned_allocation_bytes {
            return Err(
                BackupBundleFormatDenial::ManifestOwnedAllocationLimitExceeded {
                    observed_bytes,
                    maximum_bytes: self.maximum_owned_allocation_bytes,
                },
            );
        }
        self.owned_allocation_bytes = observed_bytes;
        Ok(())
    }
    fn read_exact(&mut self, bytes: &mut [u8]) -> Result<(), BackupBundleFormatDenial> {
        let requested = bytes.len() as u64;
        if requested > self.remaining_bytes {
            return Err(BackupBundleFormatDenial::InvalidManifest);
        }
        self.reader.read_exact(bytes).map_err(|error| {
            if error.kind() == std::io::ErrorKind::UnexpectedEof {
                BackupBundleFormatDenial::InvalidManifest
            } else {
                BackupBundleFormatDenial::Read(error)
            }
        })?;
        self.remaining_bytes -= requested;
        Ok(())
    }
    pub(super) fn require_eof(&mut self) -> Result<(), BackupBundleFormatDenial> {
        if self.remaining_bytes != 0 {
            return Err(BackupBundleFormatDenial::InvalidManifest);
        }
        let mut trailing = [0; 1];
        match self.reader.read(&mut trailing) {
            Ok(0) => Ok(()),
            Ok(_) => Err(BackupBundleFormatDenial::InvalidManifest),
            Err(error) => Err(BackupBundleFormatDenial::Read(error)),
        }
    }
}
