use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage,
    WorthQueryDomainSemanticVersion,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

impl WorthQueryDomainEntryMarker for ConsumerDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.consumer.callback"
    }

    fn display_name(&self) -> &'static str {
        "Consumer Callback"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

fn main() {
    let package = WorthQueryDomainPackage::declare(
        ConsumerDomain,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.consumer").unwrap(),
            WorthQueryDomainIdentityName::new("callback").unwrap(),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    );
    let _ = package.execution_callback(|| {});
}
