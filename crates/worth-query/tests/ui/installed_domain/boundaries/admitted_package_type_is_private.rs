use worth_query::facade::domain::WorthQueryAdmittedDomainPackage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

fn bypass_runtime_installation(_package: WorthQueryAdmittedDomainPackage<ConsumerDomain>) {}

fn main() {}
