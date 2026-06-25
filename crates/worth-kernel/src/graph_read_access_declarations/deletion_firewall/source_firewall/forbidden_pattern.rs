use super::source_roots::SourceFirewallRegion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ForbiddenLocalDeclarationPattern {
    text: &'static str,
    regions: &'static [SourceFirewallRegion],
}

impl ForbiddenLocalDeclarationPattern {
    const fn new(text: &'static str, regions: &'static [SourceFirewallRegion]) -> Self {
        Self { text, regions }
    }

    pub(crate) const fn text(&self) -> &'static str {
        self.text
    }

    pub(crate) fn applies_to(&self, region: SourceFirewallRegion) -> bool {
        self.regions.contains(&region)
    }

    #[cfg(test)]
    pub(crate) fn regions(&self) -> &[SourceFirewallRegion] {
        self.regions
    }
}

const DECLARATION_AND_ADOPTION: &[SourceFirewallRegion] = &[
    SourceFirewallRegion::DeclarationAuthority,
    SourceFirewallRegion::WorthKernelAdoptionAuthority,
];
const TOPOLOGY_SPATIAL: &[SourceFirewallRegion] =
    &[SourceFirewallRegion::TopologySpatialReadHelpers];
const ALL_FIREWALL_REGIONS: &[SourceFirewallRegion] = &[
    SourceFirewallRegion::DeclarationAuthority,
    SourceFirewallRegion::WorthKernelAdoptionAuthority,
    SourceFirewallRegion::TopologySpatialReadHelpers,
];

pub(crate) const FORBIDDEN_LOCAL_DECLARATION_PATTERNS: &[ForbiddenLocalDeclarationPattern] = &[
    ForbiddenLocalDeclarationPattern::new("local_graph_read_declaration", ALL_FIREWALL_REGIONS),
    ForbiddenLocalDeclarationPattern::new("local_access_requirement_row", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new("local_access_support_row", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new(
        "current_worth_kernel_construction_graph_read_access_adoption",
        DECLARATION_AND_ADOPTION,
    ),
    ForbiddenLocalDeclarationPattern::new(
        "WorthKernelGraphReadAccessAdoptionReport",
        DECLARATION_AND_ADOPTION,
    ),
    ForbiddenLocalDeclarationPattern::new(
        "WorthKernelGraphReadAccessAdoptionError",
        DECLARATION_AND_ADOPTION,
    ),
    ForbiddenLocalDeclarationPattern::new("OldGraphReadAdoption", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new("old_graph_read_adoption", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new(
        "graph_read_access_plan_consumption",
        DECLARATION_AND_ADOPTION,
    ),
    ForbiddenLocalDeclarationPattern::new(
        "ephemeral_graph_index_receipt",
        DECLARATION_AND_ADOPTION,
    ),
    ForbiddenLocalDeclarationPattern::new("graph_read_streaming_receipt", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new("live_graph_read_access", DECLARATION_AND_ADOPTION),
    ForbiddenLocalDeclarationPattern::new("local_graph_walk", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("fallback_graph_walk", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("adjacency_loop", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("broad_scan", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("visited_set", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("dedup_set", TOPOLOGY_SPATIAL),
    ForbiddenLocalDeclarationPattern::new("increase_limit_and_retry", TOPOLOGY_SPATIAL),
];

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::graph_read_access_declarations::deletion_firewall) struct ForbiddenPatternAuditRow {
    text: &'static str,
    region: SourceFirewallRegion,
}

#[cfg(test)]
impl ForbiddenPatternAuditRow {
    pub(in crate::graph_read_access_declarations::deletion_firewall) const fn text(
        &self,
    ) -> &'static str {
        self.text
    }

    pub(in crate::graph_read_access_declarations::deletion_firewall) const fn region(
        &self,
    ) -> SourceFirewallRegion {
        self.region
    }
}

#[cfg(test)]
pub(in crate::graph_read_access_declarations::deletion_firewall) fn forbidden_pattern_audit_rows(
) -> Vec<ForbiddenPatternAuditRow> {
    FORBIDDEN_LOCAL_DECLARATION_PATTERNS
        .iter()
        .flat_map(|pattern| {
            pattern
                .regions()
                .iter()
                .copied()
                .map(move |region| ForbiddenPatternAuditRow {
                    text: pattern.text(),
                    region,
                })
        })
        .collect()
}
