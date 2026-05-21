use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};
use forge_query::facade::ForgeQueryWorkspace;
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use crate::construction::certification::corpus::{
    prepare_primitive_construction_compound_milestone_closeout_report,
    prepare_primitive_construction_simplex_realization_exhaustion_witness_report,
    prepare_primitive_construction_simplex_realization_strategy_ladder_report,
    required_simplex_exhaustion_witness_kinds, required_simplex_ladder_scenarios,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundMilestoneCloseoutReport,
    PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
    PrimitiveConstructionSimplexRealizationLadderReportError,
    PrimitiveConstructionSimplexRealizationStrategyLadderReport,
};
use crate::construction::certification::profile::{
    prepare_primitive_construction_policy_pressure_report_bundle,
    PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureReportBundleError,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::{
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
};

type PhaseFiveSixProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedPhaseFiveSixArtifact = Artifact<
    VerifiedPhaseFiveSixCloseoutPhase,
    PrimitiveConstructionVerifiedPhaseFiveSixCloseoutPayload,
    Proof<PhaseFiveSixCloseoutProven, PhaseFiveSixCloseoutProofAuthority>,
    PhaseFiveSixProofBasis,
>;

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionPhaseFiveSixCloseoutRegistry {
    required_simplex_scenarios: &'static [&'static str],
    required_exhaustion_kinds: &'static [PrimitiveRealizationExhaustionWitnessKind],
    registry_digest: String,
}

impl PrimitiveConstructionPhaseFiveSixCloseoutRegistry {
    fn required_simplex_scenarios(&self) -> &'static [&'static str] {
        self.required_simplex_scenarios
    }

    fn required_exhaustion_kinds(&self) -> &'static [PrimitiveRealizationExhaustionWitnessKind] {
        self.required_exhaustion_kinds
    }
}

fn phase_five_six_registry() -> PrimitiveConstructionPhaseFiveSixCloseoutRegistry {
    let required_simplex_scenarios = required_simplex_ladder_scenarios();
    let required_exhaustion_kinds = required_simplex_exhaustion_witness_kinds();
    let registry_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &required_simplex_scenarios
            .iter()
            .map(|scenario| scenario.to_string())
            .chain(
                required_exhaustion_kinds
                    .iter()
                    .map(|kind| format!("{kind:?}")),
            )
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionPhaseFiveSixCloseoutRegistry {
        required_simplex_scenarios,
        required_exhaustion_kinds,
        registry_digest,
    }
}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionPhaseFiveSixCloseoutAssembly {
    compound_closeout: PrimitiveConstructionCompoundMilestoneCloseoutReport,
    simplex_ladder: PrimitiveConstructionSimplexRealizationStrategyLadderReport,
    simplex_exhaustion: PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
    policy_pressure: PrimitiveConstructionPolicyPressureReportBundle,
}

impl PrimitiveConstructionPhaseFiveSixCloseoutAssembly {
    fn new(
        compound_closeout: PrimitiveConstructionCompoundMilestoneCloseoutReport,
        simplex_ladder: PrimitiveConstructionSimplexRealizationStrategyLadderReport,
        simplex_exhaustion: PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
        policy_pressure: PrimitiveConstructionPolicyPressureReportBundle,
    ) -> Self {
        Self {
            compound_closeout,
            simplex_ladder,
            simplex_exhaustion,
            policy_pressure,
        }
    }
}

pub(crate) struct VerifiedPhaseFiveSixCloseoutPhase;
impl PhaseMarker for VerifiedPhaseFiveSixCloseoutPhase {}

pub(crate) struct PhaseFiveSixCloseoutProven;
impl ProofMarker for PhaseFiveSixCloseoutProven {}

