use worth_proof::{
    AuthorityMarker, AuthorityProves, CanonicalOrder, PhaseMarker, Proof, ProofMarker,
    ProofSetCons, Uniqueness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalBasisReady;
impl PhaseMarker for CanonicalBasisReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalBundleReady;
impl PhaseMarker for CanonicalBundleReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalComparisonReady;
impl PhaseMarker for CanonicalComparisonReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalExportReady;
impl PhaseMarker for CanonicalExportReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDigestDerivationReady;
impl PhaseMarker for CanonicalDigestDerivationReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalProductionTestReady;
impl PhaseMarker for CanonicalProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDomainCoherence;
impl ProofMarker for CanonicalDomainCoherence {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalRuleVersionBound;
impl ProofMarker for CanonicalRuleVersionBound {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalizationCostObserved;
impl ProofMarker for CanonicalizationCostObserved {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalEquivalenceBasisDeclared;
impl ProofMarker for CanonicalEquivalenceBasisDeclared {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalMismatchLociBound;
impl ProofMarker for CanonicalMismatchLociBound {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalExportManifestBound;
impl ProofMarker for CanonicalExportManifestBound {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDigestInputShapeBound;
impl ProofMarker for CanonicalDigestInputShapeBound {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalProductionReadinessCertified;
impl ProofMarker for CanonicalProductionReadinessCertified {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalBasisConstructionAuthority(());

impl CanonicalBasisConstructionAuthority {
    pub(crate) const fn new() -> Self {
        Self(())
    }
}

impl AuthorityMarker for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalOrder> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<Uniqueness> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalDomainCoherence> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalRuleVersionBound> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalizationCostObserved> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalEquivalenceBasisDeclared> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalMismatchLociBound> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalExportManifestBound> for CanonicalBasisConstructionAuthority {}
impl AuthorityProves<CanonicalDigestInputShapeBound> for CanonicalBasisConstructionAuthority {}

pub type CanonicalBasisReadinessProofs = ProofSetCons<
    Proof<CanonicalOrder, CanonicalBasisConstructionAuthority>,
    ProofSetCons<
        Proof<Uniqueness, CanonicalBasisConstructionAuthority>,
        ProofSetCons<
            Proof<CanonicalDomainCoherence, CanonicalBasisConstructionAuthority>,
            ProofSetCons<
                Proof<CanonicalRuleVersionBound, CanonicalBasisConstructionAuthority>,
                Proof<CanonicalizationCostObserved, CanonicalBasisConstructionAuthority>,
            >,
        >,
    >,
>;

pub type CanonicalBundleReadinessProofs = ProofSetCons<
    Proof<CanonicalRuleVersionBound, CanonicalBasisConstructionAuthority>,
    Proof<CanonicalDomainCoherence, CanonicalBasisConstructionAuthority>,
>;

pub type CanonicalComparisonReadinessProofs = ProofSetCons<
    Proof<CanonicalEquivalenceBasisDeclared, CanonicalBasisConstructionAuthority>,
    Proof<CanonicalMismatchLociBound, CanonicalBasisConstructionAuthority>,
>;

pub type CanonicalExportReadinessProofs = ProofSetCons<
    Proof<CanonicalExportManifestBound, CanonicalBasisConstructionAuthority>,
    Proof<CanonicalizationCostObserved, CanonicalBasisConstructionAuthority>,
>;

pub type CanonicalDigestDerivationReadinessProofs = ProofSetCons<
    Proof<CanonicalDigestInputShapeBound, CanonicalBasisConstructionAuthority>,
    Proof<CanonicalRuleVersionBound, CanonicalBasisConstructionAuthority>,
>;
