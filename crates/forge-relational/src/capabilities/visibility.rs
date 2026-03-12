use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::PublicationBundle;
use crate::replay::data::RelationalReplayRecord;

#[allow(dead_code)]
pub(crate) trait PublicationSink {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>>;
}

impl PublicationSink for RelationalRuntime {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }
}

#[allow(dead_code)]
pub(crate) trait LineageSource {
    fn lineage_event_count(&self) -> usize;
}

impl LineageSource for RelationalRuntime {
    fn lineage_event_count(&self) -> usize {
        self.lineage.events.len()
    }
}

pub(crate) trait VisibilityPolicySource {
    fn visibility_cache_enabled(&self) -> bool;
    fn recent_visibility_window(&self) -> usize;
    fn protect_active_snapshots(&self) -> bool;
    fn protect_branch_heads(&self) -> bool;
}

impl VisibilityPolicySource for RelationalRuntime {
    fn visibility_cache_enabled(&self) -> bool {
        self.config.visibility.cache_policy.enabled
    }

    fn recent_visibility_window(&self) -> usize {
        self.config.visibility.cache_policy.recent_version_window
    }

    fn protect_active_snapshots(&self) -> bool {
        self.config.visibility.cache_policy.protect_active_snapshots
    }

    fn protect_branch_heads(&self) -> bool {
        self.config.visibility.cache_policy.protect_branch_heads
    }
}

pub(crate) trait PublicationPolicySource {
    fn max_patch_records_per_commit(&self) -> usize;
}

impl PublicationPolicySource for RelationalRuntime {
    fn max_patch_records_per_commit(&self) -> usize {
        self.config.publication.policy.max_patch_records_per_commit
    }
}
