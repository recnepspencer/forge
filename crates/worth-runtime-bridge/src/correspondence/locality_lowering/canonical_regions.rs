use worth_signal::facade::{CanonicalChangedRegions, ChangedRegion};

use super::super::{
    BridgeCorrespondencePrecision, BridgeDeliveredCorrespondenceChange,
    BridgeSemanticDependencyCandidate, BridgeSemanticLocality, InstalledCorrespondenceTarget,
};

pub(crate) fn lower_installed_target_regions(
    dependency: &BridgeSemanticDependencyCandidate,
    target: &InstalledCorrespondenceTarget,
    changes: &[BridgeDeliveredCorrespondenceChange],
) -> CanonicalChangedRegions {
    if target.precision == BridgeCorrespondencePrecision::DeclaredWidening {
        return CanonicalChangedRegions::default();
    }

    match dependency.locality() {
        BridgeSemanticLocality::WholeLogicalGraph => CanonicalChangedRegions::default(),
        BridgeSemanticLocality::SourcePartition(_) => {
            CanonicalChangedRegions::new([ChangedRegion::new(target.partition.clone())])
        }
        BridgeSemanticLocality::SourceRecord | BridgeSemanticLocality::ManagedSourceRecord => {
            CanonicalChangedRegions::new(changes.iter().filter_map(|change| {
                change.relational_record_identity().map(|record| {
                    ChangedRegion::new(target.partition.clone())
                        .with_detail(record.terminal_projection_for_reporting())
                })
            }))
        }
    }
}
