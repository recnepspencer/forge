use std::collections::BTreeSet;

use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictDerivedDiagnosticProjection,
    WorthTouchedGraphConflictPublicProofInspection,
};

use super::consumer_step::{
    RepresentativeSelectedRouteAuthority, RepresentativeSelectedRouteConsumerKind,
    RepresentativeSelectedRouteConsumerStep, RepresentativeSelectedRouteDiagnosticStep,
    RepresentativeSelectedRouteEvidenceLookupStep, RepresentativeSelectedRoutePublicProofStep,
    RepresentativeSelectedRouteQueryBackedReadStep, RepresentativeSelectedRouteReplayConsumerStep,
    RepresentativeSelectedRouteReuseConsumerStep,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepresentativeSelectedRouteParityPathErrorKind {
    CurrentSelectedRouteUnavailable,
    CurrentPublicFacadeUnavailable,
    CurrentQueryBackedReadUnavailable,
    CurrentEvidenceLookupUnavailable,
    CurrentReplayUnavailable,
    CurrentReuseUnavailable,
    MismatchedSelectedRouteIdentity,
    MismatchedSelectedFamilyIdentity,
    MismatchedSelectedProductIdentity,
    MismatchedSelectedWitnessIdentity,
    MismatchedQueryPosture,
    MismatchedReuseIdentity,
    MismatchedResidueDigest,
    MismatchedSourceFirewallDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteParityPathError {
    kind: RepresentativeSelectedRouteParityPathErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteParityPath {
    authority: RepresentativeSelectedRouteAuthority,
    consumers: Vec<RepresentativeSelectedRouteConsumerStep>,
}

impl RepresentativeSelectedRouteParityPathError {
    pub(crate) fn new(
        kind: RepresentativeSelectedRouteParityPathErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> RepresentativeSelectedRouteParityPathErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl RepresentativeSelectedRouteParityPath {
    pub(crate) fn new(
        authority: RepresentativeSelectedRouteAuthority,
        consumers: Vec<RepresentativeSelectedRouteConsumerStep>,
    ) -> Self {
        Self {
            authority,
            consumers,
        }
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        self.authority.selected_route_identity_digest()
    }

    pub fn authority(&self) -> &RepresentativeSelectedRouteAuthority {
        &self.authority
    }

    pub fn selected_family_identity(&self) -> &str {
        self.authority.selected_family_identity()
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        self.authority.selected_product_identity_digest()
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.authority.selected_witness_identity_digest()
    }

    pub fn residue_digest(&self) -> &str {
        self.public_proof().residue_chain().residue_digest()
    }

    pub fn source_firewall_digest(&self) -> &str {
        self.authority.source_firewall_digest()
    }

    pub fn seed_digest(&self) -> &str {
        self.public_proof().milestone_fifteen_seed().seed_digest()
    }

    pub fn query_posture(&self) -> &RepresentativeSelectedRouteQueryBackedReadStep {
        self.query_backed_read()
    }

    pub fn query_backed_read(&self) -> &RepresentativeSelectedRouteQueryBackedReadStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::QueryBackedRead(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry a query-backed read step")
    }

    pub fn evidence_lookup(&self) -> &RepresentativeSelectedRouteEvidenceLookupStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::EvidenceLookup(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry an evidence-lookup step")
    }

    pub fn replay_or_conflict(&self) -> &RepresentativeSelectedRouteReplayConsumerStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::ReplayOrConflict(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry a replay or conflict step")
    }

    pub fn compiled_product_reuse(&self) -> &RepresentativeSelectedRouteReuseConsumerStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::CompiledProductReuse(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry a compiled-product reuse step")
    }

    pub fn public_proof_step(&self) -> &RepresentativeSelectedRoutePublicProofStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::PublicProof(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry a public-proof step")
    }

    pub fn public_proof(&self) -> &WorthTouchedGraphConflictPublicProofInspection {
        self.public_proof_step().inspection()
    }

    pub fn diagnostic_step(&self) -> &RepresentativeSelectedRouteDiagnosticStep {
        self.consumers
            .iter()
            .find_map(|step| match step {
                RepresentativeSelectedRouteConsumerStep::Diagnostic(step) => Some(step),
                _ => None,
            })
            .expect("representative selected-route path should carry a diagnostic step")
    }

    pub fn derived_diagnostics(&self) -> &WorthTouchedGraphConflictDerivedDiagnosticProjection {
        self.diagnostic_step().projection()
    }

    pub fn consumers(&self) -> &[RepresentativeSelectedRouteConsumerStep] {
        &self.consumers
    }

    pub fn covered_family_kinds(&self) -> BTreeSet<TouchedGraphParityFamilyKind> {
        self.consumers
            .iter()
            .map(|step| match step.kind() {
                RepresentativeSelectedRouteConsumerKind::QueryBackedRead => {
                    TouchedGraphParityFamilyKind::ReadRouting
                }
                RepresentativeSelectedRouteConsumerKind::EvidenceLookup => {
                    TouchedGraphParityFamilyKind::EvidenceLookup
                }
                RepresentativeSelectedRouteConsumerKind::ReplayOrConflict => {
                    TouchedGraphParityFamilyKind::ReplayUndo
                }
                RepresentativeSelectedRouteConsumerKind::CompiledProductReuse => {
                    TouchedGraphParityFamilyKind::CompiledProductReuse
                }
                RepresentativeSelectedRouteConsumerKind::PublicProof => {
                    TouchedGraphParityFamilyKind::PublicProof
                }
                RepresentativeSelectedRouteConsumerKind::Diagnostic => {
                    TouchedGraphParityFamilyKind::DerivedDiagnostics
                }
            })
            .collect()
    }
}
