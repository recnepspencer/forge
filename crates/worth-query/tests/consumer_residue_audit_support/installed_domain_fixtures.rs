use super::consumer_residue_audit_fixtures::HostileClassCase;
use worth_query::facade::consumer_kit::WorthQueryConsumerResidueClass;

pub const INSTALLED_DOMAIN_HOSTILE_CASES: &[HostileClassCase] = &[
    case(
        "raw-domain-string-authority",
        WorthQueryConsumerResidueClass::RawDomainStringAuthority,
        "worth_query_domain(",
        "installed-domain-handle",
        "worth_query_domain(",
        "fn residue() { let _ = worth_query_domain(\"raw-domain\"); }",
    ),
    case(
        "consumer-authored-context-digest",
        WorthQueryConsumerResidueClass::ConsumerAuthoredContextDigest,
        "fn context_identity_digest(",
        "installed-domain-context-identity",
        "fn context_identity_digest(",
        "impl Context { fn context_identity_digest(&self) -> String { String::new() } }",
    ),
    case(
        "application-facade-domain-authority",
        WorthQueryConsumerResidueClass::ApplicationFacadeDomainAuthority,
        "WorthQueryApplicationFacade",
        "installed-domain-handle",
        "WorthQueryApplicationFacade",
        "fn residue(value: WorthQueryApplicationFacade) { let _ = value; }",
    ),
    case(
        "independent-operation-registry",
        WorthQueryConsumerResidueClass::IndependentOperationRegistry,
        "WorthQueryGraphReadOperationRegistry",
        "installed-domain-execution-index",
        "WorthQueryGraphReadOperationRegistry",
        "fn residue(value: WorthQueryGraphReadOperationRegistry) { let _ = value; }",
    ),
    case(
        "caller-supplied-operation-registry",
        WorthQueryConsumerResidueClass::CallerSuppliedOperationRegistry,
        "with_operation_registry(",
        "installed-domain-execution-index",
        "with_operation_registry(",
        "fn residue(runtime: Runtime, registry: Registry) { let _ = runtime.with_operation_registry(registry); }",
    ),
    case(
        "query-phase-materializer-import",
        WorthQueryConsumerResidueClass::QueryPhaseMaterializerImport,
        "worth-query-phase-materializer-import",
        "installed-domain-capability",
        "use worth_query",
        "use worth_query::facade::runtime::{materialize_canonical_admission_artifact};",
    ),
    case(
        "consumer-semantic-domain-adapter",
        WorthQueryConsumerResidueClass::ConsumerSemanticDomainAdapter,
        "consumer-semantic-domain-adapter",
        "installed-domain-extension",
        "HadwigerDomainAuthorityAdapter",
        "struct HadwigerDomainAuthorityAdapter;",
    ),
];

const fn case(
    label: &'static str,
    class: WorthQueryConsumerResidueClass,
    detection_key: &'static str,
    replacement_lane: &'static str,
    line_needle: &'static str,
    source: &'static str,
) -> HostileClassCase {
    HostileClassCase {
        label,
        class,
        detection_key,
        replacement_lane,
        line_needle,
        source,
    }
}
