use std::sync::atomic::{AtomicBool, Ordering};

use super::super::{
    RecordAppendDenial, RecordReadDenial, RecordScanDenial, RecordStreamFailureKind,
};

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) struct ServingHealth {
    inspection_required: AtomicBool,
}

impl ServingHealth {
    pub(in crate::physical_runtime::record_serving) const fn new(
        inspection_required: bool,
    ) -> Self {
        Self {
            inspection_required: AtomicBool::new(inspection_required),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn requires_inspection(&self) -> bool {
        self.inspection_required.load(Ordering::Acquire)
    }

    pub(in crate::physical_runtime::record_serving) fn revoke(&self) {
        self.inspection_required.store(true, Ordering::Release);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_append_denial(
        &self,
        denial: RecordAppendDenial,
    ) {
        if denial == RecordAppendDenial::PublishedLayoutDamaged {
            self.revoke();
        }
    }

    pub(in crate::physical_runtime::record_serving) fn observe_read_denial(
        &self,
        denial: RecordReadDenial,
    ) {
        if matches!(
            denial,
            RecordReadDenial::ArtifactUnavailable
                | RecordReadDenial::ArtifactDamaged
                | RecordReadDenial::FormatMismatch
                | RecordReadDenial::StalePlacement(_)
        ) {
            self.revoke();
        }
    }

    pub(in crate::physical_runtime::record_serving) fn observe_stream_failure(
        &self,
        kind: RecordStreamFailureKind,
    ) {
        if matches!(
            kind,
            RecordStreamFailureKind::ArtifactDamaged
                | RecordStreamFailureKind::FormatMismatch
                | RecordStreamFailureKind::StalePlacement
        ) {
            self.revoke();
        }
    }

    pub(in crate::physical_runtime::record_serving) fn observe_scan_denial(
        &self,
        denial: RecordScanDenial,
    ) {
        match denial {
            RecordScanDenial::ManifestUnavailable => self.revoke(),
            RecordScanDenial::RecordRead(denial) => self.observe_read_denial(denial),
            RecordScanDenial::RecordStream(kind) => self.observe_stream_failure(kind),
            _ => {}
        }
    }
}
