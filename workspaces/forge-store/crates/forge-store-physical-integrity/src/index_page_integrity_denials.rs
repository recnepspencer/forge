use crate::{
    IndeterminatePhysicalDamage, IndexPageIntegrityCounters, ManifestIntegrityDenial,
    PhysicalScopeBasis, UnrecoverableAuthorityDamage,
};
use forge_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexPageIntegrityDenialKind {
    WrongPhysicalFamily,
    MissingAuthorityBasis,
    DamagedAuthority,
    StaleIndexGeneration,
    MissingGenerationLink,
    MismatchedAuthorityRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexPageIntegrityDenial {
    kind: IndexPageIntegrityDenialKind,
    counters: IndexPageIntegrityCounters,
    derived_basis: Option<PhysicalScopeBasis>,
    authority_damage: Option<UnrecoverableAuthorityDamage>,
    indeterminate: Option<IndeterminatePhysicalDamage>,
    expected_owner: Option<PhysicalGenerationOwner>,
    actual_owner: Option<PhysicalGenerationOwner>,
    manifest_denial: Option<ManifestIntegrityDenial>,
}

impl IndexPageIntegrityDenial {
    pub(crate) const fn new(
        kind: IndexPageIntegrityDenialKind,
        counters: IndexPageIntegrityCounters,
    ) -> Self {
        Self {
            kind,
            counters,
            derived_basis: None,
            authority_damage: None,
            indeterminate: None,
            expected_owner: None,
            actual_owner: None,
            manifest_denial: None,
        }
    }

    pub(crate) fn with_derived_basis(mut self, basis: PhysicalScopeBasis) -> Self {
        self.derived_basis = Some(basis);
        self
    }

    pub(crate) const fn with_authority_damage(
        mut self,
        damage: UnrecoverableAuthorityDamage,
    ) -> Self {
        self.authority_damage = Some(damage);
        self
    }

    pub(crate) const fn with_indeterminate(mut self, damage: IndeterminatePhysicalDamage) -> Self {
        self.indeterminate = Some(damage);
        self
    }

    pub(crate) const fn with_expected_actual_owner(
        mut self,
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    ) -> Self {
        self.expected_owner = Some(expected);
        self.actual_owner = Some(actual);
        self
    }

    pub(crate) fn with_manifest_denial(mut self, denial: ManifestIntegrityDenial) -> Self {
        self.manifest_denial = Some(denial);
        self
    }

    pub const fn kind(&self) -> IndexPageIntegrityDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> IndexPageIntegrityCounters {
        self.counters
    }

    pub const fn derived_basis(&self) -> Option<&PhysicalScopeBasis> {
        self.derived_basis.as_ref()
    }

    pub const fn authority_damage(&self) -> Option<&UnrecoverableAuthorityDamage> {
        self.authority_damage.as_ref()
    }

    pub const fn indeterminate_damage(&self) -> Option<&IndeterminatePhysicalDamage> {
        self.indeterminate.as_ref()
    }

    pub const fn expected_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.expected_owner
    }

    pub const fn actual_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.actual_owner
    }

    pub const fn manifest_denial(&self) -> Option<&ManifestIntegrityDenial> {
        self.manifest_denial.as_ref()
    }
}
