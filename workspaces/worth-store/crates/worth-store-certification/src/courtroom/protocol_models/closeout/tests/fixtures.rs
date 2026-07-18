use super::*;

pub(super) fn diagnostic_support_report(
) -> worth_foundational::FoundationalDiagnosticSupportReport {
    materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            diagnostic_subject(),
            FoundationalDiagnosticOutcomeKind::Denied,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .unwrap()
}

pub(super) fn diagnostic_explanation(
) -> worth_foundational::FoundationalDiagnosticExplanationBundle {
    materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            diagnostic_subject(),
            FoundationalDiagnosticOutcomeKind::Denied,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .unwrap()
}

fn diagnostic_subject() -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::BoundaryArtifact {
        artifact_locator: worth_foundational::BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(9),
            BoundaryArtifactField::Payload,
        ),
    }
}

fn diagnostic_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .unwrap()
}

pub(super) fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

pub(super) fn formal_model_crate_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("certification crate has a crates directory")
        .join("worth-store-formal-models")
}

pub(super) fn admitted_backend() -> AdmittedBackendCapabilityWitness {
    admitted_backend_for::<HostDurabilityProfile>()
}

pub(super) fn admitted_backend_for<P: BackendDurabilityProfile>(
) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            P::TARGET,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}
