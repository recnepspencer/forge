use crate::PhysicalSubstrateLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S1CertificationRow {
    PhysicalStoryTranscriptReplay,
    PhysicalScenarioPlanLoweringReplay,
    RoadmapLaneFamilyExtensionWithoutHarnessFork,
    TestSupportCannotOwnCertificationMeaning,
    SinglePageAuthorityReopen,
    StaleGenerationReferenceRejected,
    FreeSpaceGenerationReuseDetectable,
    PhysicalReferenceConstructionSealed,
    PhysicalGenerationOwnershipDeclared,
    StaleCellReferenceDeniedBeforeDecode,
    SemanticDigestCannotBePhysicalIdentity,
    BinaryFormatGoldenBytesReplay,
    BinaryFormatSerializerAccidentRejected,
    BinaryFormatMigrationPostureDeclared,
    HeaderDecodeWitnessSealed,
    PayloadViewRequiresHeaderDecodeWitness,
    UnsupportedHeaderVersionRejectedBeforePayload,
    ReservedHeaderFieldMisuseRejectedBeforePayload,
    MultiSegmentAuthorityScan,
    DerivedRecordNonAuthority,
    ExtentBackedLargeRecord,
    UnknownFrameKindRejectedBeforeDecode,
    LengthMismatchRejectedBeforePayload,
    SlotDirectoryLocateBounded,
    MovedSlotBoundedOrDenied,
    RootManifestDiscovery,
    RootPublicationAmbiguityDenied,
    LocateByReferenceIgnoresUnrelatedStoreGrowth,
    FragmentedFreeSpaceAppendBoundedOrDenied,
    LegacyHeapBackendPlatformClaimRejected,
    LegacyFileSqliteBackendTierFenced,
    WholeStoreMaterializationForbidden,
    BackendResidueGuessingForbidden,
    MinimalOfflineVerifierManifestSmoke,
    OfflineVerifierLayoutMatch,
    OfflineVerifierRuntimeDisagreementReported,
    PhysicalCounterBundleExact,
    PhysicalComplexityContractsVerified,
    FoundationalAdoptionCanonicalParity,
    FoundationalLocalStandInRejected,
    FoundationalPerformanceReceiptRequired,
    FoundationalDiagnosticProfileControlsRichness,
    S2EntryRejectsWeakerPhysicalSubstrateInputs,
}

impl S1CertificationRow {
    pub const fn required_for_s1() -> [Self; 43] {
        [
            Self::PhysicalStoryTranscriptReplay,
            Self::PhysicalScenarioPlanLoweringReplay,
            Self::RoadmapLaneFamilyExtensionWithoutHarnessFork,
            Self::TestSupportCannotOwnCertificationMeaning,
            Self::SinglePageAuthorityReopen,
            Self::StaleGenerationReferenceRejected,
            Self::FreeSpaceGenerationReuseDetectable,
            Self::PhysicalReferenceConstructionSealed,
            Self::PhysicalGenerationOwnershipDeclared,
            Self::StaleCellReferenceDeniedBeforeDecode,
            Self::SemanticDigestCannotBePhysicalIdentity,
            Self::BinaryFormatGoldenBytesReplay,
            Self::BinaryFormatSerializerAccidentRejected,
            Self::BinaryFormatMigrationPostureDeclared,
            Self::HeaderDecodeWitnessSealed,
            Self::PayloadViewRequiresHeaderDecodeWitness,
            Self::UnsupportedHeaderVersionRejectedBeforePayload,
            Self::ReservedHeaderFieldMisuseRejectedBeforePayload,
            Self::MultiSegmentAuthorityScan,
            Self::DerivedRecordNonAuthority,
            Self::ExtentBackedLargeRecord,
            Self::UnknownFrameKindRejectedBeforeDecode,
            Self::LengthMismatchRejectedBeforePayload,
            Self::SlotDirectoryLocateBounded,
            Self::MovedSlotBoundedOrDenied,
            Self::RootManifestDiscovery,
            Self::RootPublicationAmbiguityDenied,
            Self::LocateByReferenceIgnoresUnrelatedStoreGrowth,
            Self::FragmentedFreeSpaceAppendBoundedOrDenied,
            Self::LegacyHeapBackendPlatformClaimRejected,
            Self::LegacyFileSqliteBackendTierFenced,
            Self::WholeStoreMaterializationForbidden,
            Self::BackendResidueGuessingForbidden,
            Self::MinimalOfflineVerifierManifestSmoke,
            Self::OfflineVerifierLayoutMatch,
            Self::OfflineVerifierRuntimeDisagreementReported,
            Self::PhysicalCounterBundleExact,
            Self::PhysicalComplexityContractsVerified,
            Self::FoundationalAdoptionCanonicalParity,
            Self::FoundationalLocalStandInRejected,
            Self::FoundationalPerformanceReceiptRequired,
            Self::FoundationalDiagnosticProfileControlsRichness,
            Self::S2EntryRejectsWeakerPhysicalSubstrateInputs,
        ]
    }

