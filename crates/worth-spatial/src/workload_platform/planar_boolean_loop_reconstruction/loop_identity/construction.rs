use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitPersistentNameRow;

use super::counters::PlanarBooleanLoopIdentityMintingCounters;
use super::denial::{
    PlanarBooleanLoopIdentityMintingDenial, PlanarBooleanLoopIdentityMintingDenialKind,
};
use super::identity::{
    canonical_loop_identity, loop_identity_map_identity, loop_identity_row_identity,
    loop_name_map_identity, loop_signature_map_identity, propagated_name_row_identity,
    propagated_persistent_name_identity, propagated_subshape_signature_identity,
    propagated_subshape_signature_row_identity,
};
use super::input::PlanarBooleanLoopIdentityMintingInput;
use super::naming_lineage::validate_name_row_lineage;
use super::product::{
    PlanarBooleanLoopIdentityBoundary, PlanarBooleanLoopIdentityMap,
    PlanarBooleanLoopPersistentNamePropagationMap, PlanarBooleanLoopSubshapeSignatureMap,
};
use super::row::{
    PlanarBooleanLoopIdentityRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopSubshapeSignatureRow,
};
use super::support_index::{IndexedLoopIdentityInputRow, LoopIdentitySupportIndex};

pub(crate) fn mint_loop_identity_boundary(
    input: PlanarBooleanLoopIdentityMintingInput<'_>,
) -> Result<PlanarBooleanLoopIdentityBoundary, PlanarBooleanLoopIdentityMintingDenial> {
    let mut counters = PlanarBooleanLoopIdentityMintingCounters::default();
    let request_identity = input.reconstructed_loops().request_identity().to_string();
    let support_index = LoopIdentitySupportIndex::admit(input, &mut counters)?;
    let mut identity_rows = Vec::new();
    let mut name_rows = Vec::new();
    let mut signature_rows = Vec::new();
    let mut upstream_name_owner = BTreeMap::<String, String>::new();

    for admitted_loop in support_index.admitted_rows() {
        let canonical_identity = canonical_loop_identity(
            &request_identity,
            admitted_loop.tracked_loop_identity(),
            admitted_loop.loop_kind(),
            admitted_loop.role_outcome().role_outcome_identity(),
            admitted_loop
                .degenerate_outcome()
                .degenerate_loop_outcome_identity(),
        );
        identity_rows.push(PlanarBooleanLoopIdentityRow::new(
            loop_identity_row_identity(&canonical_identity, admitted_loop.tracked_loop_identity()),
            admitted_loop.tracked_loop_identity().to_string(),
            canonical_identity.clone(),
            admitted_loop.loop_kind(),
            admitted_loop.source_loop_identities().to_vec(),
            admitted_loop.fragment_identities().to_vec(),
            admitted_loop.split_vertex_identities().to_vec(),
            admitted_loop
                .role_outcome()
                .role_outcome_identity()
                .to_string(),
            admitted_loop
                .degenerate_outcome()
                .degenerate_loop_outcome_identity()
                .to_string(),
        ));
        counters.minted_loop_identity();

        let matched_name_rows = matched_name_rows(admitted_loop, &support_index, &mut counters)?;
        let mut seen_upstream_names = BTreeSet::new();
        for name_row in matched_name_rows {
            validate_name_row_lineage(
                admitted_loop,
                input.naming_support(),
                name_row.source_edge_identity(),
                &mut counters,
            )?;
            if !seen_upstream_names.insert(name_row.persistent_name_identity().to_string()) {
                counters.denied_duplicate_propagated_name();
                return Err(PlanarBooleanLoopIdentityMintingDenial::new(
                    PlanarBooleanLoopIdentityMintingDenialKind::DuplicatePropagatedPersistentName,
                    admitted_loop.tracked_loop_identity().to_string(),
                    counters,
                    "loop identity minting denies loops that resolve the same upstream persistent name identity more than once through distinct propagated rows",
                ));
            }
            if let Some(previous_owner) = upstream_name_owner.insert(
                name_row.persistent_name_identity().to_string(),
                canonical_identity.clone(),
            ) {
                if previous_owner != canonical_identity {
                    counters.denied_duplicate_propagated_name();
                    return Err(PlanarBooleanLoopIdentityMintingDenial::new(
                        PlanarBooleanLoopIdentityMintingDenialKind::DuplicatePropagatedPersistentName,
                        name_row.persistent_name_identity().to_string(),
                        counters,
                        "loop identity minting denies upstream persistent names that attempt to canonically own more than one admitted loop",
                    ));
                }
            }
            let propagated_name_identity = propagated_persistent_name_identity(
                &canonical_identity,
                name_row.persistent_name_identity(),
            );
            name_rows.push(PlanarBooleanLoopPersistentNamePropagationRow::new(
                propagated_name_row_identity(&canonical_identity, &propagated_name_identity),
                canonical_identity.clone(),
                admitted_loop.tracked_loop_identity().to_string(),
                admitted_loop.loop_kind(),
                name_row.persistent_name_identity().to_string(),
                name_row.artifact_identity().to_string(),
                propagated_name_identity,
            ));
            counters.emitted_propagated_name_row();

            let Some(signature_row) =
                support_index.signature_row_for_artifact(name_row.artifact_identity())
            else {
                counters.denied_dangling_name_reference();
                return Err(PlanarBooleanLoopIdentityMintingDenial::new(
                    PlanarBooleanLoopIdentityMintingDenialKind::DanglingNameReference,
                    name_row.artifact_identity().to_string(),
                    counters,
                    "loop identity minting denies propagated naming rows whose upstream artifact has no corresponding split-level subshape signature row",
                ));
            };
            let propagated_signature_identity = propagated_subshape_signature_identity(
                &canonical_identity,
                signature_row.artifact_identity(),
                signature_row.signature_basis_identity(),
            );
            signature_rows.push(PlanarBooleanLoopSubshapeSignatureRow::new(
                propagated_subshape_signature_row_identity(
                    &canonical_identity,
                    &propagated_signature_identity,
                ),
                canonical_identity.clone(),
                admitted_loop.tracked_loop_identity().to_string(),
                admitted_loop.loop_kind(),
                signature_row.artifact_identity().to_string(),
                propagated_signature_identity,
                signature_row.signature_basis_identity().to_string(),
            ));
            counters.emitted_subshape_signature_row();
        }
    }

    identity_rows.sort_by(|left, right| {
        left.loop_kind().cmp(&right.loop_kind()).then_with(|| {
            left.canonical_loop_identity()
                .cmp(right.canonical_loop_identity())
        })
    });
    name_rows.sort_by(|left, right| {
        left.canonical_loop_identity()
            .cmp(right.canonical_loop_identity())
            .then_with(|| {
                left.propagated_persistent_name_identity()
                    .cmp(right.propagated_persistent_name_identity())
            })
    });
    signature_rows.sort_by(|left, right| {
        left.canonical_loop_identity()
            .cmp(right.canonical_loop_identity())
            .then_with(|| {
                left.propagated_signature_identity()
                    .cmp(right.propagated_signature_identity())
            })
    });

    Ok(PlanarBooleanLoopIdentityBoundary::new(
        PlanarBooleanLoopIdentityMap::new(
            loop_identity_map_identity(&request_identity, &identity_rows),
            request_identity.clone(),
            identity_rows,
        ),
        PlanarBooleanLoopPersistentNamePropagationMap::new(
            loop_name_map_identity(&request_identity, &name_rows),
            request_identity.clone(),
            name_rows,
        ),
        PlanarBooleanLoopSubshapeSignatureMap::new(
            loop_signature_map_identity(&request_identity, &signature_rows),
            request_identity,
            signature_rows,
        ),
        counters,
    ))
}

