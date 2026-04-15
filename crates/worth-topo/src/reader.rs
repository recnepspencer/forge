use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    VerifiedTopologyCommit, WorthTopologyReadArtifact,
};

use crate::facade::{
    build_derived_read_diagnostics,
    build_derived_equivalence_contract, build_topology_read_artifact, certify_topology_view,
    interpret_topology_view,
    validate_interpreted_topology,
};
use crate::diagnostics::WorthDerivedReadDiagnostics;
use crate::interpretation::InterpretedTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::parity::WorthDerivedEquivalenceContractReport;
use crate::materialization::{WorthTopologyMaterializationError, WorthTopologyMaterializer};
use crate::validators::{DerivedTopologyValidationReport, WorthTopologyValidationError};

#[derive(Debug)]
pub enum WorthTopologyReadError {
    ReadView(String),
    Materialization(WorthTopologyMaterializationError),
    Validation(WorthTopologyValidationError),
}

impl std::fmt::Display for WorthTopologyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadView(error) => write!(f, "read view: {error}"),
            Self::Materialization(error) => write!(f, "materialization: {error}"),
            Self::Validation(error) => write!(f, "validation: {error}"),
        }
    }
}

impl std::error::Error for WorthTopologyReadError {}

impl From<WorthTopologyMaterializationError> for WorthTopologyReadError {
    fn from(value: WorthTopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<WorthTopologyValidationError> for WorthTopologyReadError {
    fn from(value: WorthTopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Clone)]
pub struct StagedWorthTopologyRead {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
}

impl StagedWorthTopologyRead {
    pub(crate) fn new(
        materialized: MaterializedTopologyView,
        interpreted: InterpretedTopologyView,
        validation: DerivedTopologyValidationReport,
    ) -> Self {
        Self {
            materialized,
            interpreted,
            validation,
        }
    }

    pub fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub fn validation(&self) -> &DerivedTopologyValidationReport {
        &self.validation
    }
}

pub(crate) fn stage_topology_read_from_view(
    read_view: &RelationalReadView,
) -> Result<StagedWorthTopologyRead, WorthTopologyReadError> {
    let materialized = WorthTopologyMaterializer::materialize_from_truth(read_view)?;
    let interpreted = interpret_topology_view(&materialized);
    let validation = validate_interpreted_topology(&materialized, &interpreted)?;
    Ok(StagedWorthTopologyRead::new(
        materialized,
        interpreted,
        validation,
    ))
}

pub struct WorthTopologyReader<'a> {
    runtime: &'a RelationalRuntime,
}

impl<'a> WorthTopologyReader<'a> {
    pub fn new(runtime: &'a RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn read_basis_from_persisted_truth(
        &self,
        persisted_truth: &PersistedTopologyTruthBatch,
    ) -> DerivedTopologyReadBasis {
        DerivedTopologyReadBasis::from_persisted_truth(persisted_truth)
    }

    pub fn read_basis_from_verified_commit(
        &self,
        verified: &VerifiedTopologyCommit,
    ) -> DerivedTopologyReadBasis {
        verified.read_basis.clone()
    }

    pub fn read_view(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<RelationalReadView, WorthTopologyReadError> {
        self.runtime
            .read_truth()
            .read_snapshot(basis.snapshot())
            .ok_or_else(|| {
                WorthTopologyReadError::ReadView(format!(
                    "worth topology reader could not open snapshot {:?}",
                    basis.snapshot()
                ))
            })
    }

    pub fn read_artifact(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTopologyReadArtifact, WorthTopologyReadError> {
        let staged = self.stage(basis)?;
        Ok(build_topology_read_artifact(basis, staged.interpreted()))
    }

    pub fn interpret(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<CertifiedTopologyInterpretation, WorthTopologyReadError> {
        let staged = self.stage(basis)?;
        Ok(certify_topology_view(basis.clone(), staged.interpreted()))
    }

    pub fn materialize(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<MaterializedTopologyView, WorthTopologyReadError> {
        let read_view = self.read_view(basis)?;
        Ok(WorthTopologyMaterializer::materialize_from_truth(&read_view)?)
    }

    pub fn interpret_materialized(
        &self,
        materialized: &MaterializedTopologyView,
    ) -> InterpretedTopologyView {
        interpret_topology_view(materialized)
    }

    pub fn validate_interpreted(
        &self,
        materialized: &MaterializedTopologyView,
        interpreted: &InterpretedTopologyView,
    ) -> Result<DerivedTopologyValidationReport, WorthTopologyReadError> {
        Ok(validate_interpreted_topology(materialized, interpreted)?)
    }

    pub fn equivalence_contract(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthDerivedEquivalenceContractReport, WorthTopologyReadError> {
        let staged = self.stage(basis)?;
        Ok(build_derived_equivalence_contract(
            basis,
            staged.materialized(),
            staged.interpreted(),
            staged.validation(),
        ))
    }

    pub fn diagnostics(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthDerivedReadDiagnostics, WorthTopologyReadError> {
        let staged = self.stage(basis)?;
        Ok(build_derived_read_diagnostics(
            basis,
            staged.materialized(),
            staged.interpreted(),
            staged.validation(),
        ))
    }

    pub(crate) fn stage(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<StagedWorthTopologyRead, WorthTopologyReadError> {
        let read_view = self.read_view(basis)?;
        stage_topology_read_from_view(&read_view)
    }
}

#[cfg(test)]
mod tests {
    use worth_schema::facade::{
        seed_minimal_topology, seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
    };

    use crate::facade::{worth_milestone_one_runtime_builder, WorthTopologyReader};

    #[test]
    fn reader_builds_artifact_and_interpretation_from_persisted_truth() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let seeded = seed_minimal_topology(&mut runtime, "reader-seeded")
            .expect("seed minimal topology");

        let reader = WorthTopologyReader::new(&runtime);
        let basis = reader.read_basis_from_persisted_truth(&seeded.persisted_truth);
        let artifact = reader.read_artifact(&basis).expect("read artifact");
        let interpretation = reader.interpret(&basis).expect("interpretation");

        assert_eq!(artifact.snapshot, seeded.snapshot);
        assert_eq!(artifact.interpretations, interpretation.interpretations);
    }

    #[test]
    fn reader_reuses_verified_commit_basis_for_admitted_primitive() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive(
            &mut runtime,
            "reader-verified",
            &WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        )
        .expect("verified primitive");

        let reader = WorthTopologyReader::new(&runtime);
        let basis = reader.read_basis_from_verified_commit(&verified);
        let interpretation = reader.interpret(&basis).expect("interpretation");

        assert!(interpretation
            .interpretations
            .wires
            .iter()
            .any(|wire| wire.branch_vertex_ids.len() == 1));
    }
}
