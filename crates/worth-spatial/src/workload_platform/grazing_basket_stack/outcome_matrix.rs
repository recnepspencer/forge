use std::collections::BTreeSet;

use super::denial::{GrazingBasketStackDenial, GrazingBasketStackDenialKind};
use super::layer_scope::BasketLayerIndex;
use super::receipt::{GrazingBasketStackCounters, GrazingBasketStackReceipt};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GrazingBasketStackOutcomeKind {
    Admitted,
    Denied,
    IntegrityMismatch,
    Unsupported,
    NoOptions,
    PredicateUncertain,
}

impl GrazingBasketStackOutcomeKind {
    pub const REQUIRED: [Self; 6] = [
        Self::Admitted,
        Self::Denied,
        Self::IntegrityMismatch,
        Self::Unsupported,
        Self::NoOptions,
        Self::PredicateUncertain,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackOutcomeRow {
    kind: GrazingBasketStackOutcomeKind,
    denial_kind: Option<GrazingBasketStackDenialKind>,
    layer: Option<BasketLayerIndex>,
    evidence_digest: String,
    counters: GrazingBasketStackCounters,
    human_reason: String,
}

impl GrazingBasketStackOutcomeRow {
    pub fn admitted(receipt: &GrazingBasketStackReceipt) -> Self {
        Self {
            kind: GrazingBasketStackOutcomeKind::Admitted,
            denial_kind: None,
            layer: None,
            evidence_digest: receipt.stack_identity().to_string(),
            counters: receipt.counters(),
            human_reason: format!(
                "Grazing basket stack preserved {} open layers and {} strips per layer without closed-shell posture.",
                receipt.counters().total_layers(),
                receipt.counters().strips_per_layer()
            ),
        }
    }

    pub fn equivalent_transform_admitted(
        receipt: &GrazingBasketStackReceipt,
        layer: BasketLayerIndex,
    ) -> Result<Self, GrazingBasketStackDenial> {
        let layer_receipt = receipt.require_layer(layer)?;
        let variant = receipt.admit_equivalent_transform_variant(
            layer,
            layer_receipt.transform_pressure().ok_or_else(|| {
                GrazingBasketStackDenial::new(
                    GrazingBasketStackDenialKind::LabelOnlyMotion,
                    Some(layer),
                    Some(layer),
                    None,
                    1,
                    receipt.stack_identity().to_string(),
                    format!(
                        "{} has no transform pressure available for equivalent admission.",
                        layer.human_name()
                    ),
                )
            })?,
        )?;
        Ok(Self {
            kind: GrazingBasketStackOutcomeKind::Admitted,
            denial_kind: None,
            layer: Some(layer),
            evidence_digest: variant.variant_identity().to_string(),
            counters: receipt.counters().for_attack(1, 1),
            human_reason: format!(
                "Equivalent transform variant preserved {} with the same movement and rotation posture.",
                layer.human_name()
            ),
        })
    }

    pub fn from_denial(
        receipt: &GrazingBasketStackReceipt,
        denial: &GrazingBasketStackDenial,
    ) -> Self {
        Self {
            kind: outcome_kind(denial.kind()),
            denial_kind: Some(denial.kind()),
            layer: denial.target_layer().or(denial.source_layer()),
            evidence_digest: denial.evidence_digest().to_string(),
            counters: receipt
                .counters()
                .for_attack(denial.touched_layers(), denial.touched_layers()),
            human_reason: denial.human_reason().to_string(),
        }
    }

    pub fn kind(&self) -> GrazingBasketStackOutcomeKind {
        self.kind
    }

    pub fn denial_kind(&self) -> Option<GrazingBasketStackDenialKind> {
        self.denial_kind
    }

    pub fn layer(&self) -> Option<BasketLayerIndex> {
        self.layer
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn counters(&self) -> GrazingBasketStackCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackOutcomeMatrix {
    rows: Vec<GrazingBasketStackOutcomeRow>,
}

impl GrazingBasketStackOutcomeMatrix {
    pub fn from_receipt(
        receipt: &GrazingBasketStackReceipt,
    ) -> Result<Self, GrazingBasketStackDenial> {
        let first = BasketLayerIndex::new(0);
        let second = BasketLayerIndex::new(1);
        let third = BasketLayerIndex::new(2);
        let fourth = BasketLayerIndex::new(3);
        let boundary = receipt
            .layer(first)
            .expect("certified stack has first layer")
            .open_boundary()
            .clone();
        let rows = vec![
            GrazingBasketStackOutcomeRow::admitted(receipt),
            GrazingBasketStackOutcomeRow::equivalent_transform_admitted(receipt, second)?,
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_label_only_motion(first)
                    .expect_err("label-only motion denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_open_boundary_perturbation(first)
                    .expect_err("open-boundary perturbation denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_cross_layer_retained_replay(first, second)
                    .expect_err("cross-layer retained denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_cross_layer_projection_identity(second, fourth)
                    .expect_err("cross-layer projection denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_cross_layer_surface_support_smuggling(third, fourth)
                    .expect_err("cross-layer surface support denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_cross_layer_parity_lane_smuggling(first, third)
                    .expect_err("cross-layer parity lane denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_surface_support_smuggling(first, "non-plane analytic")
                    .expect_err("surface support denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_storm_extraction_smuggling(
                        first,
                        receipt.projected_workload_identity(),
                    )
                    .expect_err("storm smuggling denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_missing_boundary_evidence(first)
                    .expect_err("missing boundary denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_missing_projection_evidence(second)
                    .expect_err("missing projection denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_missing_retained_checkpoint_evidence(third)
                    .expect_err("missing retained checkpoint denial"),
            ),
            GrazingBasketStackOutcomeRow::from_denial(
                receipt,
                &receipt
                    .attempt_near_graze_predicate_pressure(first, boundary)
                    .expect_err("near-graze predicate uncertainty"),
            ),
        ];
        Self::from_rows(rows)
    }

    pub fn from_rows(
        rows: Vec<GrazingBasketStackOutcomeRow>,
    ) -> Result<Self, GrazingBasketStackDenial> {
        require_every_outcome_kind_present(&rows)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[GrazingBasketStackOutcomeRow] {
        &self.rows
    }
}

fn outcome_kind(kind: GrazingBasketStackDenialKind) -> GrazingBasketStackOutcomeKind {
    match kind {
        GrazingBasketStackDenialKind::LabelOnlyMotion
        | GrazingBasketStackDenialKind::OpenBoundaryPerturbation => {
            GrazingBasketStackOutcomeKind::Denied
        }
        GrazingBasketStackDenialKind::CrossLayerRetainedReplay
        | GrazingBasketStackDenialKind::CrossLayerProjectionIdentity
        | GrazingBasketStackDenialKind::SurfaceSupportSmuggling
        | GrazingBasketStackDenialKind::CrossLayerParityLane
        | GrazingBasketStackDenialKind::StormExtractionSmuggling
        | GrazingBasketStackDenialKind::FalseClosure
        | GrazingBasketStackDenialKind::WholeStackBroadening => {
            GrazingBasketStackOutcomeKind::IntegrityMismatch
        }
        GrazingBasketStackDenialKind::WrongTopologyPattern
        | GrazingBasketStackDenialKind::WrongTopologyPosture
        | GrazingBasketStackDenialKind::UnsupportedLayerProfile
        | GrazingBasketStackDenialKind::UnsupportedSurfaceFamily => {
            GrazingBasketStackOutcomeKind::Unsupported
        }
        GrazingBasketStackDenialKind::PredicateUncertain => {
            GrazingBasketStackOutcomeKind::PredicateUncertain
        }
        GrazingBasketStackDenialKind::MissingPlatformEvidence
        | GrazingBasketStackDenialKind::MissingLayerEvidence
        | GrazingBasketStackDenialKind::MissingProjectionEvidence
        | GrazingBasketStackDenialKind::MissingRetainedCheckpointEvidence => {
            GrazingBasketStackOutcomeKind::NoOptions
        }
    }
}

fn require_every_outcome_kind_present(
    rows: &[GrazingBasketStackOutcomeRow],
) -> Result<(), GrazingBasketStackDenial> {
    let seen = rows.iter().map(|row| row.kind()).collect::<BTreeSet<_>>();
    for required in GrazingBasketStackOutcomeKind::REQUIRED {
        if !seen.contains(&required) {
            return Err(GrazingBasketStackDenial::new(
                GrazingBasketStackDenialKind::MissingLayerEvidence,
                None,
                None,
                None,
                0,
                "grazing-basket-stack-outcome-matrix",
                format!("Grazing basket stack outcome matrix is missing {required:?}."),
            ));
        }
    }
    Ok(())
}
