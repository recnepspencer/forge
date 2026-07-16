use worth_query::facade::{domain, runtime};
use worth_relational::facade::identity::KindId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InvariantDomain;

impl domain::WorthQueryDomainEntryMarker for InvariantDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.tests.invariant-package"
    }

    fn display_name(&self) -> &'static str {
        "Invariant Package"
    }

    fn required_capability_families(&self) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[]
    }
}

fn package() -> domain::WorthQueryDomainPackage<InvariantDomain> {
    domain::WorthQueryDomainPackage::declare(
        InvariantDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("invariant-package").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .invariant(domain::WorthQueryDomainInvariantDefinition::new(
        domain::WorthQueryDomainIdentityName::new("requires-owner").unwrap(),
        domain::WorthQueryDomainSemanticVersion::new(1, 0),
        domain::WorthQueryDomainInvariantPredicate::requires_outgoing_relations(
            vec![KindId::new(1)],
            vec![KindId::new(2)],
            1,
        ),
    ))
}

fn main() {
    let _ = runtime::WorthQueryRuntimeBuilder::new().domain_package(package());
}
