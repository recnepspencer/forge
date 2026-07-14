use worth_store_security::StoreCurrentSecurityScopeWitnessSet;

use super::{
    declare_comparator_law, declare_composite_key_ordering, declare_hash_collision_law,
    declare_physical_key_domain, declare_tenant_scoped_key_domain, require_canonical_key_encoding,
    require_prefix_law, require_range_bound_law, CanonicalKeyEncoding, ComparatorLaw,
    CompositeKeyOrderingLaw, HashCollisionLaw, PhysicalKeyDomainWitness, PrefixLawWitness,
    RangeBoundLawWitness, TenantScopedKeyDomain,
};
use crate::{
    catalog::{
        declare_authority_role, declare_derived_accuracy_class, require_scope_partition,
        ArtifactFamilyDenial,
    },
    AdmittedPhysicalArtifactFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalKeyDomainAdmissionCaseId {
    Admitted,
    Denied(crate::catalog::ArtifactFamilyDenialKind),
}

impl PhysicalKeyDomainAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        use crate::catalog::ArtifactFamilyDenialKind as Denial;
        match self {
            Self::Admitted => "layout.key_domain.admission.admitted",
            Self::Denied(Denial::SecurityAuthorityMismatch) => {
                "layout.key_domain.admission.denied.security_authority"
            }
            Self::Denied(Denial::CrossTenantScopePartition) => {
                "layout.key_domain.admission.denied.tenant_scope"
            }
            Self::Denied(Denial::CrossKeyScopePartition) => {
                "layout.key_domain.admission.denied.key_scope"
            }
            Self::Denied(Denial::AuthenticityBoundary) => {
                "layout.key_domain.admission.denied.authenticity"
            }
            Self::Denied(Denial::CustodyBoundary) => "layout.key_domain.admission.denied.custody",
            Self::Denied(Denial::PhysicalKeyDomainNotDeclaredForFamily) => {
                "layout.key_domain.admission.denied.domain_not_declared"
            }
            Self::Denied(_) => "layout.key_domain.admission.denied.unadvertised",
        }
    }
}

pub fn physical_key_domain_admission_cases(
) -> impl Iterator<Item = PhysicalKeyDomainAdmissionCaseId> {
    use crate::catalog::ArtifactFamilyDenialKind as Denial;
    [
        PhysicalKeyDomainAdmissionCaseId::Admitted,
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::SecurityAuthorityMismatch),
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::CrossTenantScopePartition),
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::CrossKeyScopePartition),
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::AuthenticityBoundary),
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::CustodyBoundary),
        PhysicalKeyDomainAdmissionCaseId::Denied(Denial::PhysicalKeyDomainNotDeclaredForFamily),
    ]
    .into_iter()
}

