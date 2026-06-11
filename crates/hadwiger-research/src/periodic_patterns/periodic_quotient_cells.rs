use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerArtifactReference;
use crate::mathematical_verification::ExactRational;
use crate::tiling_geometry::TilingCell;

use super::lattice_basis::{PeriodicLatticeBasis, PeriodicLatticeVector};
use super::replay_errors::GeneratedPatternReplayShapeError;
use super::replay_errors::{require_replay_non_empty, GeneratedPatternReplayError};
use super::translation_rules::PeriodicTranslationRule;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientCell {
    core: HadwigerArtifactCore,
    quotient_id: String,
    source_cell_reference: HadwigerArtifactReference,
    lattice_basis: PeriodicLatticeBasis,
    translation_rules: Vec<PeriodicTranslationRule>,
    source_tile_count: usize,
}

impl PeriodicQuotientCell {
    pub fn builder(
        quotient_id: impl Into<String>,
        source_cell_reference: HadwigerArtifactReference,
    ) -> PeriodicQuotientCellBuilder {
        PeriodicQuotientCellBuilder {
            quotient_id: quotient_id.into(),
            source_cell_reference,
            lattice_vectors: Vec::new(),
            translation_rules: Vec::new(),
            source_cell: None,
        }
    }

    pub fn quotient_id(&self) -> &str {
        &self.quotient_id
    }

    pub fn source_cell_reference(&self) -> &HadwigerArtifactReference {
        &self.source_cell_reference
    }

    pub fn lattice_basis(&self) -> &PeriodicLatticeBasis {
        &self.lattice_basis
    }

    pub fn translation_rules(&self) -> &[PeriodicTranslationRule] {
        &self.translation_rules
    }

    pub fn source_tile_count(&self) -> usize {
        self.source_tile_count
    }

    pub fn stable_token(&self) -> String {
        quotient_stable_token(
            &self.quotient_id,
            &self.source_cell_reference,
            &self.lattice_basis,
            &self.translation_rules,
        )
    }
}

impl_hadwiger_artifact!(PeriodicQuotientCell, core);

#[derive(Clone, Debug)]
pub struct PeriodicQuotientCellBuilder {
    quotient_id: String,
    source_cell_reference: HadwigerArtifactReference,
    lattice_vectors: Vec<PeriodicLatticeVector>,
    translation_rules: Vec<PeriodicTranslationRule>,
    source_cell: Option<TilingCell>,
}

impl PeriodicQuotientCellBuilder {
    pub fn with_source_cell(mut self, source_cell: TilingCell) -> Self {
        self.source_cell = Some(source_cell);
        self
    }

    pub fn with_lattice_basis_vector(
        mut self,
        vector_id: impl Into<String>,
        dx: ExactRational,
        dy: ExactRational,
    ) -> Result<Self, GeneratedPatternReplayError> {
        let vector = PeriodicLatticeVector::new(vector_id, dx, dy)?;
        reject_duplicate_vector(&self.lattice_vectors, vector.vector_id())?;
        self.lattice_vectors.push(vector);
        Ok(self)
    }

    pub fn with_translation_rule(
        mut self,
        rule: PeriodicTranslationRule,
    ) -> Result<Self, GeneratedPatternReplayError> {
        reject_duplicate_rule(&self.translation_rules, rule.rule_id())?;
        self.translation_rules.push(rule);
        Ok(self)
    }

    pub fn finish(self) -> Result<PeriodicQuotientCell, GeneratedPatternReplayError> {
        let quotient_id = require_replay_non_empty(self.quotient_id, "periodic_quotient_id")?;
        if self.translation_rules.is_empty() {
            return Err(GeneratedPatternReplayShapeError::EmptyField {
                field: "translation_rules",
            }
            .into());
        }
        let source_cell = self
            .source_cell
            .as_ref()
            .ok_or(GeneratedPatternReplayShapeError::MissingSourceCell)?;
        let lattice_basis = PeriodicLatticeBasis::new(self.lattice_vectors)?;
        let mut translation_rules = self.translation_rules;
        translation_rules.sort();
        validate_translation_rules(&lattice_basis, source_cell, &translation_rules)?;
        let stable_token = quotient_stable_token(
            &quotient_id,
            &self.source_cell_reference,
            &lattice_basis,
            &translation_rules,
        );
        let source_tile_count = source_cell.tile_count();
        let core = artifact_core(
            HadwigerArtifactKind::PeriodicQuotientCell,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "periodic_quotient_cell".to_string(),
            },
            vec![self.source_cell_reference.clone()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "forge.hadwiger.periodic_quotient_cell.v1",
                ),
                HadwigerArtifactPayloadEntry::text("quotient", stable_token),
            ],
        )?;
        Ok(PeriodicQuotientCell {
            core,
            quotient_id,
            source_cell_reference: self.source_cell_reference,
            lattice_basis,
            translation_rules,
            source_tile_count,
        })
    }
}

fn validate_translation_rules(
    lattice_basis: &PeriodicLatticeBasis,
    source_cell: &TilingCell,
    rules: &[PeriodicTranslationRule],
) -> Result<(), GeneratedPatternReplayError> {
    for rule in rules {
        lattice_basis.require_vector(rule.lattice_vector_id())?;
        source_cell.require_tile(rule.source_tile_id())?;
        source_cell.require_tile(rule.target_tile_id())?;
    }
    Ok(())
}

fn reject_duplicate_vector(
    vectors: &[PeriodicLatticeVector],
    vector_id: &str,
) -> Result<(), GeneratedPatternReplayShapeError> {
    if vectors.iter().any(|vector| vector.vector_id() == vector_id) {
        Err(GeneratedPatternReplayShapeError::DuplicateIdentity {
            field: "lattice_vector_id",
            value: vector_id.to_string(),
        })
    } else {
        Ok(())
    }
}

fn reject_duplicate_rule(
    rules: &[PeriodicTranslationRule],
    rule_id: &str,
) -> Result<(), GeneratedPatternReplayShapeError> {
    if rules.iter().any(|rule| rule.rule_id() == rule_id) {
        Err(GeneratedPatternReplayShapeError::DuplicateIdentity {
            field: "translation_rule_id",
            value: rule_id.to_string(),
        })
    } else {
        Ok(())
    }
}

fn quotient_stable_token(
    quotient_id: &str,
    source_cell_reference: &HadwigerArtifactReference,
    lattice_basis: &PeriodicLatticeBasis,
    translation_rules: &[PeriodicTranslationRule],
) -> String {
    let rules = translation_rules
        .iter()
        .map(PeriodicTranslationRule::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "{}:{}:{}:{}",
        quotient_id,
        source_cell_reference.stable_token(),
        lattice_basis.stable_token(),
        rules
    )
}
