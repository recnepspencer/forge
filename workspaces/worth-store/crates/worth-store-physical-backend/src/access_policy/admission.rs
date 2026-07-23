use crate::{
    AdmittedBackendCapabilityWitness, BackendCapabilityClaimWitness, BackendCapabilityKind,
};

use super::{
    AccessPolicyBufferLifecycleKind, AccessPolicyCounterSnapshot, AccessPolicyCounterStrength,
    AccessPolicyDenial, AccessPolicyDenialKind, AccessPolicyRequest, AdmittedAccessPolicy,
    StoreAccessMode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyAdmission<'backend> {
    backend: &'backend AdmittedBackendCapabilityWitness,
}

impl<'backend> AccessPolicyAdmission<'backend> {
    pub const fn for_backend(backend: &'backend AdmittedBackendCapabilityWitness) -> Self {
        Self { backend }
    }

    pub fn admit(
        self,
        request: AccessPolicyRequest,
    ) -> Result<AdmittedAccessPolicy, AccessPolicyDenial> {
        let mut counters = AccessPolicyCounterSnapshot::new(AccessPolicyCounterStrength::Exact);
        reject_missing_required_inputs(request, counters)?;
        counters = counters.with_security_scope_preservation();
        let capability = self.require_capability(request, counters)?;
        counters = counters_for_admitted_mode(request, counters);
        Ok(AdmittedAccessPolicy::new(request, capability, counters))
    }

    fn require_capability(
        self,
        request: AccessPolicyRequest,
        counters: AccessPolicyCounterSnapshot,
    ) -> Result<BackendCapabilityClaimWitness, AccessPolicyDenial> {
        let kind = required_backend_capability(request);
        for participant in mixed_participant_capabilities(request) {
            let Some(participant) = participant else {
                continue;
            };
            if participant != kind {
                self.backend
                    .require(participant, request.required_evidence())
                    .map_err(|denial| {
                        AccessPolicyDenial::from_backend(denial, counters.with_denial())
                    })?;
            }
        }
        self.backend
            .require(kind, request.required_evidence())
            .map_err(|denial| AccessPolicyDenial::from_backend(denial, counters.with_denial()))
    }
}

fn reject_missing_required_inputs(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    if request.reference().is_none() {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MissingPhysicalReference,
            counters.with_denial(),
        ));
    }
    if request.security_scope().is_none() {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MissingSecurityScope,
            counters.with_denial(),
        ));
    }
    reject_missing_lifecycle(request, counters)?;
    reject_missing_page_cache_policy(request, counters)?;
    reject_direct_io_without_alignment(request, counters)?;
    reject_mmap_without_fault_posture(request, counters)?;
    reject_mixed_without_coherence(request, counters)
}

fn reject_missing_page_cache_policy(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    if request.page_cache_policy().is_some() {
        return Ok(());
    }
    Err(AccessPolicyDenial::new(
        AccessPolicyDenialKind::MissingPageCachePolicy,
        counters.with_page_cache_visibility_check().with_denial(),
    ))
}

fn reject_missing_lifecycle(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    match request.buffer_lifecycle().map(|lifecycle| lifecycle.kind()) {
        Some(AccessPolicyBufferLifecycleKind::PinnedPhysicalLease)
        | Some(AccessPolicyBufferLifecycleKind::DirtyPageTracked) => Ok(()),
        Some(AccessPolicyBufferLifecycleKind::DirtyMmapPage)
            if request_involves_direct_io(request) =>
        {
            Err(AccessPolicyDenial::new(
                AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo,
                counters.with_denial(),
            ))
        }
        _ => Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MissingBufferLifecycle,
            counters.with_denial(),
        )),
    }
}

fn reject_direct_io_without_alignment(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    if !requires_direct_io_alignment(request) {
        return Ok(());
    }
    let alignment_satisfies_request = match (
        request.reference(),
        request.buffer_lifecycle(),
        request.alignment(),
    ) {
        (Some(reference), Some(lifecycle), Some(alignment)) => {
            alignment.is_satisfied_for(reference, lifecycle)
        }
        _ => false,
    };
    if !alignment_satisfies_request {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::DirectIoAlignmentRequired,
            counters.with_direct_io_alignment_check().with_denial(),
        ));
    }
    Ok(())
}

