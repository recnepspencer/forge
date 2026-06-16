use crate::runtime::{
    WorthUiCapabilityPreparedReload, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStatus, WorthUiRuntimeHost,
    WorthUiThemeTokenReloadPackage,
};

use super::WorthUiThemeTokenDelta;

impl WorthUiRuntimeHost {
    pub fn prepare_capability_reload(
        &self,
        request: WorthUiCapabilityReloadRequest,
    ) -> WorthUiCapabilityPreparedReload {
        match request {
            WorthUiCapabilityReloadRequest::ThemeTokens(theme_tokens) => {
                self.prepare_theme_token_capability_reload(&theme_tokens)
            }
        }
    }

    fn prepare_theme_token_capability_reload(
        &self,
        theme_package: &WorthUiThemeTokenReloadPackage,
    ) -> WorthUiCapabilityPreparedReload {
        let before = self.inspect_active();
        let theme_source_digest = theme_package.source_digest();
        let delta = match WorthUiThemeTokenDelta::derive(
            self.active_state_for_read().capability_snapshot(),
            theme_package,
        ) {
            Ok(delta) => delta,
            Err(denial) => {
                let evidence = WorthUiCapabilityReloadEvidence::denied(
                    self.instance_id().raw(),
                    before.snapshot_digest(),
                    theme_source_digest,
                    denial.stage(),
                    denial.detail(),
                );
                return WorthUiCapabilityPreparedReload::new(
                    self.instance_id().raw(),
                    evidence,
                    None,
                );
            }
        };
        let (
            candidate_snapshot,
            touched_theme_token_count,
            theme_token_family_entry_count,
            registry_lookup_count,
        ) = delta.into_parts();
        let candidate_snapshot_digest = candidate_snapshot.digest().as_u64();
        let status = if candidate_snapshot_digest == before.snapshot_digest() {
            WorthUiCapabilityReloadStatus::EquivalentNoOp
        } else {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
        };
        let evidence = WorthUiCapabilityReloadEvidence::prepared(
            self.instance_id().raw(),
            status,
            before.snapshot_digest(),
            candidate_snapshot_digest,
            theme_source_digest,
            touched_theme_token_count,
            theme_token_family_entry_count,
            registry_lookup_count,
        );
        let candidate_snapshot = match status {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => Some(candidate_snapshot),
            _ => None,
        };
        WorthUiCapabilityPreparedReload::new(self.instance_id().raw(), evidence, candidate_snapshot)
    }
}
