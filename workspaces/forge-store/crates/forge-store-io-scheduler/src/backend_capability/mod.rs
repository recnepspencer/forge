mod admission;
mod denial;
mod requirement;

pub use admission::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim,
    IoSchedulerBackendCapabilityAdmission,
};
pub use denial::IoSchedulerBackendCapabilityDenial;
pub use requirement::IoSchedulerBackendCapabilityRequirement;

#[cfg(test)]
mod tests {
    use forge_store_physical_backend::{
        BackendCapabilityAdmissionDenial, BackendCapabilityAdmissionRequest,
        BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
        BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
        BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
    };
    use forge_store_readiness::{
        accept_s5_1_admitted_security_scope_readiness, S51LaterMilestoneHandoffDenial,
        S51SecurityScopeReadinessReservation,
    };
    use forge_store_security::{
        admitted_store_internal_security_scope_for_s6_test,
        admitted_wrong_s6_io_qos_security_scope_for_test,
    };

    use crate::{admit_security_scope_for_scheduler, SchedulerSecurityScopeEvidence};

    use super::*;

    #[test]
    fn every_platform_claim_requires_scheduler_owned_evidence_policy() {
        for requirement in platform_requirements() {
            let witness = externally_guaranteed_witness(
                requirement.capability_kind(),
                BackendCapabilitySupportPosture::Supported,
            );

            let admission = admit_backend_capability_for_scheduler_claim(&witness, requirement)
                .expect("externally guaranteed platform claim should admit");

            assert_eq!(admission.requirement(), requirement);
            assert_eq!(admission.evidence_class(), requirement.required_evidence());
        }
    }

    #[test]
    fn every_platform_claim_denies_weaker_evidence_laundering() {
        for requirement in platform_requirements() {
            for basis in weaker_than_external_evidence() {
                let witness = witness_from_basis_and_posture(
                    requirement.capability_kind(),
                    BackendCapabilitySupportPosture::Supported,
                    basis,
                );

                let denial = admit_backend_capability_for_scheduler_claim(&witness, requirement)
                    .expect_err("scheduler must not lower platform evidence policy");

                assert_evidence_denial(denial);
            }
        }
    }

    #[test]
    fn every_platform_claim_denies_all_non_current_postures() {
        for requirement in platform_requirements() {
            for posture in non_current_postures() {
                let witness = externally_guaranteed_witness(requirement.capability_kind(), posture);

                let denial = admit_backend_capability_for_scheduler_claim(&witness, requirement)
                    .expect_err("scheduler must not consume non-current platform capability");

                assert_scheduler_posture_denial(denial, posture);
            }
        }
    }

    #[test]
    fn secure_frame_claim_requires_security_scope_admission() {
        let witness = externally_guaranteed_witness(
            BackendCapabilityKind::SecureFrameIo,
            BackendCapabilitySupportPosture::Supported,
        );

        let denial = admit_backend_capability_for_scheduler_claim(
            &witness,
            IoSchedulerBackendCapabilityRequirement::SecureFrameIo,
        )
        .expect_err("secure-frame admission must require S.5.1 security scope");

        assert_eq!(
            denial,
            IoSchedulerBackendCapabilityDenial::SecureFrameRequiresSecurityScope
        );
    }

    #[test]
    fn secure_frame_claim_admits_through_s5_1_security_scope_handoff() {
        let security_scope = valid_security_scope();
        let witness = externally_guaranteed_witness(
            BackendCapabilityKind::SecureFrameIo,
            BackendCapabilitySupportPosture::Supported,
        );

        let admission =
            admit_secure_frame_backend_capability_for_scheduler_claim(&witness, &security_scope)
                .expect("secure-frame claim should admit with bound security scope");

        assert_eq!(
            admission.requirement(),
            IoSchedulerBackendCapabilityRequirement::SecureFrameIo
        );
        assert!(admission.security_scope_bound());
    }

    #[test]
    fn secure_frame_claim_rejects_wrong_s5_1_handoff_family() {
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::blob_chunk(),
            admitted_store_internal_security_scope_for_s6_test(),
        );

        let denial = SchedulerSecurityScopeEvidence::from_s5_1_readiness(readiness)
            .expect_err("wrong S.5.1 readiness family must not hand off to S.6 IoQos");