fn reject_mmap_without_fault_posture(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    if requires_mmap_fault_posture(request) && !request.mmap_fault_posture().admits_mmap() {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MmapFaultPostureUnsupported,
            counters.with_denial(),
        ));
    }
    Ok(())
}

fn reject_mixed_without_coherence(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> Result<(), AccessPolicyDenial> {
    if request.mode() != StoreAccessMode::Mixed {
        return Ok(());
    }
    let Some(transition) = request.mixed_transition() else {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MixedModeCoherenceRequired,
            counters.with_mixed_mode_invalidation().with_denial(),
        ));
    };
    if !transition.has_only_physical_participants() {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::InvalidMixedAccessTransition,
            counters.with_mixed_mode_invalidation().with_denial(),
        ));
    }
    let Some(reference) = request.reference() else {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MissingPhysicalReference,
            counters.with_denial(),
        ));
    };
    let Some(security_scope) = request.security_scope() else {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MissingSecurityScope,
            counters.with_denial(),
        ));
    };
    if !request
        .coherence_basis()
        .is_some_and(|basis| basis.matches_request(transition, reference, security_scope))
    {
        return Err(AccessPolicyDenial::new(
            AccessPolicyDenialKind::MixedModeCoherenceRequired,
            counters.with_mixed_mode_invalidation().with_denial(),
        ));
    }
    Ok(())
}

fn required_backend_capability(request: AccessPolicyRequest) -> BackendCapabilityKind {
    match request.mode() {
        StoreAccessMode::Buffered => capability_for_mode(StoreAccessMode::Buffered),
        StoreAccessMode::Mmap => capability_for_mode(StoreAccessMode::Mmap),
        StoreAccessMode::DirectIo => capability_for_mode(StoreAccessMode::DirectIo),
        StoreAccessMode::Mixed => match request
            .mixed_transition()
            .map(|transition| transition.requested())
        {
            Some(mode) => capability_for_mode(mode),
            None => BackendCapabilityKind::BufferedFile,
        },
    }
}

fn mixed_participant_capabilities(
    request: AccessPolicyRequest,
) -> [Option<BackendCapabilityKind>; 2] {
    let Some(transition) = request.mixed_transition() else {
        return [None, None];
    };
    [
        Some(capability_for_mode(transition.previous())),
        Some(capability_for_mode(transition.requested())),
    ]
}

const fn capability_for_mode(mode: StoreAccessMode) -> BackendCapabilityKind {
    match mode {
        StoreAccessMode::Buffered | StoreAccessMode::Mixed => BackendCapabilityKind::BufferedFile,
        StoreAccessMode::Mmap => BackendCapabilityKind::Mmap,
        StoreAccessMode::DirectIo => BackendCapabilityKind::DirectIo,
    }
}

fn requires_direct_io_alignment(request: AccessPolicyRequest) -> bool {
    request_involves_direct_io(request)
}

fn requires_mmap_fault_posture(request: AccessPolicyRequest) -> bool {
    matches!(request.mode(), StoreAccessMode::Mmap)
        || request
            .mixed_transition()
            .is_some_and(|transition| transition.involves(StoreAccessMode::Mmap))
}

fn request_involves_direct_io(request: AccessPolicyRequest) -> bool {
    matches!(request.mode(), StoreAccessMode::DirectIo)
        || request
            .mixed_transition()
            .is_some_and(|transition| transition.involves(StoreAccessMode::DirectIo))
}

fn counters_for_admitted_mode(
    request: AccessPolicyRequest,
    counters: AccessPolicyCounterSnapshot,
) -> AccessPolicyCounterSnapshot {
    match request.mode() {
        StoreAccessMode::Buffered => counters
            .with_page_cache_visibility_check()
            .with_buffered_admission(),
        StoreAccessMode::Mmap => counters.with_mmap_admission(),
        StoreAccessMode::DirectIo => counters
            .with_direct_io_alignment_check()
            .with_direct_io_admission(),
        StoreAccessMode::Mixed => counters.with_mixed_mode_admission(),
    }
}