#[derive(Debug, PartialEq, Eq)]
enum PhysicalKeyDomainAdmissionCase {
    Admitted(Box<AdmittedPhysicalKeyDomain>),
    Denied(ArtifactFamilyDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalKeyDomainAdmissionOutcome {
    case: PhysicalKeyDomainAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalKeyDomainAdmissionView<'a> {
    Admitted(&'a AdmittedPhysicalKeyDomain),
    Denied(&'a ArtifactFamilyDenial),
}

impl PhysicalKeyDomainAdmissionOutcome {
    fn admit(
        family: AdmittedPhysicalArtifactFamily,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Self {
        let case = match AdmittedPhysicalKeyDomain::admit(family, security) {
            Ok(domain) => PhysicalKeyDomainAdmissionCase::Admitted(Box::new(domain)),
            Err(denial) => PhysicalKeyDomainAdmissionCase::Denied(denial),
        };
        Self { case }
    }

    pub fn view(&self) -> PhysicalKeyDomainAdmissionView<'_> {
        match &self.case {
            PhysicalKeyDomainAdmissionCase::Admitted(domain) => {
                PhysicalKeyDomainAdmissionView::Admitted(domain.as_ref())
            }
            PhysicalKeyDomainAdmissionCase::Denied(denial) => {
                PhysicalKeyDomainAdmissionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> PhysicalKeyDomainAdmissionCaseId {
        match &self.case {
            PhysicalKeyDomainAdmissionCase::Admitted(_) => {
                PhysicalKeyDomainAdmissionCaseId::Admitted
            }
            PhysicalKeyDomainAdmissionCase::Denied(denial) => {
                PhysicalKeyDomainAdmissionCaseId::Denied(denial.kind())
            }
        }
    }

    pub fn into_result(self) -> Result<AdmittedPhysicalKeyDomain, ArtifactFamilyDenial> {
        match self.case {
            PhysicalKeyDomainAdmissionCase::Admitted(domain) => Ok(*domain),
            PhysicalKeyDomainAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> AdmittedPhysicalKeyDomain {
        self.into_result().unwrap()
    }

    pub fn unwrap_err(self) -> ArtifactFamilyDenial {
        self.into_result().unwrap_err()
    }
}

impl PartialEq<Result<AdmittedPhysicalKeyDomain, ArtifactFamilyDenial>>
    for PhysicalKeyDomainAdmissionOutcome
{
    fn eq(&self, other: &Result<AdmittedPhysicalKeyDomain, ArtifactFamilyDenial>) -> bool {
        match (self.view(), other) {
            (PhysicalKeyDomainAdmissionView::Admitted(left), Ok(right)) => left == right,
            (PhysicalKeyDomainAdmissionView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalKeyDomain {
    family: AdmittedPhysicalArtifactFamily,
    domain: PhysicalKeyDomainWitness,
    encoding: CanonicalKeyEncoding,
    comparator: ComparatorLaw,
    prefix: Option<PrefixLawWitness>,
    range: Option<RangeBoundLawWitness>,
    hash_collision: HashCollisionLaw,
    composite_ordering: CompositeKeyOrderingLaw,
    tenant_partition: TenantScopedKeyDomain,
}

impl AdmittedPhysicalKeyDomain {
    fn admit(
        family: AdmittedPhysicalArtifactFamily,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<Self, ArtifactFamilyDenial> {
        if family.security_identity() != security.key_scope().identity() {
            return Err(ArtifactFamilyDenial::SecurityAuthorityMismatch);
        }
        if family.authority_identity() != security.authority_identity() {
            return Err(ArtifactFamilyDenial::SecurityAuthorityMismatch);
        }
        let role = declare_authority_role(family.classification());
        let accuracy = declare_derived_accuracy_class(role);
        let scope = require_scope_partition(accuracy, security)?;
        let domain = declare_physical_key_domain(scope)?;
        let encoding = require_canonical_key_encoding(domain);
        let comparator = declare_comparator_law(encoding);
        Ok(Self {
            family,
            domain,
            encoding,
            comparator,
            prefix: require_prefix_law(encoding).ok(),
            range: require_range_bound_law(comparator).ok(),
            hash_collision: declare_hash_collision_law(domain),
            composite_ordering: declare_composite_key_ordering(domain),
            tenant_partition: declare_tenant_scoped_key_domain(domain),
        })
    }

    pub const fn family(self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }

    pub const fn witness(self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn domain(self) -> super::PhysicalKeyDomain {
        self.domain.domain()
    }

    pub const fn encoding(self) -> CanonicalKeyEncoding {
        self.encoding
    }

    pub const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }

    pub const fn prefix(self) -> Option<PrefixLawWitness> {
        self.prefix
    }

    pub const fn range(self) -> Option<RangeBoundLawWitness> {
        self.range
    }

    pub const fn hash_collision(self) -> HashCollisionLaw {
        self.hash_collision
    }

    pub const fn composite_ordering(self) -> CompositeKeyOrderingLaw {
        self.composite_ordering
    }

    pub const fn tenant_partition(self) -> TenantScopedKeyDomain {
        self.tenant_partition
    }
}

impl crate::catalog::LayoutDeclarationsFacade {
    pub fn admit_physical_key_domain(
        &self,
        family: AdmittedPhysicalArtifactFamily,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> PhysicalKeyDomainAdmissionOutcome {
        PhysicalKeyDomainAdmissionOutcome::admit(family, security)
    }
}
