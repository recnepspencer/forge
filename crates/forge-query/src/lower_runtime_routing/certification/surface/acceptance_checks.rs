use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    forge_query_lower_runtime_gap_registry, forge_query_lower_runtime_support_matrix,
    ForgeQueryLowerRuntimeBoundaryExecutionKind, ForgeQueryLowerRuntimeCloseoutPosture,
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSupportPosture,
};

use super::evidence::ForgeQueryLowerRuntimeRepresentativeEvidenceSource;
use super::evidence::ForgeQueryLowerRuntimeRepresentativeSurface;

pub(super) fn control_digest(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(&[
        classification_exactness_digest(),
        deletable_seam_deletion_digest(),
        surviving_specialist_justification_digest(),
        admitted_crossing_cardinality_digest(surface),
        support_behavior_agreement_digest(surface),
        required_concrete_seam_coverage_digest(surface),
    ])
}

pub(super) fn hostile_digest(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(&[
        "deleted-seam-survival-is-forbidden".to_string(),
        "specialist-gap-survival-is-forbidden".to_string(),
        "crossing-cardinality-drift-is-forbidden".to_string(),
        "support-behavior-drift-is-forbidden".to_string(),
        "required-phase-six-seams-must-not-fall-back-to-synthetic".to_string(),
        surface.route_parity_digest().to_string(),
    ])
}

pub(crate) fn required_phase_six_concrete_seams() -> &'static [ForgeQueryLowerRuntimeSeamKey] {
    &[
        ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
    ]
}

fn required_concrete_seam_coverage_digest(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> String {
    for seam_key in required_phase_six_concrete_seams() {
        assert_eq!(
            surface.evidence_source_for(*seam_key),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture),
            "required phase six seam {} must remain runtime-backed",
            seam_key.as_str()
        );
    }

    hash_parts(
        &required_phase_six_concrete_seams()
            .iter()
            .map(|seam_key| seam_key.as_str().to_string())
            .collect::<Vec<_>>(),
    )
}

