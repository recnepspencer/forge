use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionScanPattern {
    OrdinaryOverlapHelper,
    LockFirstAdmission,
    SpeculativeRollbackAdmission,
    RollbackAdmission,
    CallerOwnedCompatibilityList,
    CallerOwnedSerializationHint,
    BroadOverlapScan,
}

impl ConflictBatchAdmissionScanPattern {
    pub(crate) const fn all() -> &'static [Self] {
        &[
            Self::OrdinaryOverlapHelper,
            Self::LockFirstAdmission,
            Self::SpeculativeRollbackAdmission,
            Self::RollbackAdmission,
            Self::CallerOwnedCompatibilityList,
            Self::CallerOwnedSerializationHint,
            Self::BroadOverlapScan,
        ]
    }

    pub const fn pattern(self) -> &'static str {
        match self {
            Self::OrdinaryOverlapHelper => "overlap",
            Self::LockFirstAdmission => "lock-first admission",
            Self::SpeculativeRollbackAdmission => "speculative rollback admission",
            Self::RollbackAdmission => "rollback admission",
            Self::CallerOwnedCompatibilityList => "caller-owned compatibility",
            Self::CallerOwnedSerializationHint => "caller-owned serialization",
            Self::BroadOverlapScan => "broad scan",
        }
    }

    pub(crate) fn matches_surface(self, path: &Path, identifier: &str) -> bool {
        let path = normalized_path(path);
        let identifier = identifier
            .rsplit("::")
            .next()
            .unwrap_or(identifier)
            .to_ascii_lowercase();
        let tokens = identifier_tokens(&identifier);
        match self {
            Self::OrdinaryOverlapHelper => {
                is_ordinary_overlap_authority_identifier(&path, &identifier)
            }
            Self::LockFirstAdmission => {
                has_lock_token(&identifier, &tokens)
                    && (identifier.contains("admit")
                        || identifier.contains("admission")
                        || identifier.contains("batch")
                        || identifier.contains("first"))
            }
            Self::SpeculativeRollbackAdmission => {
                identifier.contains("speculative")
                    && (identifier.contains("rollback")
                        || identifier.contains("admit")
                        || identifier.contains("admission"))
            }
            Self::RollbackAdmission => {
                path.contains("/undo_family_execution/rollback_admission.rs")
                    && (identifier.starts_with("lower_")
                        || identifier.contains("admit")
                        || identifier.contains("admission")
                        || identifier.contains("scope_product"))
            }
            Self::CallerOwnedCompatibilityList => {
                is_compatibility_identifier(&identifier)
                    || is_duplicate_split_compatibility_decision(&path, &identifier)
            }
            Self::CallerOwnedSerializationHint => {
                identifier.contains("serial")
                    && (identifier.contains("caller") || identifier.contains("hint"))
            }
            Self::BroadOverlapScan => {
                identifier.contains("broad")
                    && (identifier.contains("scan")
                        || identifier.contains("overlap")
                        || identifier.contains("receipt")
                        || identifier.contains("ledger"))
            }
        }
    }
}

fn is_compatibility_identifier(identifier: &str) -> bool {
    identifier.contains("compatib")
        && (identifier.contains("caller")
            || identifier.contains("local")
            || identifier.contains("list")
            || identifier.contains("basis")
            || identifier.contains("map")
            || identifier.contains("motion")
            || identifier.contains("posture")
            || identifier.contains("rebuild")
            || identifier.contains("require"))
}

fn is_ordinary_overlap_authority_identifier(path: &str, identifier: &str) -> bool {
    if !identifier.contains("overlap") {
        return false;
    }
    if identifier.ends_with("_identity") {
        return false;
    }
    if path.contains("/certification/public_facade_contracts/contracts/") {
        return is_certification_overlap_coordination_identifier(identifier);
    }
    let is_catalog_constructor = path.contains("/workload_catalog/catalog_constructors.rs")
        && (identifier.contains("coplanar_overlap_pair")
            || identifier.contains("coplanar_overlap_storm"));
    let is_workload_operator = path.contains("/workload_platform/workload_operators/")
        && (identifier.contains("overlap_operator")
            || identifier.contains("overlapworkloadoperator"));
    let is_overlap_chain_module =
        path.contains("/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/");
    let squashed = identifier.replace('_', "");
    identifier.starts_with("build_overlap")
        || is_catalog_constructor
        || identifier.contains("overlapedgechain")
        || identifier.contains("overlap_edge_chain")
        || (is_overlap_chain_module && squashed.contains("overlapchain"))
        || is_workload_operator
        || identifier.contains("projectedoverlap")
        || identifier.contains("certifiedprojectedoverlap")
        || identifier.contains("overlapextractionbundle")
        || identifier.contains("overlapbridgeauthority")
        || identifier.contains("overlaphelper")
        || squashed.contains("overlaphelper")
        || identifier.contains("overlapconflict")
        || squashed.contains("overlapconflict")
        || identifier.contains("overlapcompat")
        || squashed.contains("overlapcompat")
        || identifier.contains("overlapshortcut")
        || squashed.contains("overlapshortcut")
}

fn is_certification_overlap_coordination_identifier(identifier: &str) -> bool {
    let squashed = identifier.replace('_', "");
    squashed.contains("overlapcompatibilityhelper")
        || squashed.contains("overlaphelper")
        || squashed.contains("overlapshortcut")
        || squashed.contains("overlapconflict")
}

fn is_duplicate_split_compatibility_decision(path: &str, identifier: &str) -> bool {
    path.contains("/duplicate_split_normalization/contradiction_basis.rs")
        && (identifier.contains("reject_contradictory")
            || identifier.contains("deny_contradictory"))
}

fn has_lock_token(identifier: &str, tokens: &[&str]) -> bool {
    tokens.contains(&"lock")
        || identifier.contains("lock_first")
        || identifier.contains("lockfirst")
}

fn identifier_tokens(identifier: &str) -> Vec<&str> {
    identifier
        .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
        .flat_map(|part| part.split('_'))
        .filter(|part| !part.is_empty())
        .collect()
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