    pub const fn physical_substrate_lanes(self) -> &'static [PhysicalSubstrateLane] {
        use PhysicalSubstrateLane as Lane;
        match self {
            Self::PhysicalStoryTranscriptReplay
            | Self::PhysicalScenarioPlanLoweringReplay
            | Self::RoadmapLaneFamilyExtensionWithoutHarnessFork
            | Self::TestSupportCannotOwnCertificationMeaning
            | Self::SinglePageAuthorityReopen
            | Self::MultiSegmentAuthorityScan
            | Self::DerivedRecordNonAuthority
            | Self::ExtentBackedLargeRecord
            | Self::RootManifestDiscovery
            | Self::LocateByReferenceIgnoresUnrelatedStoreGrowth => &[Lane::HappyAuthority],
            Self::StaleGenerationReferenceRejected
            | Self::FreeSpaceGenerationReuseDetectable
            | Self::PhysicalReferenceConstructionSealed
            | Self::PhysicalGenerationOwnershipDeclared
            | Self::StaleCellReferenceDeniedBeforeDecode
            | Self::SemanticDigestCannotBePhysicalIdentity
            | Self::MovedSlotBoundedOrDenied => &[Lane::HostileReference],
            Self::BinaryFormatGoldenBytesReplay
            | Self::BinaryFormatSerializerAccidentRejected
            | Self::BinaryFormatMigrationPostureDeclared
            | Self::HeaderDecodeWitnessSealed
            | Self::PayloadViewRequiresHeaderDecodeWitness
            | Self::UnknownFrameKindRejectedBeforeDecode
            | Self::UnsupportedHeaderVersionRejectedBeforePayload
            | Self::LengthMismatchRejectedBeforePayload
            | Self::ReservedHeaderFieldMisuseRejectedBeforePayload
            | Self::SlotDirectoryLocateBounded => &[Lane::HostileFormat],
            Self::RootPublicationAmbiguityDenied => &[Lane::HostileFormat, Lane::HappyAuthority],
            Self::FragmentedFreeSpaceAppendBoundedOrDenied
            | Self::PhysicalCounterBundleExact
            | Self::PhysicalComplexityContractsVerified => &[Lane::ScaleLocality],
            Self::LegacyHeapBackendPlatformClaimRejected
            | Self::LegacyFileSqliteBackendTierFenced
            | Self::WholeStoreMaterializationForbidden
            | Self::BackendResidueGuessingForbidden => &[Lane::LegacyOverclaim],
            Self::MinimalOfflineVerifierManifestSmoke
            | Self::OfflineVerifierLayoutMatch
            | Self::OfflineVerifierRuntimeDisagreementReported => &[Lane::OfflineVerifier],
            Self::FoundationalAdoptionCanonicalParity
            | Self::FoundationalLocalStandInRejected
            | Self::FoundationalPerformanceReceiptRequired
            | Self::FoundationalDiagnosticProfileControlsRichness => &[Lane::FoundationalExport],
            Self::S2EntryRejectsWeakerPhysicalSubstrateInputs => &[Lane::S2Handoff],
        }
    }
}