fn classification_exactness_digest() -> String {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let mut counts = BTreeMap::new();
    for row in crossings.rows() {
        *counts.entry(row.seam_key().as_str()).or_insert(0usize) += 1;
    }
    assert!(
        counts.values().all(|count| *count == 1),
        "every crossing must be classified exactly once"
    );
    hash_parts(
        &crossings
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|{}",
                    row.seam_key().as_str(),
                    row.classification().as_str(),
                    row.route_kind().as_str()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn deletable_seam_deletion_digest() -> String {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let gaps = forge_query_lower_runtime_gap_registry();
    let support = forge_query_lower_runtime_support_matrix();
    let eliminated = forge_query_lower_runtime_closeout_registry()
        .rows()
        .iter()
        .filter(|row| row.posture() == ForgeQueryLowerRuntimeCloseoutPosture::SeamEliminated)
        .collect::<Vec<_>>();

    for row in &eliminated {
        assert!(
            !crossings
                .rows()
                .iter()
                .any(|crossing| crossing.seam_key() == row.seam_key()),
            "deleted seam must not survive in the crossing inventory"
        );
        assert!(
            !gaps
                .rows()
                .iter()
                .any(|gap| gap.seam_key() == row.seam_key()),
            "deleted seam must not survive in the gap registry"
        );
        let support_row = support
            .support_for(row.seam_key())
            .expect("deleted seam must remain locatable in support");
        assert_eq!(
            support_row.posture(),
            ForgeQueryLowerRuntimeSupportPosture::SeamEliminated
        );
    }

    hash_parts(
        &eliminated
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|{}",
                    row.seam_key().as_str(),
                    row.closeout_target(),
                    row.required_closeout()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn surviving_specialist_justification_digest() -> String {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let gaps = forge_query_lower_runtime_gap_registry();
    let specialists = [
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
    ];

    for seam_key in specialists {
        let row = crossings
            .rows()
            .iter()
            .find(|row| row.seam_key() == seam_key)
            .unwrap_or_else(|| {
                panic!(
                    "specialist seam {} must remain inventoried",
                    seam_key.as_str()
                )
            });
        assert_ne!(
            row.classification(),
            ForgeQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane,
            "specialist seam {} must not survive as compatibility debt",
            seam_key.as_str()
        );
        assert!(
            !gaps.rows().iter().any(|gap| gap.seam_key() == seam_key),
            "specialist seam {} must not survive in the gap registry",
            seam_key.as_str()
        );
    }

    hash_parts(
        &specialists
            .iter()
            .map(|seam_key| {
                let row = crossings
                    .rows()
                    .iter()
                    .find(|row| row.seam_key() == *seam_key)
                    .expect("specialist row should exist");
                format!(
                    "{}|{}|{}",
                    row.seam_key().as_str(),
                    row.classification().as_str(),
                    row.required_action()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn admitted_crossing_cardinality_digest(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> String {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let request_counts = count_by_seam(
        surface
            .requests()
            .iter()
            .map(|request| request.seam_key().as_str()),
    );
    let route_plan_counts = count_by_seam(
        surface
            .route_plans()
            .iter()
            .map(|plan| plan.eligibility().request().seam_key().as_str()),
    );
    let envelope_counts = count_by_seam(
        surface
            .envelopes()
            .iter()
            .map(|envelope| envelope.seam_key().as_str()),
    );
    let receipt_counts = count_receipts_by_seam(surface);
    let receipt_kinds = receipt_kind_by_seam(surface);

    for row in crossings.rows() {
        assert_eq!(
            request_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        assert_eq!(
            receipt_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        assert_eq!(
            envelope_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        match row.route_kind() {
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
                assert_eq!(
                    route_plan_counts.get(row.seam_key().as_str()).copied(),
                    Some(1)
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan)
                );
            }
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
                assert_eq!(
                    route_plan_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    0
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff)
                );
            }
        }
    }

    hash_parts(
        &crossings
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|req:{}|plan:{}|receipt:{}|envelope:{}",
                    row.seam_key().as_str(),
                    row.route_kind().as_str(),
                    request_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    route_plan_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    receipt_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    envelope_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0)
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn support_behavior_agreement_digest(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> String {
    let support = forge_query_lower_runtime_support_matrix();
    let closeout = forge_query_lower_runtime_closeout_registry();

    for envelope in surface.envelopes() {
        let support_row = support
            .support_for(envelope.seam_key())
            .expect("every admitted envelope seam must exist in the support matrix");
        assert_eq!(support_row.posture(), envelope.support_posture());
        assert_eq!(support_row.authority_owner(), envelope.authority_owner());
        assert_eq!(support_row.route_kind(), envelope.route_kind());
        assert_eq!(support_row.capability_label(), envelope.capability_label());
    }
    for row in closeout.rows() {
        let support_row = support
            .support_for(row.seam_key())
            .expect("every closeout seam must exist in the support matrix");
        assert_eq!(support_row.authority_owner(), row.owner());
        assert_eq!(support_row.route_kind(), row.route_kind());
        match row.posture() {
            ForgeQueryLowerRuntimeCloseoutPosture::SeamEliminated => {
                assert_eq!(
                    support_row.posture(),
                    ForgeQueryLowerRuntimeSupportPosture::SeamEliminated
                );
            }
            ForgeQueryLowerRuntimeCloseoutPosture::DeferredNeighbor => {
                assert_eq!(
                    support_row.posture(),
                    ForgeQueryLowerRuntimeSupportPosture::Deferred
                );
            }
        }
    }

    hash_parts(&[
        hash_parts(
            &surface
                .envelopes()
                .iter()
                .map(|envelope| {
                    let support_row = support
                        .support_for(envelope.seam_key())
                        .expect("support row must exist");
                    format!(
                        "{}|{}|{}|{}",
                        envelope.seam_key().as_str(),
                        support_row.posture().as_str(),
                        support_row.authority_owner().as_str(),
                        support_row.route_kind().as_str()
                    )
                })
                .collect::<Vec<_>>(),
        ),
        hash_parts(
            &closeout
                .rows()
                .iter()
                .map(|row| {
                    let support_row = support
                        .support_for(row.seam_key())
                        .expect("support row must exist");
                    format!(
                        "{}|{}|{}",
                        row.seam_key().as_str(),
                        support_row.posture().as_str(),
                        row.certification_row()
                    )
                })
                .collect::<Vec<_>>(),
        ),
    ])
}

fn count_by_seam(seams: impl Iterator<Item = &'static str>) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for seam_key in seams {
        *counts.entry(seam_key).or_insert(0usize) += 1;
    }
    counts
}

fn count_receipts_by_seam(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> BTreeMap<&'static str, usize> {
    let request_by_digest: BTreeMap<_, _> = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_digest().to_string(),
                request.seam_key().as_str(),
            )
        })
        .collect();
    let mut counts = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_digest
            .get(receipt.request_digest())
            .unwrap_or_else(|| panic!("receipt request {} must exist", receipt.request_digest()));
        *counts.entry(*seam_key).or_insert(0usize) += 1;
    }
    counts
}

fn receipt_kind_by_seam(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> BTreeMap<&'static str, ForgeQueryLowerRuntimeBoundaryExecutionKind> {
    let request_by_digest: BTreeMap<_, _> = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_digest().to_string(),
                request.seam_key().as_str(),
            )
        })
        .collect();
    let mut kinds = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_digest
            .get(receipt.request_digest())
            .unwrap_or_else(|| panic!("receipt request {} must exist", receipt.request_digest()));
        kinds.insert(*seam_key, receipt.kind());
    }
    kinds
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::certification::surface::{
        forge_query_lower_runtime_representative_surface,
        ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    };

    #[test]
    fn required_phase_six_concrete_seams_are_enforced_hostilely() {
        let surface = forge_query_lower_runtime_representative_surface()
            .with_evidence_source_override(
                ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
                ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
            );

        let panic = std::panic::catch_unwind(|| required_concrete_seam_coverage_digest(&surface))
            .expect_err("required concrete seam fallback must fail acceptance");
        let message = panic_message(panic);

        assert!(message.contains("subscription-activation"));
    }

    fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
        if let Some(message) = payload.downcast_ref::<String>() {
            return message.clone();
        }
        if let Some(message) = payload.downcast_ref::<&str>() {
            return (*message).to_string();
        }
        "non-string panic payload".to_string()
    }
}
