use super::{ByteGuardReleaseReceipt, PhysicalByteGuardDenial, PhysicalByteGuardScope};
use crate::{
    PhysicalByteGuardAdmission, PhysicalByteGuardScopeKind, PhysicalReadProtectedFootprintBasis,
};
use forge_store_buffer_pool::{BoundedCopyRecordView, PinnedFrameView, PinnedPageLease};
use forge_store_physical_format::{PhysicalGenerationOwner, PhysicalPayloadViewAdmission};

#[derive(Debug)]
pub struct PhysicalByteGuard<'a> {
    scope: PhysicalByteGuardScope,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    bytes: GuardedPhysicalBytes<'a>,
}

#[derive(Debug)]
enum GuardedPhysicalBytes<'a> {
    PinnedFrame(PinnedFrameView<'a>),
    BorrowedPayload(&'a [u8]),
    OwnedReadBuffer(Vec<u8>),
}

impl<'a> PhysicalByteGuard<'a> {
    pub fn from_pinned_frame(
        admission: PhysicalByteGuardAdmission,
        lease: &'a PinnedPageLease<'a>,
    ) -> Result<Self, PhysicalByteGuardDenial> {
        let scope = admission.scope();
        reject_scope_kind(scope, PhysicalByteGuardScopeKind::ResidentFrame)?;
        reject_owner_mismatch(
            scope.reference().owner(),
            lease.physical_reference()?.generation_owner(),
        )?;
        match scope.resident_frame_token() {
            Some(token) if token == lease.resident_frame_token() => Ok(Self {
                scope,
                footprint_basis: admission.footprint_basis(),
                bytes: GuardedPhysicalBytes::PinnedFrame(lease.view()?),
            }),
            _ => Err(PhysicalByteGuardDenial::GuardScopeMismatch {
                expected: scope,
                observed: PhysicalByteGuardScope::for_resident_frame(
                    scope.reference(),
                    lease.resident_frame_token(),
                ),
            }),
        }
    }

    pub fn from_bounded_copy(
        admission: PhysicalByteGuardAdmission,
        copy: BoundedCopyRecordView,
    ) -> Result<Self, PhysicalByteGuardDenial> {
        reject_scope_kind(
            admission.scope(),
            PhysicalByteGuardScopeKind::OwnedReadBuffer,
        )?;
        reject_owner_mismatch(
            admission.scope().reference().owner(),
            copy.reference().generation_owner(),
        )?;
        let (_, bytes) = copy.into_physical_record_bytes();
        Ok(Self {
            scope: admission.scope(),
            footprint_basis: admission.footprint_basis(),
            bytes: GuardedPhysicalBytes::OwnedReadBuffer(bytes),
        })
    }

    pub fn from_extent_window(
        admission: PhysicalByteGuardAdmission,
        payload: PhysicalPayloadViewAdmission<'a>,
    ) -> Result<Self, PhysicalByteGuardDenial> {
        Self::from_borrowed_payload(admission, payload, PhysicalByteGuardScopeKind::ExtentWindow)
    }

    pub fn from_mmap_view(
        admission: PhysicalByteGuardAdmission,
        payload: PhysicalPayloadViewAdmission<'a>,
    ) -> Result<Self, PhysicalByteGuardDenial> {
        Self::from_borrowed_payload(admission, payload, PhysicalByteGuardScopeKind::MmapView)
    }

    fn from_borrowed_payload(
        admission: PhysicalByteGuardAdmission,
        payload: PhysicalPayloadViewAdmission<'a>,
        expected: PhysicalByteGuardScopeKind,
    ) -> Result<Self, PhysicalByteGuardDenial> {
        reject_scope_kind(admission.scope(), expected)?;
        let view = payload.view();
        reject_owner_mismatch(
            admission.scope().reference().owner(),
            view.witness().owner(),
        )?;
        Ok(Self {
            scope: admission.scope(),
            footprint_basis: admission.footprint_basis(),
            bytes: GuardedPhysicalBytes::BorrowedPayload(view.as_bytes()),
        })
    }

    pub const fn scope(&self) -> PhysicalByteGuardScope {
        self.scope
    }

    pub const fn footprint_basis(&self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub(crate) fn bytes_for_execution(&self) -> &[u8] {
        match &self.bytes {
            GuardedPhysicalBytes::PinnedFrame(view) => view.as_bytes(),
            GuardedPhysicalBytes::BorrowedPayload(bytes) => bytes,
            GuardedPhysicalBytes::OwnedReadBuffer(bytes) => bytes.as_slice(),
        }
    }

    pub fn release(self) -> ByteGuardReleaseReceipt {
        ByteGuardReleaseReceipt::new(self.scope, self.bytes_for_execution().len() as u64)
    }
}

fn reject_scope_kind(
    scope: PhysicalByteGuardScope,
    expected: PhysicalByteGuardScopeKind,
) -> Result<(), PhysicalByteGuardDenial> {
    if scope.kind() == expected {
        Ok(())
    } else {
        Err(PhysicalByteGuardDenial::GuardScopeKindMismatch {
            expected,
            observed: scope.kind(),
        })
    }
}

fn reject_owner_mismatch(
    expected: PhysicalGenerationOwner,
    observed: PhysicalGenerationOwner,
) -> Result<(), PhysicalByteGuardDenial> {
    if expected == observed {
        Ok(())
    } else {
        Err(PhysicalByteGuardDenial::ByteProvenanceMismatch { expected, observed })
    }
}
