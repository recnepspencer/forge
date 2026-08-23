use super::CompileCase;

const AUTHORITY_CASES: &[CompileCase] = &[
    case(
        "product",
        "fail",
        "product-native-preparation-no-builder-extraction",
    ),
    case("product", "pass", "product-native-preparation-valid"),
];
const ORDER_CASES: &[CompileCase] = &[
    case("product", "fail", "product-paint-identities-non-orderable"),
    case(
        "certification",
        "pass",
        "product-paint-identities-lawful-correlation",
    ),
];
const PLATFORM_CASES: &[CompileCase] = &[
    case("product", "fail", "product-cannot-bind-native-host"),
    case("product", "pass", "product-native-preparation-valid"),
];
const PRESENTATION_CASES: &[CompileCase] = &[
    case("host", "fail", "host-presentation-work-authority"),
    case("host", "pass", "host-presentation-mechanics-consumer"),
];
const PROTOCOL_CASES: &[CompileCase] = &[
    case("host", "pass", "host-presentation-mechanics-consumer"),
    case(
        "product",
        "fail",
        "product-raw-protocol-consumer-substitution",
    ),
];
const PHASE5_ASYNC_CASES: &[CompileCase] = &[
    case(
        "certification",
        "fail",
        "certification-phase5-async-authority",
    ),
    case(
        "certification",
        "fail",
        "certification-phase5-signal-effect-authority",
    ),
    case(
        "certification",
        "fail",
        "certification-phase5-serialized-recovery-authority",
    ),
    case(
        "certification",
        "fail",
        "certification-phase5-reporting-authority",
    ),
    case(
        "certification",
        "pass",
        "certification-phase5-async-authority-lawful",
    ),
];

const fn case(owner: &'static str, kind: &'static str, target: &'static str) -> CompileCase {
    CompileCase {
        owner,
        kind,
        target,
    }
}

pub(crate) fn compile_cases_for(requirement: &str) -> &'static [CompileCase] {
    match requirement {
        "P1-AUTHORITY-01" => AUTHORITY_CASES,
        "P1-ORDER-SOURCE-01" => ORDER_CASES,
        "P1-PLATFORM-AUTHORITY-01" => PLATFORM_CASES,
        "P1-PRESENTATION-AUTHORITY-01" => PRESENTATION_CASES,
        "P1-PROTOCOL-01" => PROTOCOL_CASES,
        "P5-TEXT-ASYNC-PRESENTATION-01" => PHASE5_ASYNC_CASES,
        _ => &[],
    }
}
