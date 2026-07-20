use std::sync::Arc;
use worth_proof::ProofOutcome;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    FilesystemBackendProfile, FilesystemMediaOwner, FilesystemMediaOwnerAdmissionDenial,
    FilesystemQualificationMode, FilesystemQualificationRequest, MediaQualificationDeferred,
    MediaQualificationDenial, MediaQualificationFailure, MediaQualificationIdentity,
    MediaQualificationRebindRequired, MediaQualificationStale, QualifiedMediaCapabilities,
    RootProfileQualificationBasis, RootProfileQualificationReport,
};

pub type AdmittedFilesystemMedia = ProofOutcome<
    QualifiedFilesystemMedia,
    MediaQualificationDenial,
    MediaQualificationDeferred,
    MediaQualificationStale,
    MediaQualificationRebindRequired,
    MediaQualificationFailure,
>;

pub struct QualifiedFilesystemMedia {
    owner: FilesystemMediaOwner,
    profile: FilesystemBackendProfile,
    basis: RootProfileQualificationBasis,
    capabilities: QualifiedMediaCapabilities,
    store_identity: StableStoreIdentity,
    mode: FilesystemQualificationMode,
}

impl QualifiedFilesystemMedia {
    pub fn profile(&self) -> &FilesystemBackendProfile {
        &self.profile
    }

    pub fn basis(&self) -> &RootProfileQualificationBasis {
        &self.basis
    }

    pub fn qualification_report(&self) -> RootProfileQualificationReport {
        RootProfileQualificationReport::new(self.basis.binding().clone())
    }

    pub fn capabilities(&self) -> &QualifiedMediaCapabilities {
        &self.capabilities
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store_identity
    }

    pub const fn mode(&self) -> FilesystemQualificationMode {
        self.mode
    }

    pub fn mutation_owner(&self) -> super::MutationOwnerObservation {
        self.owner.mutation_owner()
    }

    pub fn counters(&self) -> super::MediaCounterSnapshot {
        self.owner.counters()
    }

    pub fn counter_observer(&self) -> super::MediaCounterObserver {
        self.owner.counter_observer()
    }

    #[cfg(feature = "certification-test-authority")]
    #[doc(hidden)]
    pub fn certification_confinement_probe(
        &self,
        _authority: super::CertificationMediaFaultAuthority,
        component: &str,
    ) -> Result<(), super::NamespaceConfinementDenial> {
        self.owner.certification_confinement_probe(component)
    }

    #[cfg(feature = "certification-test-authority")]
    #[doc(hidden)]
    pub fn certification_staging_effect_probe(
        &self,
        _authority: super::CertificationMediaFaultAuthority,
        component: &str,
    ) -> super::CertificationConfinementEffect {
        self.owner.certification_staging_effect_probe(component)
    }

    pub fn close(self) -> super::OwnershipReleaseOutcome {
        let Self {
            owner,
            profile,
            basis,
            capabilities,
            store_identity: _,
            mode: _,
        } = self;
        drop((profile, basis, capabilities));
        owner.close()
    }

    #[cfg(test)]
    pub(super) fn into_runtime_parts(
        self,
    ) -> (
        FilesystemMediaOwner,
        FilesystemBackendProfile,
        RootProfileQualificationBasis,
        QualifiedMediaCapabilities,
        StableStoreIdentity,
    ) {
        (
            self.owner,
            self.profile,
            self.basis,
            self.capabilities,
            self.store_identity,
        )
    }
}

