use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration {
    edge_ref: &'static str,
    parameter: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn declaration_family(&self) -> &'static str {
        "split-edge"
    }

    fn canonical_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.edge_ref",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.parameter",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.parameter.to_string()),
            ),
        ]
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = query
        .domain(GeometryDomain)
        .with_operating_context(GeometryOperatingContext)
        .validate()
        .unwrap()
        .admit()
        .unwrap();

    let declaration = handle
        .declare(SplitEdgeDeclaration {
            edge_ref: "edge:42",
            parameter: "midpoint",
        })
        .unwrap();

    let _ = declaration.declaration_family();
    let _ = declaration.declaration_digest();
    let _ = declaration.canonical_basis_bundle();
}