        assert!(matches!(
            denial,
            S51LaterMilestoneHandoffDenial::WrongReadinessFamily { .. }
        ));
    }

    #[test]
    fn secure_frame_claim_rejects_wrong_s5_1_scope_identity() {
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::io_qos(),
            admitted_wrong_s6_io_qos_security_scope_for_test(),
        );

        let denial = SchedulerSecurityScopeEvidence::from_s5_1_readiness(readiness)
            .expect_err("wrong admitted security identity must not hand off to S.6 IoQos");

        assert!(matches!(
            denial,
            S51LaterMilestoneHandoffDenial::WrongKeyScope { .. }
        ));
    }

    #[test]
    fn secure_frame_claim_with_scope_still_denies_weak_backend_evidence() {
        let security_scope = valid_security_scope();

        for basis in weaker_than_external_evidence() {
            let witness = witness_from_basis_and_posture(
                BackendCapabilityKind::SecureFrameIo,
                BackendCapabilitySupportPosture::Supported,
                basis,
            );

            let denial = admit_secure_frame_backend_capability_for_scheduler_claim(
                &witness,
                &security_scope,
            )
            .expect_err("security scope cannot strengthen backend evidence");

            assert_evidence_denial(denial);
        }
    }

    #[test]
    fn secure_frame_claim_with_scope_denies_all_non_current_postures() {
        let security_scope = valid_security_scope();

        for posture in non_current_postures() {
            let witness =
                externally_guaranteed_witness(BackendCapabilityKind::SecureFrameIo, posture);

            let denial = admit_secure_frame_backend_capability_for_scheduler_claim(
                &witness,
                &security_scope,
            )
            .expect_err("scheduler must not consume non-current secure-frame capability");

            assert_scheduler_posture_denial(denial, posture);
        }
    }

    fn externally_guaranteed_witness(
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    ) -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
        witness_from_basis_and_posture(
            kind,
            posture,
            BackendCapabilityEvidenceBasis::externally_guaranteed(1),
        )
    }

    fn witness_from_basis_and_posture(
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
        basis: BackendCapabilityEvidenceBasis,
    ) -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
        let support = BackendCapabilitySupportSet::all_supported().with_posture(kind, posture);
        let request = BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            basis,
            support,
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io()
                .with_flush_ordering(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        );
        PhysicalBackendCapabilityAdmissionAuthority::store_owned()
            .admit_backend_capability(request)
            .expect("baseline backend should admit")
    }

    fn valid_security_scope() -> crate::IoSchedulerSecurityScopeAdmission {
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::io_qos(),
            admitted_store_internal_security_scope_for_s6_test(),
        );
        let handoff = SchedulerSecurityScopeEvidence::from_s5_1_readiness(readiness)
            .expect("S.6 IoQos handoff should admit from S.5.1 readiness");
        admit_security_scope_for_scheduler(handoff)
    }

    fn assert_evidence_denial(denial: IoSchedulerBackendCapabilityDenial) {
        assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::EvidenceClassTooWeak { .. }
            )
        ));
    }

    fn assert_scheduler_posture_denial(
        denial: IoSchedulerBackendCapabilityDenial,
        posture: BackendCapabilitySupportPosture,
    ) {
        match posture {
            BackendCapabilitySupportPosture::Unsupported => assert!(matches!(
                denial,
                IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                    BackendCapabilityAdmissionDenial::UnsupportedCapability { .. }
                )
            )),
            BackendCapabilitySupportPosture::Unavailable => assert!(matches!(
                denial,
                IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                    BackendCapabilityAdmissionDenial::UnavailableCapability { .. }
                )
            )),
            BackendCapabilitySupportPosture::Unknown => assert!(matches!(
                denial,
                IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                    BackendCapabilityAdmissionDenial::UnknownCapability { .. }
                )
            )),
            BackendCapabilitySupportPosture::Stale => assert!(matches!(
                denial,
                IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                    BackendCapabilityAdmissionDenial::StaleCapability { .. }
                )
            )),
            BackendCapabilitySupportPosture::RebindRequired => assert!(matches!(
                denial,
                IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                    BackendCapabilityAdmissionDenial::RebindRequired { .. }
                )
            )),
            BackendCapabilitySupportPosture::Supported => unreachable!(),
        }
    }

    const fn platform_requirements() -> [IoSchedulerBackendCapabilityRequirement; 6] {
        [
            IoSchedulerBackendCapabilityRequirement::DirectIo,
            IoSchedulerBackendCapabilityRequirement::Mmap,
            IoSchedulerBackendCapabilityRequirement::AsyncIo,
            IoSchedulerBackendCapabilityRequirement::Fsync,
            IoSchedulerBackendCapabilityRequirement::DirectorySync,
            IoSchedulerBackendCapabilityRequirement::DurableRename,
        ]
    }

    const fn weaker_than_external_evidence() -> [BackendCapabilityEvidenceBasis; 3] {
        [
            BackendCapabilityEvidenceBasis::declared_by_config(1),
            BackendCapabilityEvidenceBasis::observed_by_probe(1),
            BackendCapabilityEvidenceBasis::unverifiable_assumption(),
        ]
    }

    const fn non_current_postures() -> [BackendCapabilitySupportPosture; 5] {
        [
            BackendCapabilitySupportPosture::Unsupported,
            BackendCapabilitySupportPosture::Unavailable,
            BackendCapabilitySupportPosture::Unknown,
            BackendCapabilitySupportPosture::Stale,
            BackendCapabilitySupportPosture::RebindRequired,
        ]
    }
}
