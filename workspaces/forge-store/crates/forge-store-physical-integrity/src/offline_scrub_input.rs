use crate::{ScrubWindow, ScrubWindowOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineScrubVerifierBasis {
    declared_window_count: u64,
    declared_byte_count: u64,
}

impl OfflineScrubVerifierBasis {
    pub const fn declared_window_count(self) -> u64 {
        self.declared_window_count
    }

    pub const fn declared_byte_count(self) -> u64 {
        self.declared_byte_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineScrubInspectionInputDenial {
    EmptyDeclaredWindowSet,
    EmptyDeclaredWindow { ordinal: ScrubWindowOrdinal },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineScrubInspectionInput<'lease> {
    windows: Vec<ScrubWindow<'lease>>,
    basis: OfflineScrubVerifierBasis,
}

impl<'lease> OfflineScrubInspectionInput<'lease> {
    pub fn from_declared_windows(
        windows: Vec<(ScrubWindowOrdinal, &'lease [u8])>,
    ) -> Result<Self, OfflineScrubInspectionInputDenial> {
        if windows.is_empty() {
            return Err(OfflineScrubInspectionInputDenial::EmptyDeclaredWindowSet);
        }

        let mut declared_byte_count = 0u64;
        let mut admitted = Vec::with_capacity(windows.len());
        for (ordinal, bytes) in windows {
            if bytes.is_empty() {
                return Err(OfflineScrubInspectionInputDenial::EmptyDeclaredWindow { ordinal });
            }
            declared_byte_count += bytes.len() as u64;
            admitted.push(ScrubWindow::offline_declared(ordinal, bytes));
        }

        Ok(Self {
            basis: OfflineScrubVerifierBasis {
                declared_window_count: admitted.len() as u64,
                declared_byte_count,
            },
            windows: admitted,
        })
    }

    pub fn windows(&self) -> &[ScrubWindow<'lease>] {
        &self.windows
    }

    pub const fn basis(&self) -> OfflineScrubVerifierBasis {
        self.basis
    }

    pub const fn proves_live_runtime_state(&self) -> bool {
        false
    }
}
