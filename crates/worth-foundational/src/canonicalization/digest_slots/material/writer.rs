#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalMaterialByteLimitExceeded {
    maximum: usize,
    attempted: usize,
}

pub(super) type CanonicalMaterialResult<T = ()> = Result<T, CanonicalMaterialByteLimitExceeded>;

impl CanonicalMaterialByteLimitExceeded {
    pub(crate) const fn maximum(self) -> usize {
        self.maximum
    }

    pub(crate) const fn attempted(self) -> usize {
        self.attempted
    }
}

pub(super) struct CanonicalMaterialWriter {
    material: String,
    maximum_encoded_bytes: usize,
}

impl CanonicalMaterialWriter {
    pub(super) fn bounded(maximum_encoded_bytes: usize) -> Self {
        Self {
            material: String::new(),
            maximum_encoded_bytes,
        }
    }

    pub(super) fn append(&mut self, value: &str) -> CanonicalMaterialResult {
        let attempted = self.material.len().checked_add(value.len()).ok_or(
            CanonicalMaterialByteLimitExceeded {
                maximum: self.maximum_encoded_bytes,
                attempted: usize::MAX,
            },
        )?;
        if attempted > self.maximum_encoded_bytes {
            return Err(CanonicalMaterialByteLimitExceeded {
                maximum: self.maximum_encoded_bytes,
                attempted,
            });
        }
        if attempted > self.material.capacity() {
            self.material
                .reserve_exact(attempted - self.material.capacity());
        }
        self.material.push_str(value);
        Ok(())
    }

    pub(super) fn finish(self) -> CanonicalEncodedMaterial {
        CanonicalEncodedMaterial {
            allocation_bytes: self.material.capacity(),
            material: self.material.into_bytes(),
        }
    }
}

pub(crate) struct CanonicalEncodedMaterial {
    material: Vec<u8>,
    allocation_bytes: usize,
}

impl CanonicalEncodedMaterial {
    pub(crate) const fn encoded_bytes(&self) -> usize {
        self.material.len()
    }

    pub(crate) const fn allocation_bytes(&self) -> usize {
        self.allocation_bytes
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.material
    }
}
