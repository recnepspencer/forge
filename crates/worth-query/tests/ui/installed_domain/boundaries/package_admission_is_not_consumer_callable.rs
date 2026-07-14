use worth_query::facade::domain::{
    WorthQueryApplicationFacade, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage,
    WorthQueryDomainSemanticVersion,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

impl worth_query::facade::domain::WorthQueryDomainEntryMarker for ConsumerDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.consumer.example"
    }

    fn display_name(&self) -> &'static str {
        "ConsumerDomain"
    }

    fn required_capability_families(
        &self,
    ) -> &'static [worth_query::facade::domain::WorthQueryCapabilityFamily] {
        &[]
    }
}

fn main() {
    let package = WorthQueryDomainPackage::declare(
        ConsumerDomain,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.consumer").unwrap(),
            WorthQueryDomainIdentityName::new("example").unwrap(),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .validate()
    .unwrap();

    let _ = package.admit(&WorthQueryApplicationFacade::runtime_backed_default());
}