pub(crate) struct PhaseFiveSixCloseoutProofAuthority;
impl AuthorityMarker for PhaseFiveSixCloseoutProofAuthority {}
impl AuthorityProves<PhaseFiveSixCloseoutProven> for PhaseFiveSixCloseoutProofAuthority {}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedPhaseFiveSixCloseoutPayload {
    registry: PrimitiveConstructionPhaseFiveSixCloseoutRegistry,
    assembly: PrimitiveConstructionPhaseFiveSixCloseoutAssembly,
    report_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch {
    CompoundCloseoutUnverified,
    RequiredSimplexScenarioMissing,
    SimplexExhaustionInventoryDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure {
    registry: Vec<String>,
    compound_closeout: PrimitiveConstructionCompoundMilestoneCloseoutReport,
    simplex_ladder: PrimitiveConstructionSimplexRealizationStrategyLadderReport,
    simplex_exhaustion: PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
    policy_pressure: PrimitiveConstructionPolicyPressureReportBundle,
    missing_simplex_scenarios: Vec<String>,
    missing_exhaustion_kinds: Vec<PrimitiveRealizationExhaustionWitnessKind>,
    mismatches: Vec<PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure {
    pub fn compound_closeout(&self) -> &PrimitiveConstructionCompoundMilestoneCloseoutReport {
        &self.compound_closeout
    }

    pub fn simplex_ladder(&self) -> &PrimitiveConstructionSimplexRealizationStrategyLadderReport {
        &self.simplex_ladder
    }

    pub fn simplex_exhaustion(
        &self,
    ) -> &PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
        &self.simplex_exhaustion
    }

    pub fn policy_pressure(&self) -> &PrimitiveConstructionPolicyPressureReportBundle {
        &self.policy_pressure
    }

    pub fn missing_simplex_scenarios(&self) -> &[String] {
        &self.missing_simplex_scenarios
    }

    pub fn missing_exhaustion_kinds(&self) -> &[PrimitiveRealizationExhaustionWitnessKind] {
        &self.missing_exhaustion_kinds
    }

    pub fn mismatches(&self) -> &[PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch] {
        &self.mismatches
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub struct PrimitiveConstructionPhaseFiveSixCloseoutReport(VerifiedPhaseFiveSixArtifact);

impl Clone for PrimitiveConstructionPhaseFiveSixCloseoutReport {
    fn clone(&self) -> Self {
        Self::from_parts(
            self.0.payload().registry.clone(),
            self.0.payload().assembly.clone(),
        )
    }
}

impl std::fmt::Debug for PrimitiveConstructionPhaseFiveSixCloseoutReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionPhaseFiveSixCloseoutReport")
            .field("report_digest", &self.report_digest())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionPhaseFiveSixCloseoutReport {
    fn eq(&self, other: &Self) -> bool {
        self.0.payload().registry == other.0.payload().registry
            && self.0.payload().assembly == other.0.payload().assembly
    }
}

impl PrimitiveConstructionPhaseFiveSixCloseoutReport {
    fn from_parts(
        registry: PrimitiveConstructionPhaseFiveSixCloseoutRegistry,
        assembly: PrimitiveConstructionPhaseFiveSixCloseoutAssembly,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(PhaseFiveSixCloseoutProofAuthority);
        let report_digest = closeout_digest(&registry, &assembly);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedPhaseFiveSixCloseoutPayload {
                registry,
                assembly,
                report_digest,
            },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn compound_closeout(&self) -> &PrimitiveConstructionCompoundMilestoneCloseoutReport {
        &self.0.payload().assembly.compound_closeout
    }

    pub fn simplex_ladder(&self) -> &PrimitiveConstructionSimplexRealizationStrategyLadderReport {
        &self.0.payload().assembly.simplex_ladder
    }

    pub fn simplex_exhaustion(
        &self,
    ) -> &PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
        &self.0.payload().assembly.simplex_exhaustion
    }

    pub fn policy_pressure(&self) -> &PrimitiveConstructionPolicyPressureReportBundle {
        &self.0.payload().assembly.policy_pressure
    }

    pub fn required_simplex_scenarios(&self) -> &'static [&'static str] {
        self.0.payload().registry.required_simplex_scenarios()
    }

    pub fn required_exhaustion_kinds(
        &self,
    ) -> &'static [PrimitiveRealizationExhaustionWitnessKind] {
        self.0.payload().registry.required_exhaustion_kinds()
    }

    pub fn report_digest(&self) -> &str {
        &self.0.payload().report_digest
    }

    pub fn proof_grade(&self) -> PrimitiveConstructionProofGrade {
        PrimitiveConstructionProofGrade::MilestoneCloseout
    }

    pub fn proof_subject(&self) -> PrimitiveConstructionProofSubject {
        PrimitiveConstructionProofSubject::PhaseFiveSixCloseout
    }
}

fn verify_closeout(
    assembly: PrimitiveConstructionPhaseFiveSixCloseoutAssembly,
) -> Result<
    PrimitiveConstructionPhaseFiveSixCloseoutReport,
    PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure,
> {
    let registry = phase_five_six_registry();
    let missing_simplex_scenarios = registry
        .required_simplex_scenarios()
        .iter()
        .filter(|scenario_id| assembly.simplex_ladder.row_for(scenario_id).is_none())
        .map(|scenario_id| (*scenario_id).to_string())
        .collect::<Vec<_>>();
    let missing_exhaustion_kinds = registry
        .required_exhaustion_kinds()
        .iter()
        .copied()
        .filter(|kind| assembly.simplex_exhaustion.row_for(*kind).is_none())
        .collect::<Vec<_>>();
    let mut mismatches = Vec::new();
    if !assembly.compound_closeout.closeout_gate_verified() {
        mismatches.push(
            PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch::CompoundCloseoutUnverified,
        );
    }
    if !missing_simplex_scenarios.is_empty() {
        mismatches.push(
            PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch::RequiredSimplexScenarioMissing,
        );
    }
    if !missing_exhaustion_kinds.is_empty() {
        mismatches.push(
            PrimitiveConstructionPhaseFiveSixCloseoutVerificationMismatch::SimplexExhaustionInventoryDrift,
        );
    }
    if mismatches.is_empty() {
        return Ok(PrimitiveConstructionPhaseFiveSixCloseoutReport::from_parts(
            registry, assembly,
        ));
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            registry.registry_digest.clone(),
            assembly.compound_closeout.report_digest().to_string(),
            assembly.simplex_ladder.report_digest().to_string(),
            assembly.simplex_exhaustion.report_digest().to_string(),
            assembly.policy_pressure.report_digest().to_string(),
            format!("{missing_simplex_scenarios:?}"),
            format!("{missing_exhaustion_kinds:?}"),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure {
            registry: registry
                .required_simplex_scenarios()
                .iter()
                .map(|scenario| (*scenario).to_string())
                .collect(),
            compound_closeout: assembly.compound_closeout,
            simplex_ladder: assembly.simplex_ladder,
            simplex_exhaustion: assembly.simplex_exhaustion,
            policy_pressure: assembly.policy_pressure,
            missing_simplex_scenarios,
            missing_exhaustion_kinds,
            mismatches,
            report_digest,
        },
    )
}

fn closeout_digest(
    registry: &PrimitiveConstructionPhaseFiveSixCloseoutRegistry,
    assembly: &PrimitiveConstructionPhaseFiveSixCloseoutAssembly,
) -> String {
    digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            registry.registry_digest.clone(),
            assembly.compound_closeout.report_digest().to_string(),
            assembly.simplex_ladder.report_digest().to_string(),
            assembly.simplex_exhaustion.report_digest().to_string(),
            assembly.policy_pressure.report_digest().to_string(),
        ],
    )
}

