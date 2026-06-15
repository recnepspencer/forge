use worth_primitives::{truth_digest_parts, TruthDigestScope};

use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanDeclarationReceipt, PlanarBooleanEntryBasis,
    PlanarBooleanOutcomeReceipt,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[path = "public_api_planar_boolean_7_0_closeout/anti_theatre_guards.rs"]
mod anti_theatre_guards;
#[path = "public_api_planar_boolean_7_0_closeout/boundary_claims.rs"]
mod boundary_claims;
#[path = "public_api_planar_boolean_7_0_closeout/proof_packets.rs"]
mod proof_packets;

use proof_packets::{PlanarBoolean7_0AntiTheatreProof, PlanarBoolean7_0EvidenceProof};

const M7_0_ENTRY_BOUNDARY_HANDOFF_NOTE: &str =
    "Later boolean milestones must consume the registered 7.0 entry boundary instead of rebuilding it.";

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBoolean7_0ProofRow {
    kind: PlanarBoolean7_0ProofRowKind,
    identity: String,
    source: PlanarBoolean7_0ProofSource,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlanarBoolean7_0ProofSource {
    Real,
    Synthetic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlanarBoolean7_0ProofRowKind {
    DeclarationFamily,
    EntryBasis,
    OutcomeProvenance,
    CatalogRecipe,
    EvidenceStage,
    AntiTheatre,
    FutureExecutionStage(WorkloadEvidenceStage),
}

impl PlanarBoolean7_0ProofRowKind {
    fn human_name(self) -> &'static str {
        match self {
            Self::DeclarationFamily => "boolean declaration-family proof",
            Self::EntryBasis => "boolean entry-basis proof",
            Self::OutcomeProvenance => "boolean outcome/provenance proof",
            Self::CatalogRecipe => "boolean catalog recipe proof",
            Self::EvidenceStage => "boolean evidence-stage proof",
            Self::AntiTheatre => "boolean anti-theatre proof",
            Self::FutureExecutionStage(_) => "future boolean execution-stage proof",
        }
    }
}

impl PlanarBoolean7_0ProofRow {
    fn real(kind: PlanarBoolean7_0ProofRowKind, identity: String) -> Self {
        Self {
            kind,
            identity,
            source: PlanarBoolean7_0ProofSource::Real,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisteredPlanarBoolean7_0EntryBoundary {
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisteredPlanarBoolean7_0Boundary {
    entry_boundary: RegisteredPlanarBoolean7_0EntryBoundary,
    handoff_note: &'static str,
}

impl RegisteredPlanarBoolean7_0Boundary {
    fn entry_boundary(&self) -> &RegisteredPlanarBoolean7_0EntryBoundary {
        &self.entry_boundary
    }

    fn handoff_note(&self) -> &'static str {
        self.handoff_note
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LaterPlanarBooleanMilestoneBoundary {
    consumed_entry_boundary_digest: String,
    handoff_note: &'static str,
}

impl LaterPlanarBooleanMilestoneBoundary {
    fn try_from_registered_closeout(
        registered: Option<&RegisteredPlanarBoolean7_0Boundary>,
    ) -> Result<Self, PlanarBoolean7_0CloseoutError> {
        let registered =
            registered.ok_or(PlanarBoolean7_0CloseoutError::MissingRegisteredEntryBoundary)?;
        Ok(Self {
            consumed_entry_boundary_digest: registered.entry_boundary().digest.clone(),
            handoff_note: registered.handoff_note(),
        })
    }

    fn consumed_entry_boundary_digest(&self) -> &str {
        &self.consumed_entry_boundary_digest
    }

    fn handoff_note(&self) -> &'static str {
        self.handoff_note
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBoolean7_0CloseoutBundle {
    rows: Vec<PlanarBoolean7_0ProofRow>,
    entry_boundary_digest: Option<String>,
    boundary_claims: boundary_claims::PlanarBoolean7_0BoundaryClaims,
}

impl PlanarBoolean7_0CloseoutBundle {
    fn collect() -> Self {
        Self {
            rows: Vec::new(),
            entry_boundary_digest: None,
            boundary_claims: boundary_claims::PlanarBoolean7_0BoundaryClaims::default(),
        }
    }

    fn with_declaration_family_proof(
        mut self,
        declaration: &PlanarBooleanDeclarationReceipt,
    ) -> Self {
        self.boundary_claims
            .record_readiness_basis_digest(declaration.readiness_basis_digest());
        self.boundary_claims
            .record_declaration_digest(declaration.query_declaration_digest());
        self.boundary_claims
            .record_operand_pair_identity(declaration.operand_pair_identity().as_str());
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::DeclarationFamily,
            declaration.query_declaration_digest().to_string(),
        ));
        self
    }

    fn with_entry_basis_proof(mut self, basis: &PlanarBooleanEntryBasis) -> Self {
        self.boundary_claims
            .record_readiness_basis_digest(basis.readiness_receipt_identity());
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::EntryBasis,
            entry_basis_proof_identity(basis),
        ));
        self
    }

    fn with_outcome_and_provenance_proof(mut self, outcome: &PlanarBooleanOutcomeReceipt) -> Self {
        self.boundary_claims
            .record_readiness_basis_digest(outcome.declaration().readiness_basis_digest());
        if let Some(blocker) = outcome.blocker_provenance() {
            self.boundary_claims
                .record_blocker_digest(blocker.provenance_digest());
        }
        let outcome_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-7-0-outcome-provenance".to_string(),
                format!("kind:{:?}", outcome.kind()),
                format!(
                    "declaration:{}",
                    outcome.declaration().query_declaration_digest()
                ),
                format!("support:{}", outcome.support().query_support_digest()),
                format!(
                    "blocker:{}",
                    outcome
                        .blocker_provenance()
                        .map(
                            |receipt: &worth_spatial::facade::blocker_provenance::WorkloadBlockerProvenanceReceipt| {
                                receipt.provenance_digest()
                            },
                        )
                        .unwrap_or("none")
                ),
            ],
        );
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::OutcomeProvenance,
            outcome_identity,
        ));
        self
    }

    fn with_catalog_recipe_proof(mut self, pair: &BuiltBooleanOperandPairRecipe) -> Self {
        self.boundary_claims
            .record_operand_pair_identity(pair.operand_pair_identity());
        let identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-7-0-catalog-recipe".to_string(),
                format!("recipe:{}", pair.recipe().query_key()),
                format!(
                    "declaration:{}",
                    pair.declaration().query_declaration_digest()
                ),
                format!("support:{}", pair.support().query_support_digest()),
                format!("pair:{}", pair.operand_pair_identity()),
            ],
        );
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::CatalogRecipe,
            identity,
        ));
        self
    }

    fn with_evidence_stage_proof(mut self, proof: &PlanarBoolean7_0EvidenceProof) -> Self {
        self.entry_boundary_digest = Some(proof.entry_boundary_digest().to_string());
        self.boundary_claims
            .record_readiness_basis_digest(proof.readiness_basis_digest());
        self.boundary_claims
            .record_declaration_digest(proof.declaration_digest());
        self.boundary_claims
            .record_operand_pair_identity(proof.operand_pair_identity());
        self.boundary_claims
            .record_blocker_digest(proof.blocker_digest());
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::EvidenceStage,
            proof.proof_digest().to_string(),
        ));
        self
    }

    fn with_anti_theatre_proof(mut self, proof: &PlanarBoolean7_0AntiTheatreProof) -> Self {
        self.boundary_claims
            .record_blocker_digest(proof.blocker_digest());
        self.boundary_claims
            .record_pair_construction_digest(proof.pair_construction_digest());
        self.rows.push(PlanarBoolean7_0ProofRow::real(
            PlanarBoolean7_0ProofRowKind::AntiTheatre,
            proof.proof_digest().to_string(),
        ));
        self
    }

    fn register(self) -> Result<RegisteredPlanarBoolean7_0Boundary, PlanarBoolean7_0CloseoutError> {
        let entry_boundary_digest =
            self.entry_boundary_digest
                .ok_or(PlanarBoolean7_0CloseoutError::MissingProofRow(
                    PlanarBoolean7_0ProofRowKind::EvidenceStage,
                ))?;
        require_row_integrity(&self.rows)?;
        self.boundary_claims.validate()?;
        Ok(RegisteredPlanarBoolean7_0Boundary {
            entry_boundary: RegisteredPlanarBoolean7_0EntryBoundary {
                digest: entry_boundary_digest,
            },
            handoff_note: M7_0_ENTRY_BOUNDARY_HANDOFF_NOTE,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlanarBoolean7_0CloseoutError {
    MissingProofRow(PlanarBoolean7_0ProofRowKind),
    DuplicateProofRow(PlanarBoolean7_0ProofRowKind),
    SyntheticProofRow(PlanarBoolean7_0ProofRowKind),
    ForbiddenFutureExecutionRow(WorkloadEvidenceStage),
    MissingRegisteredEntryBoundary,
    InvalidAntiTheatreGuard(&'static str),
    MismatchedProofBoundary(&'static str),
}

impl PlanarBoolean7_0CloseoutError {
    fn human_reason(self) -> String {
        match self {
            Self::MissingProofRow(kind) => format!("7.0 closeout is missing {}", kind.human_name()),
            Self::DuplicateProofRow(kind) => {
                format!("7.0 closeout registered duplicate {}", kind.human_name())
            }
            Self::SyntheticProofRow(kind) => {
                format!("7.0 closeout cannot accept synthetic {}", kind.human_name())
            }
            Self::ForbiddenFutureExecutionRow(stage) => format!(
                "7.0 closeout cannot register future execution stage {}",
                stage.human_name()
            ),
            Self::MissingRegisteredEntryBoundary => {
                "later milestones must consume the registered 7.0 entry boundary".to_string()
            }
            Self::InvalidAntiTheatreGuard(guard) => {
                format!("7.0 closeout anti-theatre proof requires a real {guard} guard")
            }
            Self::MismatchedProofBoundary(boundary) => {
                format!("7.0 closeout proof rows disagree about {boundary}")
            }
        }
    }
}

fn entry_basis_proof_identity(basis: &PlanarBooleanEntryBasis) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-7-0-entry-basis".to_string(),
            format!("readiness:{}", basis.readiness_receipt_identity()),
            format!("coverage:{}", basis.stage_coverage().coverage_digest()),
            format!("query:{}", basis.query_declaration_digest()),
            format!("envelope:{}", basis.query_envelope_digest()),
            format!("handle:{}", basis.query_handle_digest()),
        ],
    )
}

fn require_row_integrity(
    rows: &[PlanarBoolean7_0ProofRow],
) -> Result<(), PlanarBoolean7_0CloseoutError> {
    for required in [
        PlanarBoolean7_0ProofRowKind::DeclarationFamily,
        PlanarBoolean7_0ProofRowKind::EntryBasis,
        PlanarBoolean7_0ProofRowKind::OutcomeProvenance,
        PlanarBoolean7_0ProofRowKind::CatalogRecipe,
        PlanarBoolean7_0ProofRowKind::EvidenceStage,
        PlanarBoolean7_0ProofRowKind::AntiTheatre,
    ] {
        let matching: Vec<&PlanarBoolean7_0ProofRow> =
            rows.iter().filter(|row| row.kind == required).collect();
        if matching.is_empty() {
            return Err(PlanarBoolean7_0CloseoutError::MissingProofRow(required));
        }
        if matching.len() > 1 {
            return Err(PlanarBoolean7_0CloseoutError::DuplicateProofRow(required));
        }
        if matching[0].source != PlanarBoolean7_0ProofSource::Real {
            return Err(PlanarBoolean7_0CloseoutError::SyntheticProofRow(required));
        }
    }

    for row in rows {
        if let PlanarBoolean7_0ProofRowKind::FutureExecutionStage(stage) = row.kind {
            return Err(PlanarBoolean7_0CloseoutError::ForbiddenFutureExecutionRow(
                stage,
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[cfg(test)]
mod tests {
    #[path = "boundary_mismatches.rs"]
    mod boundary_mismatches;
    #[path = "downstream_consumption.rs"]
    mod downstream_consumption;
    #[path = "required_rows.rs"]
    mod required_rows;
    #[path = "support.rs"]
    mod support;
    #[path = "synthetic_and_duplicate_denials.rs"]
    mod synthetic_and_duplicate_denials;
}
