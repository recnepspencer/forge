mod continuity;
mod execution;
mod lineage_authority;
mod strategy_replay;
mod surface_comparison;

use crate::performance::ReplayLineageAuthorityIndexedSource;
use crate::replay::data::{DescriptorComparisonBasis, ReplayAuthorityBasisKind};
use crate::runtime::RelationalRuntime;
use crate::schema::ValidatedSchemaContinuityBundle;

pub struct ReplayAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

struct ValidatedReplayContinuityEnvelope<'a> {
    _validated_bundle: ValidatedSchemaContinuityBundle<'a>,
    transition_basis: Option<DescriptorComparisonBasis>,
    continuation_basis: Option<DescriptorComparisonBasis>,
    reconciliation_basis: Option<DescriptorComparisonBasis>,
    lineage_basis: Option<DescriptorComparisonBasis>,
}

struct SelectedPublishedLineageAuthority<'a> {
    kind: ReplayAuthorityBasisKind,
    indexed_source: Option<ReplayLineageAuthorityIndexedSource>,
    artifact: &'a crate::lineage::data::PublishedLineageArtifact,
}

impl<'runtime> ReplayAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

impl RelationalRuntime {
    pub fn replay_authority(&mut self) -> ReplayAuthority<'_> {
        ReplayAuthority::new(self)
    }
}
