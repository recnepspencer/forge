use worth_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_sequence, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use super::aspect_kinds::{HadwigerAspectKind, HadwigerAspectPosture};
use super::dependency_edges::{HadwigerAspectDependencyEdge, HadwigerAspectInvalidationScope};

const HADWIGER_ASPECT_CLOSURE_DIGEST_VERSION: &str = "WORTH.hadwiger.aspect_closure.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerConservativeInvalidationPosture {
    ExactAspectOnly,
    DependencyClosure,
    ConservativeEscalationRequired,
}

impl HadwigerConservativeInvalidationPosture {
    pub fn requires_conservative_invalidation(self) -> bool {
        self == Self::ConservativeEscalationRequired
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerDependencyClosureBlocker {
    aspect_kind: HadwigerAspectKind,
    observed_posture: HadwigerAspectPosture,
    dependency_edge: Option<HadwigerAspectDependencyEdge>,
    observed_aspect_token: Option<String>,
    reason: String,
}

impl HadwigerDependencyClosureBlocker {
    pub(crate) fn new(
        aspect_kind: HadwigerAspectKind,
        observed_posture: HadwigerAspectPosture,
        dependency_edge: Option<HadwigerAspectDependencyEdge>,
        observed_aspect_token: Option<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            aspect_kind,
            observed_posture,
            dependency_edge,
            observed_aspect_token,
            reason: reason.into(),
        }
    }

    pub fn aspect_kind(&self) -> HadwigerAspectKind {
        self.aspect_kind
    }

    pub fn observed_posture(&self) -> HadwigerAspectPosture {
        self.observed_posture
    }

    pub fn dependency_edge(&self) -> Option<&HadwigerAspectDependencyEdge> {
        self.dependency_edge.as_ref()
    }

    pub fn observed_aspect_token(&self) -> Option<&str> {
        self.observed_aspect_token.as_deref()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.aspect_kind.as_str(),
            self.observed_posture.as_str(),
            self.dependency_edge
                .as_ref()
                .map(HadwigerAspectDependencyEdge::stable_token)
                .unwrap_or_else(|| "no_edge".to_string()),
            self.observed_aspect_token
                .as_deref()
                .unwrap_or("missing_aspect_token"),
            self.reason
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerDependencyClosureReport {
    closure_graph_id: String,
    root_aspect: HadwigerAspectKind,
    required_aspects: Vec<HadwigerAspectKind>,
    present_aspects: Vec<HadwigerAspectKind>,
    present_aspect_tokens: Vec<String>,
    blockers: Vec<HadwigerDependencyClosureBlocker>,
    weakest_posture: HadwigerAspectPosture,
    invalidation_posture: HadwigerConservativeInvalidationPosture,
    closure_digest: String,
}

impl HadwigerDependencyClosureReport {
    pub(crate) fn new(
        closure_graph_id: String,
        root_aspect: HadwigerAspectKind,
        required_aspects: Vec<HadwigerAspectKind>,
        present_aspects: Vec<HadwigerAspectKind>,
        present_aspect_tokens: Vec<String>,
        blockers: Vec<HadwigerDependencyClosureBlocker>,
        edge_scopes: &[HadwigerAspectInvalidationScope],
    ) -> Self {
        let weakest_posture = blockers
            .iter()
            .map(HadwigerDependencyClosureBlocker::observed_posture)
            .max_by_key(|posture| posture.severity_rank())
            .unwrap_or(HadwigerAspectPosture::Admitted);
        let invalidation_posture = invalidation_posture(weakest_posture, edge_scopes);
        let closure_digest = closure_digest(
            &closure_graph_id,
            root_aspect,
            &required_aspects,
            &present_aspects,
            &present_aspect_tokens,
            &blockers,
            invalidation_posture,
        );
        Self {
            closure_graph_id,
            root_aspect,
            required_aspects,
            present_aspects,
            present_aspect_tokens,
            blockers,
            weakest_posture,
            invalidation_posture,
            closure_digest,
        }
    }

    pub fn closure_graph_id(&self) -> &str {
        &self.closure_graph_id
    }

    pub fn root_aspect(&self) -> HadwigerAspectKind {
        self.root_aspect
    }

    pub fn required_aspects(&self) -> &[HadwigerAspectKind] {
        &self.required_aspects
    }

    pub fn present_aspects(&self) -> &[HadwigerAspectKind] {
        &self.present_aspects
    }

    pub fn present_aspect_tokens(&self) -> &[String] {
        &self.present_aspect_tokens
    }

    pub fn blockers(&self) -> &[HadwigerDependencyClosureBlocker] {
        &self.blockers
    }

    pub fn weakest_posture(&self) -> HadwigerAspectPosture {
        self.weakest_posture
    }

    pub fn invalidation_posture(&self) -> HadwigerConservativeInvalidationPosture {
        self.invalidation_posture
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }

    pub fn requires_conservative_invalidation(&self) -> bool {
        self.invalidation_posture
            .requires_conservative_invalidation()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

fn invalidation_posture(
    weakest_posture: HadwigerAspectPosture,
    edge_scopes: &[HadwigerAspectInvalidationScope],
) -> HadwigerConservativeInvalidationPosture {
    if edge_scopes
        .iter()
        .any(|scope| scope.requires_conservative_invalidation())
        || matches!(
            weakest_posture,
            HadwigerAspectPosture::Stale
                | HadwigerAspectPosture::Missing
                | HadwigerAspectPosture::Conflict
        )
    {
        HadwigerConservativeInvalidationPosture::ConservativeEscalationRequired
    } else if edge_scopes.contains(&HadwigerAspectInvalidationScope::DependencyClosure) {
        HadwigerConservativeInvalidationPosture::DependencyClosure
    } else {
        HadwigerConservativeInvalidationPosture::ExactAspectOnly
    }
}

fn closure_digest(
    closure_graph_id: &str,
    root_aspect: HadwigerAspectKind,
    required_aspects: &[HadwigerAspectKind],
    present_aspects: &[HadwigerAspectKind],
    present_aspect_tokens: &[String],
    blockers: &[HadwigerDependencyClosureBlocker],
    invalidation_posture: HadwigerConservativeInvalidationPosture,
) -> String {
    let domain = CanonicalBasisDomain::Future("WORTH.hadwiger.aspect_closure");
    let mut entries = vec![
        text_entry(
            domain,
            "digest_schema_version",
            HADWIGER_ASPECT_CLOSURE_DIGEST_VERSION,
        ),
        text_entry(domain, "closure_graph_id", closure_graph_id),
        text_entry(domain, "root_aspect", root_aspect.as_str()),
        text_entry(
            domain,
            "invalidation_posture",
            format!("{invalidation_posture:?}"),
        ),
    ];
    for (index, aspect) in required_aspects.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("required.{index:04}"),
            aspect.as_str(),
        ));
    }
    for (index, aspect) in present_aspects.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("present.{index:04}"),
            aspect.as_str(),
        ));
    }
    for (index, present_token) in present_aspect_tokens.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("present_token.{index:04}"),
            present_token,
        ));
    }
    for (index, blocker) in blockers.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("blocker.{index:04}"),
            blocker.stable_token(),
        ));
    }

    let version = CanonicalizationRuleVersion::new(HADWIGER_ASPECT_CLOSURE_DIGEST_VERSION)
        .expect("Hadwiger aspect closure digest version is a stable literal");
    let sequence = match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(sequence) => sequence,
        _ => panic!("Hadwiger aspect closure basis is built from stable literal fields"),
    };
    let ready = match CanonicalDigestFrontDoor
        .for_sequence(sequence, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("Hadwiger aspect closure digest algorithm is a stable literal"),
    };
    derive_canonical_digest(ready)
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Field,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}
