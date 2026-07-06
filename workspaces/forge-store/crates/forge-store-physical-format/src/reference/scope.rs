use crate::{
    PageGenerationCell, PhysicalGenerationOwner, PhysicalReference,
    PhysicalReferenceValidationWitness, PhysicalRootManifest, RootPublicationValidationWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScopeFamily {
    Page,
    Frame,
    WalFrame,
    Manifest,
    ChunkLike,
    DerivedIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceScope {
    family: PhysicalScopeFamily,
    owner: PhysicalGenerationOwner,
    reference: Option<PhysicalReference>,
}

impl PhysicalReferenceScope {
    pub const fn page(cell: PageGenerationCell) -> Self {
        Self {
            family: PhysicalScopeFamily::Page,
            owner: cell.owner(),
            reference: None,
        }
    }

    pub fn frame(validation: PhysicalReferenceValidationWitness) -> Self {
        Self::from_validated_reference(PhysicalScopeFamily::Frame, validation)
    }

    pub fn wal_frame(validation: PhysicalReferenceValidationWitness) -> Self {
        Self::from_validated_reference(PhysicalScopeFamily::WalFrame, validation)
    }

    pub const fn manifest_page(cell: PageGenerationCell) -> Self {
        Self {
            family: PhysicalScopeFamily::Manifest,
            owner: cell.owner(),
            reference: None,
        }
    }

    pub fn chunk_like(validation: PhysicalReferenceValidationWitness) -> Self {
        Self::from_validated_reference(PhysicalScopeFamily::ChunkLike, validation)
    }

    pub const fn derived_index(cell: PageGenerationCell) -> Self {
        Self {
            family: PhysicalScopeFamily::DerivedIndex,
            owner: cell.owner(),
            reference: None,
        }
    }

    pub const fn family(self) -> PhysicalScopeFamily {
        self.family
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        self.owner
    }

    pub const fn reference(self) -> Option<PhysicalReference> {
        self.reference
    }

    fn from_validated_reference(
        family: PhysicalScopeFamily,
        validation: PhysicalReferenceValidationWitness,
    ) -> Self {
        Self {
            family,
            owner: validation.owner(),
            reference: Some(validation.reference()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootManifestIntegrityPosture {
    CurrentRootAdmitted(CurrentRootManifestAdmission),
    MissingRoot,
    AmbiguousRoot,
    DamagedRoot,
    TornRootPointer,
    MultipleValidRoots,
    RootGenerationMismatch,
    ResidueRootRejected,
    RecoveryBlockingRootDamage,
    WrongRootPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentRootManifestAdmission {
    root_owner: PhysicalGenerationOwner,
}

impl CurrentRootManifestAdmission {
    pub const fn root_owner(self) -> PhysicalGenerationOwner {
        self.root_owner
    }
}

impl RootManifestIntegrityPosture {
    pub const fn current_root_admitted(membership: ManifestMembershipProof) -> Self {
        Self::CurrentRootAdmitted(CurrentRootManifestAdmission {
            root_owner: membership.root_owner(),
        })
    }

    pub fn current_root_publication(validation: RootPublicationValidationWitness) -> Self {
        Self::CurrentRootAdmitted(CurrentRootManifestAdmission {
            root_owner: validation.owner(),
        })
    }

    pub const fn admits_scope(self) -> bool {
        matches!(self, Self::CurrentRootAdmitted(_))
    }

    pub const fn root_owner(self) -> Option<PhysicalGenerationOwner> {
        match self {
            Self::CurrentRootAdmitted(admission) => Some(admission.root_owner()),
            Self::MissingRoot
            | Self::AmbiguousRoot
            | Self::DamagedRoot
            | Self::TornRootPointer
            | Self::MultipleValidRoots
            | Self::RootGenerationMismatch
            | Self::ResidueRootRejected
            | Self::RecoveryBlockingRootDamage
            | Self::WrongRootPosture => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointAdjacencyPosture {
    NotApplicable,
    CheckpointAdjacent,
    NotCheckpointAdjacent,
    MismatchedCheckpointAdjacency,
}

impl CheckpointAdjacencyPosture {
    pub const fn admits_scope(self) -> bool {
        !matches!(self, Self::MismatchedCheckpointAdjacency)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestMembershipProof {
    scope: PhysicalReferenceScope,
    root_owner: PhysicalGenerationOwner,
}

impl ManifestMembershipProof {
    pub fn from_root(
        root: &PhysicalRootManifest,
        scope: PhysicalReferenceScope,
    ) -> Result<Self, ManifestMembershipDenial> {
        if !root_contains_scope(root, scope) {
            return Err(ManifestMembershipDenial::MissingManifestMembership(scope));
        }
        Ok(Self {
            scope,
            root_owner: root.root_publication().owner(),
        })
    }

    pub const fn scope(self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn root_owner(self) -> PhysicalGenerationOwner {
        self.root_owner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestMembershipDenial {
    MissingManifestMembership(PhysicalReferenceScope),
}

fn root_contains_scope(root: &PhysicalRootManifest, scope: PhysicalReferenceScope) -> bool {
    match scope.family() {
        PhysicalScopeFamily::Page | PhysicalScopeFamily::Manifest => {
            root_contains_page(root, scope.owner())
        }
        PhysicalScopeFamily::Frame | PhysicalScopeFamily::WalFrame => {
            scope.reference().is_some_and(|reference| {
                root.page_slots()
                    .iter()
                    .any(|entry| reference.generation_owner() == entry.page_slot().owner())
            })
        }
        PhysicalScopeFamily::ChunkLike => scope.reference().is_some_and(|reference| {
            root.extents()
                .iter()
                .any(|entry| reference.generation_owner() == entry.extent().owner())
        }),
        PhysicalScopeFamily::DerivedIndex => root_contains_page(root, scope.owner()),
    }
}

fn root_contains_page(root: &PhysicalRootManifest, owner: PhysicalGenerationOwner) -> bool {
    root.page_slots().iter().any(|entry| {
        let slot = entry.page_slot();
        owner.segment_id() == Some(slot.segment_id()) && owner.page_id() == Some(slot.page_id())
    })
}