#[derive(Debug)]
pub enum PrimitiveConstructionPhaseFiveSixCloseoutReportError {
    Compound(PrimitiveConstructionCompoundAdversarialSiegeError),
    SimplexLadder(PrimitiveConstructionSimplexRealizationLadderReportError),
    PolicyPressure(PrimitiveConstructionPolicyPressureReportBundleError),
    Verification(PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionPhaseFiveSixCloseoutReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compound(error) => write!(f, "{error}"),
            Self::SimplexLadder(error) => write!(f, "{error}"),
            Self::PolicyPressure(error) => write!(f, "{error}"),
            Self::Verification(failure) => write!(
                f,
                "phase five/six closeout failed verification: {:?}",
                failure.mismatches()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPhaseFiveSixCloseoutReportError {}

pub fn prepare_primitive_construction_phase_five_six_closeout_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionPhaseFiveSixCloseoutReport,
    PrimitiveConstructionPhaseFiveSixCloseoutReportError,
> {
    let compound_closeout =
        prepare_primitive_construction_compound_milestone_closeout_report(workspace)
            .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::Compound)?;
    let simplex_ladder =
        prepare_primitive_construction_simplex_realization_strategy_ladder_report(workspace)
            .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::SimplexLadder)?;
    let simplex_exhaustion =
        prepare_primitive_construction_simplex_realization_exhaustion_witness_report();
    let policy_pressure = prepare_primitive_construction_policy_pressure_report_bundle()
        .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::PolicyPressure)?;
    verify_closeout(PrimitiveConstructionPhaseFiveSixCloseoutAssembly::new(
        compound_closeout,
        simplex_ladder,
        simplex_exhaustion,
        policy_pressure,
    ))
    .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::Verification)
}
