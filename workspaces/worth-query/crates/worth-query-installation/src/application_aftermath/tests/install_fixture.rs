//! Shared install fixtures for aftermath classification tests.

use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolVersion, CanonicalDigestId,
};
use worth_query_declaration::facade::application_aftermath::DeclaredApplicationAftermathContract;
use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredCompensation, DeclaredCorrectionMechanism,
    DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand, DeclaredRecordedInverse,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationExternalEffectProtocol, ApplicationSchemaBindingIdentity,
};

use super::super::{
    install_application_aftermath, AftermathLoweringCorrespondenceCatalog,
    InstalledExternalEffectContract, InstalledLoweringCorrespondence,
    OperationAftermathInstallation, OperationDeclaredReadFields,
    WorthQueryAftermathInstallationDenial, WorthQueryInstalledAftermathContract,
};

pub(super) fn digest(byte: u8) -> CanonicalDigestId {
    CanonicalDigestId::new([byte; 32])
}

/// One operation under aftermath installation, with every axis a test may vary.
///
/// The escaping lane defaults to `None` — the operation declared no
/// operation-definition external-effect slot — and `escaping()` is the twin where it
/// did. Making the lane a visible axis here is the point: it is the operation's
/// own contract, and no aftermath declaration can contradict it (Q8.25-C1).
pub(super) struct AftermathInstall<'a> {
    binding: ApplicationSchemaBindingIdentity,
    operation_slot: &'a str,
    declared_reads: OperationDeclaredReadFields,
    lowering_catalog: AftermathLoweringCorrespondenceCatalog,
    external_effect: InstalledExternalEffectContract,
}

impl<'a> AftermathInstall<'a> {
    pub(super) fn new(binding: ApplicationSchemaBindingIdentity, operation_slot: &'a str) -> Self {
        Self {
            binding,
            operation_slot,
            declared_reads: OperationDeclaredReadFields::from_field_slots(Vec::<String>::new()),
            lowering_catalog: AftermathLoweringCorrespondenceCatalog::empty(),
            external_effect: InstalledExternalEffectContract::None,
        }
    }

    pub(super) fn reads(mut self, slots: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.declared_reads = OperationDeclaredReadFields::from_field_slots(slots);
        self
    }

    pub(super) fn catalog(mut self, catalog: AftermathLoweringCorrespondenceCatalog) -> Self {
        self.lowering_catalog = catalog;
        self
    }

    /// The operation definition declares an external-effect slot on the schema.
    pub(super) fn escaping(self) -> Self {
        self.escaping_with_protocol(protocol(1))
    }

    pub(super) fn escaping_with_protocol(
        self,
        protocol: ApplicationExternalEffectProtocol,
    ) -> Self {
        self.escaping_with_contract("fixture::EscapingPayload", protocol)
    }

    pub(super) fn escaping_with_contract(
        mut self,
        rust_payload_type: &str,
        protocol: ApplicationExternalEffectProtocol,
    ) -> Self {
        self.external_effect = InstalledExternalEffectContract::Declared {
            correlation_family: "escaped-rail".to_owned(),
            effect: "EscapingEffect".to_owned(),
            rust_payload_type: rust_payload_type.to_owned(),
            protocol,
            maximum_payload_bytes: 64,
        };
        self
    }

    pub(super) fn install(
        &self,
        declared: &DeclaredApplicationAftermathContract,
    ) -> Result<WorthQueryInstalledAftermathContract, WorthQueryAftermathInstallationDenial> {
        install_application_aftermath(OperationAftermathInstallation {
            binding: &self.binding,
            operation_slot: self.operation_slot,
            declared,
            declared_reads: &self.declared_reads,
            external_effect: &self.external_effect,
            lowering_catalog: &self.lowering_catalog,
        })
    }
}

pub(super) fn protocol(version: u32) -> ApplicationExternalEffectProtocol {
    ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.escaping-payload"),
        BoundaryProtocolVersion::new(version),
    )
}

pub(super) fn binding(
    package: CanonicalDigestId,
    schema: CanonicalDigestId,
    generation: u64,
) -> ApplicationSchemaBindingIdentity {
    ApplicationSchemaBindingIdentity::from_installed_parts(1, generation, package, schema)
}

pub(super) fn geometry_catalog(
    generation: u64,
    graph: CanonicalDigestId,
) -> AftermathLoweringCorrespondenceCatalog {
    AftermathLoweringCorrespondenceCatalog::new([InstalledLoweringCorrespondence::new(
        "geometry-inverse",
        CanonicalDigestId::new([0xCC; 32]),
        generation,
        graph,
    )
    .unwrap()])
}

pub(super) fn recorded_inverse(field: &str) -> DeclaredCorrectionMechanism {
    DeclaredCorrectionMechanism::RecordedInverse(
        DeclaredRecordedInverse::new(
            "unfreeze",
            DeclaredLoweringCorrespondenceRef::new("geometry-inverse").unwrap(),
            DeclaredAftermathPostcondition::ExactPriorTruth,
            DeclaredPreImageDemand::new([field], 256).unwrap(),
        )
        .unwrap(),
    )
}

pub(super) fn compensation() -> DeclaredCorrectionMechanism {
    DeclaredCorrectionMechanism::Compensation(
        DeclaredCompensation::new(
            "compensating-journal",
            DeclaredAftermathPostcondition::BusinessPostcondition {
                identity: "settled".into(),
            },
        )
        .unwrap(),
    )
}
