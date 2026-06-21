use super::touched_graph_inventory::NO_ORDINARY_PUBLIC_FACADE;
use super::touched_graph_types::{
    WorthTouchedGraphAuthorityDisposition as Action,
    WorthTouchedGraphAuthorityInventoryCategory as Category,
    WorthTouchedGraphAuthorityInventoryRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthTouchedGraphStaticAuthorityEntry {
    source_id: &'static str,
    source_path: &'static str,
    category: Category,
    registry_name: &'static str,
    authority_surface: &'static str,
}

impl WorthTouchedGraphStaticAuthorityEntry {
    pub(crate) const fn new(
        source_id: &'static str,
        source_path: &'static str,
        category: Category,
        registry_name: &'static str,
        authority_surface: &'static str,
    ) -> Self {
        Self {
            source_id,
            source_path,
            category,
            registry_name,
            authority_surface,
        }
    }

    pub(crate) const fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub(crate) const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub(crate) const fn category(&self) -> Category {
        self.category
    }

    pub(crate) const fn registry_name(&self) -> &'static str {
        self.registry_name
    }

    pub(crate) const fn authority_surface(&self) -> &'static str {
        self.authority_surface
    }
}

pub(crate) fn current_worth_touched_graph_static_authority_entries(
) -> Vec<WorthTouchedGraphStaticAuthorityEntry> {
    vec![
        entry("ownership", "crates/worth-topo/src/validation/ownership"),
        entry(
            "loop_wiring",
            "crates/worth-topo/src/validation/loop_wiring",
        ),
        entry(
            "radial_rings",
            "crates/worth-topo/src/validation/radial_rings",
        ),
        entry(
            "shell_closure",
            "crates/worth-topo/src/validation/shell_closure",
        ),
        entry(
            "vertex_disks",
            "crates/worth-topo/src/validation/vertex_disks",
        ),
    ]
}

pub(crate) fn current_worth_touched_graph_static_authority_inventory_rows(
) -> Vec<WorthTouchedGraphAuthorityInventoryRow> {
    vec![
        rule_row("ownership"),
        rule_row("loop_wiring"),
        rule_row("radial_rings"),
        rule_row("shell_closure"),
        rule_row("vertex_disks"),
    ]
}

fn entry(
    registry_name: &'static str,
    source_path: &'static str,
) -> WorthTouchedGraphStaticAuthorityEntry {
    WorthTouchedGraphStaticAuthorityEntry::new(
        rule_source_id(registry_name),
        source_path,
        Category::StaticInvariant,
        registry_name,
        "DERIVED_TOPOLOGY_RULE_SPECS",
    )
}

fn rule_row(registry_name: &'static str) -> WorthTouchedGraphAuthorityInventoryRow {
    WorthTouchedGraphAuthorityInventoryRow::new(
        rule_source_id(registry_name),
        "crates/worth-topo/src/validation/rule_registry.rs",
        Category::StaticInvariant,
        "worth-topo",
        "DERIVED_TOPOLOGY_RULE_SPECS contains this individual rule family as static invariant authority.",
        "Phase 5 touched graph predicate coverage must classify this exact rule family before it can execute.",
        Action::Collapse,
        "not-residue",
        "Phase 5 replaces the static rule-family registry entry with touched predicate bucket coverage.",
        NO_ORDINARY_PUBLIC_FACADE,
        "static authority manifest test fails if this rule family lacks a touched inventory row",
    )
}

fn rule_source_id(registry_name: &'static str) -> &'static str {
    match registry_name {
        "ownership" => "topology.validation.rule.ownership",
        "loop_wiring" => "topology.validation.rule.loop_wiring",
        "radial_rings" => "topology.validation.rule.radial_rings",
        "shell_closure" => "topology.validation.rule.shell_closure",
        "vertex_disks" => "topology.validation.rule.vertex_disks",
        _ => "topology.validation.rule.unknown",
    }
}