impl FilesystemMediaOwner {
    pub(crate) fn qualify(request: FilesystemQualificationRequest) -> AdmittedFilesystemMedia {
        let Some(access_contract) = request.access.admitted_contract() else {
            return worth_proof::TransitionOutcome::denied(
                MediaQualificationDenial::UnmanagedWriterPosture {
                    counters: Box::new(super::MediaCounterSnapshot::default()),
                },
            )
            .into();
        };
        let preflight_counters = Arc::new(super::operation_counters::MediaCounterCells::default());
        let preflight_boundary = super::fault_interposition::MediaFaultInterposer::new(
            request.fault_schedule,
            Arc::clone(&preflight_counters),
        );
        if let Some(runtime_incarnation) = request.runtime_incarnation {
            preflight_boundary.bind_runtime_incarnation(runtime_incarnation);
        }
        let preflight_profile = match super::profile_observation::observe_admission_profile(
            &request.root,
            &preflight_boundary,
        ) {
            Ok(profile) => profile,
            Err(kind) => {
                return worth_proof::TransitionOutcome::stale(
                    MediaQualificationStale::RootUnavailable {
                        kind,
                        counters: preflight_counters.snapshot(),
                    },
                )
                .into()
            }
        };
        if let Some(denial) = super::profile_observation::deny_profile(
            &preflight_profile,
            preflight_counters.snapshot(),
        ) {
            return worth_proof::TransitionOutcome::denied(denial).into();
        }
        if let Some(expected) = request.expected_basis.as_ref() {
            let binding =
                super::profile_observation::profile_binding(&preflight_profile, access_contract);
            if let Some(drift) = super::qualification_basis_drift::basis_drift(expected, &binding) {
                return super::qualification_basis_drift::pre_ownership_drift(
                    drift,
                    binding.root_identity,
                    preflight_counters.snapshot(),
                );
            }
        }
        let owner = match Self::admit_with_boundary(
            &request.root,
            preflight_boundary,
            preflight_counters,
        ) {
            Ok(owner) => owner,
            Err(failure)
                if failure.denial
                    == FilesystemMediaOwnerAdmissionDenial::Ownership(
                        super::MutationOwnershipDenial::Contended,
                    )
                    && !failure.changed_namespace() =>
            {
                return worth_proof::TransitionOutcome::deferred(
                    MediaQualificationDeferred::MutationOwnerContended {
                        counters: failure.counters,
                    },
                )
                .into();
            }
            Err(failure) if !failure.changed_namespace() => {
                return worth_proof::TransitionOutcome::denied(
                    MediaQualificationDenial::OwnerPreEffect {
                        denial: failure.denial,
                        counters: Box::new(failure.counters),
                    },
                )
                .into()
            }
            Err(failure) => {
                return worth_proof::TransitionOutcome::failed(
                    MediaQualificationFailure::OwnerAfterEffect {
                        denial: failure.denial,
                        counters: Box::new(failure.counters),
                    },
                )
                .into()
            }
        };
        let profile =
            match super::profile_observation::observe_profile(&request.root, owner.boundary()) {
                Ok(profile) => profile,
                Err(kind) => {
                    let failure = MediaQualificationFailure::ProfileObservation {
                        kind,
                        counters: Box::new(owner.counters()),
                    };
                    return worth_proof::TransitionOutcome::failed(close_with_failure(
                        owner, failure,
                    ))
                    .into();
                }
            };
        if let Some(denial) = super::profile_observation::deny_profile(&profile, owner.counters()) {
            return worth_proof::TransitionOutcome::denied(close_with_denial(owner, denial)).into();
        }
        let binding = super::profile_observation::profile_binding(&profile, access_contract);
        if let Some(expected) = request.expected_basis.as_ref() {
            if let Some(drift) = super::qualification_basis_drift::basis_drift(expected, &binding) {
                let failure = MediaQualificationFailure::ProfileChangedAfterOwnership {
                    drift,
                    counters: Box::new(owner.counters()),
                };
                return worth_proof::TransitionOutcome::failed(close_with_failure(owner, failure))
                    .into();
            }
        }
        let admitted_identity =
            match super::namespace_identity_admission::admit_store_identity(&owner) {
                Ok(identity) => identity,
                Err(failure) => {
                    return worth_proof::TransitionOutcome::failed(close_with_failure(
                        owner, failure,
                    ))
                    .into()
                }
            };
        owner
            .boundary()
            .bind_store(admitted_identity.stable_identity());
        #[cfg(any(test, feature = "certification-test-authority"))]
        if request.mode == FilesystemQualificationMode::Certification
            && super::qualification_transaction::run_bounded_qualification(&owner, &request.root)
                .is_err()
        {
            let failure = MediaQualificationFailure::QualificationTransaction {
                counters: Box::new(owner.counters()),
            };
            return worth_proof::TransitionOutcome::failed(close_with_failure(owner, failure))
                .into();
        }
        #[cfg(any(test, feature = "certification-test-authority"))]
        if request.mode == FilesystemQualificationMode::Certification {
            owner.boundary().counters().qualification_transaction();
        }
        let Some(qualification) = MediaQualificationIdentity::generate() else {
            let failure = MediaQualificationFailure::QualificationIdentityExhausted {
                counters: Box::new(owner.counters()),
            };
            return worth_proof::TransitionOutcome::failed(close_with_failure(owner, failure))
                .into();
        };
        let (buffered, file_sync, directory_sync, durable_rename) =
            match super::capability_qualification::qualify_backend_claims(
                &owner,
                &admitted_identity,
                &profile,
            ) {
                Ok(claims) => claims,
                Err(denial) => {
                    let denial = MediaQualificationDenial::Capability {
                        denial,
                        counters: Box::new(owner.counters()),
                    };
                    return worth_proof::TransitionOutcome::denied(close_with_denial(
                        owner, denial,
                    ))
                    .into();
                }
            };
        let capabilities = QualifiedMediaCapabilities::for_observed_profile(
            qualification,
            &profile,
            buffered,
            file_sync,
            directory_sync,
            durable_rename,
        );
        let store_identity = admitted_identity.stable_identity();
        worth_proof::TransitionOutcome::success(QualifiedFilesystemMedia {
            owner,
            profile,
            basis: RootProfileQualificationBasis::new(binding),
            capabilities,
            store_identity,
            mode: request.mode,
        })
        .into()
    }
}

fn close_with_failure(
    owner: FilesystemMediaOwner,
    failure: MediaQualificationFailure,
) -> MediaQualificationFailure {
    let counters = owner.counter_observer();
    let _release = owner.close();
    failure.with_terminal_counters(counters.snapshot())
}

fn close_with_denial(
    owner: FilesystemMediaOwner,
    denial: MediaQualificationDenial,
) -> MediaQualificationDenial {
    let counters = owner.counter_observer();
    let _release = owner.close();
    denial.with_terminal_counters(counters.snapshot())
}
