use super::{
    validation::validate_layout_durable_observation, LayoutDurableArtifactKind as ArtifactKind,
    LayoutDurableArtifactObservation as Artifact, LayoutDurableOrdering as Ordering,
    LayoutFormalInvariant as Invariant, LayoutFormalObservationDenial,
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
    validate_layout_durable_observation(source)?;
    let value = source.lsm_value();
    let generation = source.lsm_generation();
    let tombstone = source.lsm_tombstone();
    let output = source.lsm_output();
    let activation = source.lsm_activation();
    let old_root = source.physical_old_root();
    let new_root = source.physical_new_root();

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
        Artifact::WalRecord {
            kind: ArtifactKind::LsmValue,
            identity: value,
        },
        Artifact::WalRecord {
            kind: ArtifactKind::LsmGenerationPublication,
            identity: generation,
        },
        Artifact::WalRecord {
            kind: ArtifactKind::LsmTombstone,
            identity: tombstone,
        },
        Artifact::WalRecord {
            kind: ArtifactKind::LsmReplacementOutput,
            identity: output,
        },
        Artifact::CheckpointManifest {
            kind: ArtifactKind::LsmActivationManifest,
            scope: activation.clone(),
        },
        Artifact::PhysicalRoot {
            kind: ArtifactKind::PhysicalCompactionOldRoot,
            root: old_root,
        },
        Artifact::PhysicalRoot {
            kind: ArtifactKind::PhysicalCompactionNewRoot,
            root: new_root,
        },
    ];
    Ok(LayoutFormalObservation {
        transcript_identity: bundle.transcript_identity(),
        owners,
        artifacts,
        orderings: vec![
            Ordering::BTreeReferenceObservedUnderStableRoot,
            Ordering::LsmValueBeforeGeneration,
            Ordering::LsmGenerationBeforeTombstone,
            Ordering::LsmTombstoneBeforeReplacementOutput,
            Ordering::LsmActivationCoversInputAndOutput,
            Ordering::PhysicalCompactionRootAdvances,
        ],
        invariants: vec![
            Invariant::BTreeSelectedReferenceBoundToStableExecution,
            Invariant::LsmMembershipRolesAreCanonical,
            Invariant::LsmMembershipSequenceIsStrict,
            Invariant::LsmTombstoneSurvivesReplacementFrontier,
            Invariant::LsmActivationBindsCompactionFrontier,
            Invariant::PhysicalCompactionPublishesNewerRoot,
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
