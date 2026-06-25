use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
};
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase,
    PlanarLocalFrameCertificateDeclarationFamily, PlanarLocalFrameCertificateEntry,
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
    PlanarLocalFramePerformanceCounters,
};

use super::proof_fixture::{local_frame_basis, precision_handle, precision_receipt};

#[test]
fn spatial_public_facade_exports_planar_local_frame_certificate_surface() {
    let precision = precision_receipt(&precision_handle("surface"), "movement:rotation-cancelled");
    let basis = local_frame_basis(
        &precision,
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis.clone()),
    );

    let _: PlanarLocalFrameCertificateEntry = entry;
    let _: PlanarLocalFrameBasis = basis;
    let _: PlanarLocalFrameCertificateDeclarationFamily =
        PlanarLocalFrameCertificateDeclarationFamily;
    let _: PlanarLocalFrameCertificateQueryDomain = PlanarLocalFrameCertificateQueryDomain;
    let _: PlanarLocalFrameCertificateQueryWorld =
        PlanarLocalFrameCertificateQueryWorld::new("public");
    let _: Option<PlanarLocalFramePerformanceCounters> = None;
}

#[test]
fn planar_local_frame_certificate_family_is_query_native_and_relational() {
    let aspect_contract = PlanarLocalFrameCertificateDeclarationFamily::aspect_contract();

    assert_eq!(
        PlanarLocalFrameCertificateDeclarationFamily::semantic_family_key(),
        "PlanarLocalFrameCertificate"
    );
    assert_eq!(
        PlanarLocalFrameCertificateDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_local_frame.precision_fact"
        )));
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_local_frame.precision_declaration"
        )));
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_local_frame.precision_envelope"
        )));
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.planar_local_frame.transform_chain"
        )));
    assert!(aspect_contract.preserved().contains(
        &crate::query_contract_helpers::aspect_field_key("geometry.planar_local_frame.axes")
    ));
}

#[test]
fn planar_local_frame_declaration_identity_commits_retained_basis_parts() {
    let precision = precision_receipt(
        &precision_handle("declaration-basis"),
        "movement:rotation-cancelled",
    );
    let basis = local_frame_basis(
        &precision,
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis.clone()),
    );
    let canonical_entries = entry.canonical_declaration_entries();

    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_local_frame.precision_declaration"
        ),
        precision.declaration_digest()
    );
    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_local_frame.precision_envelope"
        ),
        precision.envelope_digest()
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_local_frame.u_axis"),
        format!("{:?}", basis.u_axis())
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_local_frame.v_axis"),
        format!("{:?}", basis.v_axis())
    );
    assert_eq!(
        canonical_entry_text(&canonical_entries, "geometry.planar_local_frame.w_axis"),
        format!("{:?}", basis.w_axis())
    );
    assert_eq!(
        canonical_entry_text(
            &canonical_entries,
            "geometry.planar_local_frame.scale_separation"
        ),
        basis.scale_separation_orders().to_string()
    );
}

fn canonical_entry_text(entries: &[ForgeQueryDeclarationCanonicalEntry], locus: &str) -> String {
    let entry = entries
        .iter()
        .find(|entry| entry.locus() == locus)
        .unwrap_or_else(|| panic!("missing canonical declaration entry for {locus}"));
    match entry.value() {
        ForgeQueryDeclarationCanonicalValue::ExactText(text) => text.clone(),
        other => panic!("expected exact text canonical value for {locus}, got {other:?}"),
    }
}
