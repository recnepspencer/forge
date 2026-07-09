use worth_store_physical_format::PhysicalRawSecurityMetadataProjectionSource;

use crate::{
    readmit_deserialized_security_scope_declaration, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, StoreRawPhysicalSecurityMetadataDeclaration,
    StoreRawPhysicalSecurityMetadataProjection, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeDeclarationProvenance,
};

use super::support::{admitted_scope, current_authority, physical_witness};

#[test]
fn terminal_metadata_lowers_only_to_raw_readmission_input() {
    assert_projection_lowers_only_to_raw_readmission_input(
        PhysicalRawSecurityMetadataProjectionSource::TerminalProjected,
    );
}

#[test]
fn serde_loaded_metadata_lowers_only_to_raw_readmission_input() {
    assert_projection_lowers_only_to_raw_readmission_input(
        PhysicalRawSecurityMetadataProjectionSource::SerdeLoaded,
    );
}

fn assert_projection_lowers_only_to_raw_readmission_input(
    source: PhysicalRawSecurityMetadataProjectionSource,
) {
    let authority = current_authority("s51.phase3.projection", source_label(source));
    let witnesses = admitted_scope(&authority);
    let declaration = StoreRawPhysicalSecurityMetadataDeclaration::new(
        witnesses.key_scope().key_scope(),
        witnesses.tenant_scope().tenant_scope(),
        witnesses.authenticity_scope().requirement(),
        witnesses.custody_scope().custody_posture(),
        StoreLegacySecurityPosture::ReadmissionRequired,
        StoreKeyVersionPosture::Current,
    );
    let projection = projection_from_source(source, declaration);

    let raw = projection.to_raw_security_scope_declaration(authority.physical_witness());

    assert_eq!(projection.source(), source);
    assert_eq!(
        raw.provenance(),
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted
    );
    assert_eq!(raw.physical_witness(), physical_witness());
    assert_eq!(raw.key_scope(), declaration.key_scope());
    assert_eq!(raw.tenant_scope(), declaration.tenant_scope());
    assert_eq!(
        raw.authenticity_requirement(),
        Some(declaration.authenticity_requirement())
    );
    assert_eq!(raw.custody_posture(), Some(declaration.custody_posture()));
    assert_eq!(raw.key_version_posture(), declaration.key_version_posture());
    assert!(projection
        .declaration()
        .legacy_posture()
        .requires_readmission_when_unscoped());

    let readmitted = readmit_deserialized_security_scope_declaration(
        &authority,
        raw,
        StoreSecurityScopeAdmissionExpectation::platform_page_envelope(),
    )
    .expect("projected metadata must require explicit Store readmission");
    assert_eq!(
        readmitted.provenance(),
        StoreSecurityScopeDeclarationProvenance::StoreReadmitted
    );
}

const fn projection_from_source(
    source: PhysicalRawSecurityMetadataProjectionSource,
    declaration: StoreRawPhysicalSecurityMetadataDeclaration,
) -> StoreRawPhysicalSecurityMetadataProjection {
    match source {
        PhysicalRawSecurityMetadataProjectionSource::SerdeLoaded => {
            StoreRawPhysicalSecurityMetadataProjection::serde_loaded(declaration)
        }
        PhysicalRawSecurityMetadataProjectionSource::TerminalProjected => {
            StoreRawPhysicalSecurityMetadataProjection::terminal_projected(declaration)
        }
    }
}

const fn source_label(source: PhysicalRawSecurityMetadataProjectionSource) -> &'static str {
    match source {
        PhysicalRawSecurityMetadataProjectionSource::SerdeLoaded => "serde",
        PhysicalRawSecurityMetadataProjectionSource::TerminalProjected => "terminal",
    }
}
