use forge_foundational::facade::{
    CanonicalComparisonOutcome, CanonicalEquivalenceBasis, CanonicalizationRuleVersion,
};

use super::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationCanonicalizationVersion,
    ForgeQueryDeclarationInput,
};
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::QueryComposition];
const OPERATING_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::HistoricalEvaluation];
const OPERATING_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::Relational,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext {
    regime: &'static str,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            regime: "collaborative-authoritative",
        }
    }

    fn restricted() -> Self {
        Self {
            regime: "restricted-authoritative",
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        OPERATING_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        OPERATING_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry-regime:{}", self.regime)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration {
    edge_ref: &'static str,
    parameter: &'static str,
}

impl SplitEdgeDeclaration {
    fn midpoint(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            parameter: "midpoint",
        }
    }

    fn midpoint_builder(edge_ref: &'static str) -> Self {
        Self::midpoint(edge_ref)
    }

    fn at_parameter(edge_ref: &'static str, parameter: &'static str) -> Self {
        Self {
            edge_ref,
            parameter,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn declaration_family(&self) -> &'static str {
        "split-edge"
    }

    fn canonical_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText("split-edge".to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.parameter",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.parameter.to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.edge_ref",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
            ),
        ]
    }
}

fn admitted_handle(
    regime: GeometryOperatingContext,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    GeometryDomain,
    GeometryOperatingContext,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(regime)
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

#[test]
fn equivalent_declaration_authoring_paths_share_the_same_digest() {
    let handle = admitted_handle(GeometryOperatingContext::collaborative());
    let left = handle
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("midpoint declaration should canonicalize");
    let right = handle
        .declare(SplitEdgeDeclaration::midpoint_builder("edge:42"))
        .expect("equivalent midpoint declaration should canonicalize");

    assert_eq!(left.declaration_family(), "split-edge");
    assert_eq!(left.declaration_digest(), right.declaration_digest());
}

#[test]
fn distinct_declaration_meaning_yields_distinct_digests() {
    let handle = admitted_handle(GeometryOperatingContext::collaborative());
    let midpoint = handle
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("midpoint declaration should canonicalize");
    let quarter = handle
        .declare(SplitEdgeDeclaration::at_parameter("edge:42", "quarter"))
        .expect("quarter declaration should canonicalize");

    assert_ne!(midpoint.declaration_digest(), quarter.declaration_digest());
}

#[test]
fn admitted_operating_world_changes_declaration_identity_when_meaning_depends_on_it() {
    let collaborative = admitted_handle(GeometryOperatingContext::collaborative());
    let restricted = admitted_handle(GeometryOperatingContext::restricted());

    let left = collaborative
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("collaborative declaration should canonicalize");
    let right = restricted
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("restricted declaration should canonicalize");

    assert_ne!(
        left.handle_identity_digest(),
        right.handle_identity_digest()
    );
    assert_ne!(left.declaration_digest(), right.declaration_digest());
}

#[test]
fn ordinary_pinned_and_explicit_version_paths_agree_when_version_matches() {
    let handle = admitted_handle(GeometryOperatingContext::collaborative());
    let ordinary = handle
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("ordinary declaration should canonicalize");
    let explicit = handle
        .declare_with_version(
            SplitEdgeDeclaration::midpoint("edge:42"),
            ForgeQueryDeclarationCanonicalizationVersion::explicit(
                CanonicalizationRuleVersion::new("forge.query.declaration.v1")
                    .expect("valid explicit declaration version"),
            ),
        )
        .expect("explicit version declaration should canonicalize");

    assert_eq!(ordinary.declaration_digest(), explicit.declaration_digest());
}

#[test]
fn canonical_comparison_preserves_equivalent_mismatched_and_unsupported_posture() {
    let handle = admitted_handle(GeometryOperatingContext::collaborative());
    let left = handle
        .declare(SplitEdgeDeclaration::midpoint("edge:42"))
        .expect("left declaration should canonicalize");
    let same = handle
        .declare(SplitEdgeDeclaration::midpoint_builder("edge:42"))
        .expect("same declaration should canonicalize");
    let different = handle
        .declare(SplitEdgeDeclaration::at_parameter("edge:42", "quarter"))
        .expect("different declaration should canonicalize");

    let equivalent = left
        .compare_under(&same, CanonicalEquivalenceBasis::ExactCanonicalBasis)
        .expect("exact comparison should prepare");
    assert!(matches!(
        equivalent.outcome(),
        CanonicalComparisonOutcome::Equivalent(_)
    ));

    let mismatched = left
        .compare_under(&different, CanonicalEquivalenceBasis::ExactCanonicalBasis)
        .expect("exact mismatch comparison should prepare");
    assert!(matches!(
        mismatched.outcome(),
        CanonicalComparisonOutcome::Mismatched(_)
    ));

    let unsupported = left
        .compare_under(&same, CanonicalEquivalenceBasis::DigestEquivalent)
        .expect("digest comparison should prepare");
    assert!(matches!(
        unsupported.outcome(),
        CanonicalComparisonOutcome::Unsupported(_)
    ));
}
