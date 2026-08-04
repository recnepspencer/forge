use super::{
    LayoutDurableArtifactKind as ArtifactKind, LayoutDurableArtifactObservation as Artifact,
    LayoutDurableOrdering as Ordering, LayoutFormalInvariant as Invariant,
    LayoutFormalObservationDenial,
};
use crate::courtroom::layout::adjudication::{
    LayoutCourtroomTranscriptIdentity, LayoutEvidenceBundle,
};
use crate::courtroom::layout::owner_coverage::LayoutOwnerFamily;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutFormalOwnerFamilyObservation {
    family: LayoutOwnerFamily,
    cases: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutFormalObservation {
    transcript_identity: LayoutCourtroomTranscriptIdentity,
    owners: Vec<LayoutFormalOwnerFamilyObservation>,
    artifacts: Vec<Artifact>,
    orderings: Vec<Ordering>,
    invariants: Vec<Invariant>,
}

pub fn observe_layout_formal_model(
    bundle: &LayoutEvidenceBundle,
) -> Result<LayoutFormalObservation, LayoutFormalObservationDenial> {
    let source = bundle.durable();
    let owners = bundle
        .coverage()
        .families()
        .map(|family| LayoutFormalOwnerFamilyObservation {
            family,
            cases: bundle.coverage().cases(family).iter().copied().collect(),
        })
        .collect();
    let artifacts = vec![
        Artifact::PhysicalRoot {
            kind: ArtifactKind::BTreeStableRoot,
            root: source.btree_root(),
        },
        Artifact::PhysicalReference {
            kind: ArtifactKind::BTreeSelectedReference,
            reference: source.btree_selected_reference(),
        },
    ];
    Ok(LayoutFormalObservation {
        transcript_identity: bundle.transcript_identity(),
        owners,
        artifacts,
        orderings: vec![Ordering::BTreeReferenceObservedUnderStableRoot],
        invariants: vec![
            Invariant::BTreeSelectedReferenceBoundToStableExecution,
            Invariant::OwnerCaseCoverageIsExact,
        ],
    })
}

impl LayoutFormalObservation {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.transcript_identity
    }

    pub fn owners(&self) -> &[LayoutFormalOwnerFamilyObservation] {
        &self.owners
    }

    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    pub fn orderings(&self) -> &[Ordering] {
        &self.orderings
    }

    pub fn invariants(&self) -> &[Invariant] {
        &self.invariants
    }

    pub fn owner_case_count(&self) -> usize {
        self.owners.iter().map(|owner| owner.cases.len()).sum()
    }
}

impl LayoutFormalOwnerFamilyObservation {
    pub const fn family(&self) -> LayoutOwnerFamily {
        self.family
    }

    pub fn cases(&self) -> &[&'static str] {
        &self.cases
    }
}
