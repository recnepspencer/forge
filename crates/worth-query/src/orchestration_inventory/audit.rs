use std::collections::{BTreeMap, BTreeSet};

use super::family::{
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceVisibility,
};
use super::row::{WorthQueryOrchestrationSurfaceInventory, WorthQueryOrchestrationSurfaceRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationInventoryAudit {
    inventory_digest: String,
    duplicate_public_names: Vec<String>,
    uninventoried_public_verbs: Vec<String>,
    missing_doc_rows: Vec<String>,
    missing_transcript_rows: Vec<String>,
    missing_certification_rows: Vec<String>,
    missing_support_rows: Vec<String>,
    missing_binding_projection_rows: Vec<String>,
    ordinary_projection_mismatches: Vec<String>,
    family_visibility_gaps: Vec<String>,
    semantic_attachment_gaps: Vec<String>,
}

impl WorthQueryOrchestrationInventoryAudit {
    pub fn current() -> Self {
        Self::from_inventory(&WorthQueryOrchestrationSurfaceInventory::current())
    }

    pub fn from_inventory(inventory: &WorthQueryOrchestrationSurfaceInventory) -> Self {
        let mut seen = BTreeSet::new();
        let mut duplicate_public_names = Vec::new();
        let inventory_names = inventory
            .rows()
            .iter()
            .map(|row| row.public_name().to_string())
            .collect::<BTreeSet<_>>();
        let actual_public_names = actual_orchestration_public_verbs();
        let mut uninventoried_public_verbs = actual_public_names
            .difference(&inventory_names)
            .cloned()
            .collect::<Vec<_>>();
        let mut missing_doc_rows = Vec::new();
        let mut missing_transcript_rows = Vec::new();
        let mut missing_certification_rows = Vec::new();
        let mut missing_support_rows = Vec::new();
        let mut missing_binding_projection_rows = Vec::new();
        let mut ordinary_projection_mismatches = Vec::new();

        for row in inventory.rows() {
            if !seen.insert(row.public_name().to_string()) {
                duplicate_public_names.push(row.public_name().to_string());
            }
            if row.doc_reference().path().is_empty() || row.doc_reference().section().is_empty() {
                missing_doc_rows.push(row.public_name().to_string());
            }
            if row.proof_contract().checked_type_name().is_empty()
                || row.proof_contract().proof_type_name().is_empty()
            {
                missing_transcript_rows.push(row.public_name().to_string());
            }
            if row.certification_reference().suite().is_empty()
                || row.certification_reference().command().is_empty()
            {
                missing_certification_rows.push(row.public_name().to_string());
            }
            if row.proof_contract().support_surface().as_str().is_empty() {
                missing_support_rows.push(row.public_name().to_string());
            }
            if binding_projection_missing(row) {
                missing_binding_projection_rows.push(row.public_name().to_string());
            }
            if row.visibility() == WorthQueryOrchestrationSurfaceVisibility::OrdinaryOutcome
                && !row.ordinary_outcome_supported()
            {
                ordinary_projection_mismatches.push(row.public_name().to_string());
            }
        }

        uninventoried_public_verbs.sort();

        Self {
            inventory_digest: inventory.inventory_digest().to_string(),
            duplicate_public_names,
            uninventoried_public_verbs,
            missing_doc_rows,
            missing_transcript_rows,
            missing_certification_rows,
            missing_support_rows,
            missing_binding_projection_rows,
            ordinary_projection_mismatches,
            family_visibility_gaps: family_visibility_gaps(inventory.rows()),
            semantic_attachment_gaps: semantic_attachment_gaps(inventory.rows()),
        }
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn duplicate_public_names(&self) -> &[String] {
        &self.duplicate_public_names
    }

    pub fn uninventoried_public_verbs(&self) -> &[String] {
        &self.uninventoried_public_verbs
    }

    pub fn missing_doc_rows(&self) -> &[String] {
        &self.missing_doc_rows
    }

    pub fn missing_transcript_rows(&self) -> &[String] {
        &self.missing_transcript_rows
    }

    pub fn missing_certification_rows(&self) -> &[String] {
        &self.missing_certification_rows
    }

    pub fn missing_support_rows(&self) -> &[String] {
        &self.missing_support_rows
    }

    pub fn missing_binding_projection_rows(&self) -> &[String] {
        &self.missing_binding_projection_rows
    }

    pub fn ordinary_projection_mismatches(&self) -> &[String] {
        &self.ordinary_projection_mismatches
    }

    pub fn family_visibility_gaps(&self) -> &[String] {
        &self.family_visibility_gaps
    }

    pub fn semantic_attachment_gaps(&self) -> &[String] {
        &self.semantic_attachment_gaps
    }
}

fn binding_projection_missing(row: &WorthQueryOrchestrationSurfaceRow) -> bool {
    match row.family() {
        WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
        | WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
        | WorthQueryOrchestrationSurfaceFamily::ContinuationExecute => {
            row.binding_projection()
                != WorthQueryOrchestrationBindingProjection::SharedContinuationBinding
        }
        WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration => {
            row.binding_projection()
                != WorthQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding
        }
        WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration => {
            row.binding_projection()
                != WorthQueryOrchestrationBindingProjection::SharedContributionBinding
        }
        WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration => {
            row.binding_projection()
                != WorthQueryOrchestrationBindingProjection::SharedGroupedBinding
        }
        WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary => {
            row.binding_projection() != WorthQueryOrchestrationBindingProjection::None
        }
        WorthQueryOrchestrationSurfaceFamily::DeclarationEntry
        | WorthQueryOrchestrationSurfaceFamily::RouteFromProgressed
        | WorthQueryOrchestrationSurfaceFamily::ReceiptFromProgressed
        | WorthQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed => {
            row.binding_projection() != WorthQueryOrchestrationBindingProjection::None
        }
    }
}

fn semantic_attachment_gaps(rows: &[WorthQueryOrchestrationSurfaceRow]) -> Vec<String> {
    let mut gaps = Vec::new();
    for row in rows {
        if aspect_posture_missing(row) {
            gaps.push(format!("{}:missing aspect posture", row.public_name()));
        }
        if lower_authority_missing(row) {
            gaps.push(format!(
                "{}:missing lower authority attachment",
                row.public_name()
            ));
        }
        if strategy_attachment_missing(row) {
            gaps.push(format!("{}:missing strategy attachment", row.public_name()));
        }
        if contribution_compatibility_missing(row) {
            gaps.push(format!(
                "{}:missing contribution compatibility",
                row.public_name()
            ));
        }
    }
    gaps.extend(helper_semantic_drift(rows));
    gaps
}

fn aspect_posture_missing(row: &WorthQueryOrchestrationSurfaceRow) -> bool {
    matches!(
        row.family(),
        WorthQueryOrchestrationSurfaceFamily::DeclarationEntry
            | WorthQueryOrchestrationSurfaceFamily::RouteFromProgressed
            | WorthQueryOrchestrationSurfaceFamily::ReceiptFromProgressed
            | WorthQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed
            | WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
            | WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
            | WorthQueryOrchestrationSurfaceFamily::ContinuationExecute
            | WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
            | WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
            | WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration
    ) && row.aspect_posture().as_str() == "none"
}

fn lower_authority_missing(row: &WorthQueryOrchestrationSurfaceRow) -> bool {
    let attachment = row.lower_authority_attachment();
    !(attachment.includes_relational()
        || attachment.includes_signal()
        || attachment.includes_runtime_bridge()
        || attachment.includes_foundational_profile())
}

fn strategy_attachment_missing(row: &WorthQueryOrchestrationSurfaceRow) -> bool {
    matches!(
        row.family(),
        WorthQueryOrchestrationSurfaceFamily::DeclarationEntry
            | WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
            | WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
    ) && row.strategy_attachment().as_str() == "none"
}

fn contribution_compatibility_missing(row: &WorthQueryOrchestrationSurfaceRow) -> bool {
    match row.family() {
        WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration => {
            let compatibility = row.contribution_compatibility();
            compatibility.kind().as_str() == "none" || compatibility.supported_families().is_empty()
        }
        WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration => false,
        WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary => false,
        _ => false,
    }
}

fn helper_semantic_drift(rows: &[WorthQueryOrchestrationSurfaceRow]) -> Vec<String> {
    let generics = rows
        .iter()
        .filter(|row| {
            row.doc_reference().path()
                != "crates/worth-query/docs/domain-capabilities/family-helpers.md"
                && row.visibility() == WorthQueryOrchestrationSurfaceVisibility::Ordinary
        })
        .map(|row| (row.family(), row))
        .collect::<BTreeMap<_, _>>();

    rows.iter()
        .filter(|row| {
            row.doc_reference().path()
                == "crates/worth-query/docs/domain-capabilities/family-helpers.md"
                && row.visibility() == WorthQueryOrchestrationSurfaceVisibility::Ordinary
        })
        .filter_map(|row| {
            let generic = generics.get(&row.family())?;
            let drift = row.aspect_posture() != generic.aspect_posture()
                || row.lower_authority_attachment() != generic.lower_authority_attachment()
                || row.strategy_attachment() != generic.strategy_attachment()
                || row.collaborative_extension_posture()
                    != generic.collaborative_extension_posture();
            drift.then(|| format!("{}:helper semantic drift", row.public_name()))
        })
        .collect()
}

fn actual_orchestration_public_verbs() -> BTreeSet<String> {
    let mut verbs = BTreeSet::new();
    for source in admitted_handle_sources() {
        verbs.extend(extract_pub_fn_names(source));
    }
    verbs
}

fn admitted_handle_sources() -> [&'static str; 8] {
    [
        include_str!(
            "../application/domain_handle/admitted_handle/declaration_entry/orchestration.rs"
        ),
        include_str!("../application/domain_handle/admitted_handle/declaration_entry/products.rs"),
        include_str!("../application/domain_handle/admitted_handle/continuation.rs"),
        include_str!(
            "../application/domain_handle/admitted_handle/signal_compatibility_orchestration.rs"
        ),
        include_str!(
            "../application/domain_handle/admitted_handle/contribution_composed_orchestration.rs"
        ),
        include_str!("../application/domain_handle/admitted_handle/recovery.rs"),
        include_str!("../family_helpers/geometry/mod.rs"),
        include_str!("../family_helpers/geometry/continuation.rs"),
    ]
}

fn extract_pub_fn_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let name = trimmed.strip_prefix("pub fn ")?;
            let end = name.find(['<', '(']).unwrap_or(name.len());
            Some(name[..end].to_string())
        })
        .filter(|name| {
            name.starts_with("orchestrate_")
                || name.starts_with("prepare_continuation_")
                || name.starts_with("prepare_preview_for_active_face_selection")
                || name.starts_with("prepare_runtime_route_for_active_face_selection")
                || name.starts_with("prepare_current_truth_view_for_active_face_selection")
                || name.starts_with("prepare_historical_truth_view_for_active_face_selection")
                || name == "execute_prepared_continuation"
                || name == "execute_prepared_continuation_outcome"
                || name == "execute_prepared_continuation_checked"
                || name == "execute_prepared_continuation_proof"
                || name.starts_with("recover_from_")
        })
        .collect()
}

fn family_visibility_gaps(rows: &[WorthQueryOrchestrationSurfaceRow]) -> Vec<String> {
    let mut groups: BTreeMap<
        (&'static str, WorthQueryOrchestrationSurfaceFamily),
        BTreeSet<&'static str>,
    > = BTreeMap::new();
    for row in rows {
        groups
            .entry((row.canonical_base_name(), row.family()))
            .or_default()
            .insert(row.visibility().as_str());
    }

    let mut gaps = Vec::new();
    for ((canonical_base_name, family), visibilities) in groups {
        if family == WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary {
            continue;
        }
        if !visibilities.contains(WorthQueryOrchestrationSurfaceVisibility::Ordinary.as_str()) {
            gaps.push(format!("{family:?}:{canonical_base_name}:missing ordinary"));
        }
        if !visibilities.contains(WorthQueryOrchestrationSurfaceVisibility::Checked.as_str()) {
            gaps.push(format!("{family:?}:{canonical_base_name}:missing checked"));
        }
        if !visibilities.contains(WorthQueryOrchestrationSurfaceVisibility::ProofVisible.as_str()) {
            gaps.push(format!("{family:?}:{canonical_base_name}:missing proof"));
        }
    }
    gaps
}