fn matched_name_rows<'a>(
    admitted_loop: &IndexedLoopIdentityInputRow<'a>,
    support_index: &'a LoopIdentitySupportIndex<'a>,
    counters: &mut PlanarBooleanLoopIdentityMintingCounters,
) -> Result<Vec<&'a PlanarBooleanSplitPersistentNameRow>, PlanarBooleanLoopIdentityMintingDenial> {
    let mut matched = Vec::new();
    for artifact_identity in admitted_loop.seed_artifact_identities() {
        if let Some(rows) = support_index.name_rows_for_artifact(&artifact_identity) {
            matched.extend(rows.iter().copied());
        }
    }
    if matched.is_empty() {
        counters.denied_missing_name_seed();
        return Err(PlanarBooleanLoopIdentityMintingDenial::new(
            PlanarBooleanLoopIdentityMintingDenialKind::MissingSplitNamingSeed,
            admitted_loop.tracked_loop_identity().to_string(),
            *counters,
            "loop identity minting requires at least one split-level naming seed for every admitted loop identity",
        ));
    }
    matched.sort_by(|left, right| {
        left.persistent_name_identity()
            .cmp(right.persistent_name_identity())
            .then_with(|| left.artifact_identity().cmp(right.artifact_identity()))
    });
    Ok(matched)
}
