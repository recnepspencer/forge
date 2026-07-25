use super::WorkspaceSourceInventory;
use syn::{ImplItem, Item, Visibility};

const ORDINARY_AUTHORITY_LANES: &[AuthorityLane] = &[
    AuthorityLane::crate_visible(
        "crates/worth-ui-runtime/src/facade/lifecycle/freeze.rs",
        "prepare_application_authority",
    ),
    AuthorityLane::public("crates/worth-ui-runtime/src/facade/entry/app.rs", "launch"),
    AuthorityLane::crate_visible(
        "crates/worth-ui-runtime/src/facade/host_session_authority.rs",
        "activate",
    ),
    AuthorityLane::public(
        "crates/worth-ui-runtime/src/facade/entry/application_replacement/cutover.rs",
        "activate_prepared_replacement",
    ),
];

const FORBIDDEN_ORDINARY_SURFACES: &[&str] = &[
    "pub fn launch_runtime(",
    "pub fn from_canonical_artifact(",
    "pub fn into_candidate(",
    "pub fn freeze_infallibly(",
];

pub fn audit_application_authority_topology(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let mut findings = Vec::new();
    for lane in ORDINARY_AUTHORITY_LANES {
        let occurrences = count_authority_lane(inventory, *lane);
        if occurrences != 1 {
            findings.push(format!(
                "{} must own exactly one `{}` lane with {:?} visibility; found {occurrences}",
                lane.path, lane.function, lane.visibility
            ));
        }
    }

    for source in inventory.rust_files_under("crates/worth-ui-runtime/src") {
        if is_support_or_test_source(source.relative_path()) {
            continue;
        }
        for forbidden in FORBIDDEN_ORDINARY_SURFACES {
            if source.text().contains(forbidden) {
                findings.push(format!(
                    "{} exposes removed ordinary authority surface `{forbidden}`",
                    source.relative_path().display()
                ));
            }
        }
    }
    findings.sort();
    findings
}

#[derive(Clone, Copy, Debug)]
struct AuthorityLane {
    path: &'static str,
    function: &'static str,
    visibility: AuthorityVisibility,
}

#[derive(Clone, Copy, Debug)]
enum AuthorityVisibility {
    Public,
    Crate,
}

impl AuthorityLane {
    const fn public(path: &'static str, function: &'static str) -> Self {
        Self {
            path,
            function,
            visibility: AuthorityVisibility::Public,
        }
    }

    const fn crate_visible(path: &'static str, function: &'static str) -> Self {
        Self {
            path,
            function,
            visibility: AuthorityVisibility::Crate,
        }
    }
}

fn count_authority_lane(inventory: &WorkspaceSourceInventory, lane: AuthorityLane) -> usize {
    let source = inventory
        .source(lane.path)
        .unwrap_or_else(|| panic!("authority owner `{}` exists", lane.path));
    let syntax = syn::parse_file(source.text())
        .unwrap_or_else(|error| panic!("{} parses: {error}", lane.path));
    syntax
        .items
        .iter()
        .map(|item| match item {
            Item::Fn(function)
                if function.sig.ident == lane.function
                    && visibility_matches(&function.vis, lane.visibility) =>
            {
                1
            }
            Item::Impl(item) => item
                .items
                .iter()
                .filter(|member| {
                    matches!(
                        member,
                        ImplItem::Fn(method)
                            if method.sig.ident == lane.function
                                && visibility_matches(&method.vis, lane.visibility)
                    )
                })
                .count(),
            _ => 0,
        })
        .sum()
}

fn visibility_matches(visibility: &Visibility, required: AuthorityVisibility) -> bool {
    match (visibility, required) {
        (Visibility::Public(_), AuthorityVisibility::Public) => true,
        (Visibility::Restricted(restricted), AuthorityVisibility::Crate) => restricted
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "crate"),
        _ => false,
    }
}

fn is_support_or_test_source(path: &std::path::Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.contains("/certification_support/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with("_test_support.rs")
}
