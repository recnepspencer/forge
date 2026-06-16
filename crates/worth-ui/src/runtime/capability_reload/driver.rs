use crate::runtime::{
    WorthUiCapabilityPreparedReload, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStatus, WorthUiRuntimeHost,
    WorthUiThemeTokenReloadPackage,
};

use super::{WorthUiCommandDelta, WorthUiCommandProjectionDelta, WorthUiThemeTokenDelta};

impl WorthUiRuntimeHost {
    pub fn prepare_capability_reload(
        &self,
        request: WorthUiCapabilityReloadRequest,
    ) -> WorthUiCapabilityPreparedReload {
        match request {
            WorthUiCapabilityReloadRequest::ThemeTokens(theme_tokens) => {
                self.prepare_theme_token_capability_reload(&theme_tokens)
            }
            WorthUiCapabilityReloadRequest::Commands(commands) => {
                let before = self.inspect_active();
                match WorthUiCommandDelta::derive(
                    self.active_state_for_read().capability_snapshot(),
                    &commands,
                ) {
                    Ok(delta) => self.prepare_snapshot_delta_reload(
                        before.snapshot_digest(),
                        commands.source_digest(),
                        delta.into_parts(),
                    ),
                    Err(denial) => WorthUiCapabilityPreparedReload::new(
                        self.instance_id().raw(),
                        WorthUiCapabilityReloadEvidence::denied(
                            self.instance_id().raw(),
                            before.snapshot_digest(),
                            commands.source_digest(),
                            denial.stage(),
                            denial.detail(),
                        ),
                        None,
                    ),
                }
            }
            WorthUiCapabilityReloadRequest::CommandProjections(projections) => {
                let before = self.inspect_active();
                match WorthUiCommandProjectionDelta::derive(
                    self.active_state_for_read().capability_snapshot(),
                    &projections,
                ) {
                    Ok(delta) => self.prepare_snapshot_delta_reload(
                        before.snapshot_digest(),
                        projections.source_digest(),
                        delta.into_parts(),
                    ),
                    Err(denial) => WorthUiCapabilityPreparedReload::new(
                        self.instance_id().raw(),
                        WorthUiCapabilityReloadEvidence::denied(
                            self.instance_id().raw(),
                            before.snapshot_digest(),
                            projections.source_digest(),
                            denial.stage(),
                            denial.detail(),
                        ),
                        None,
                    ),
                }
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
            changed_facts,
        ) = delta.into_parts();
        self.prepare_snapshot_delta_reload(
            before.snapshot_digest(),
            theme_source_digest,
            (
                candidate_snapshot,
                touched_theme_token_count,
                theme_token_family_entry_count,
                registry_lookup_count,
                changed_facts,
            ),
        )
    }

    fn prepare_snapshot_delta_reload(
        &self,
        active_snapshot_digest_before: u64,
        source_digest: u64,
        delta: (
            crate::capability::CapabilitySnapshot,
            usize,
            usize,
            usize,
            crate::runtime::WorthUiRuntimeFactSet,
        ),
    ) -> WorthUiCapabilityPreparedReload {
        let (
            candidate_snapshot,
            touched_theme_token_count,
            theme_token_family_entry_count,
            registry_lookup_count,
            changed_facts,
        ) = delta;
        let candidate_snapshot_digest = candidate_snapshot.digest().as_u64();
        let status = if candidate_snapshot_digest == active_snapshot_digest_before {
            WorthUiCapabilityReloadStatus::EquivalentNoOp
        } else {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
        };
        let evidence = WorthUiCapabilityReloadEvidence::prepared(
            self.instance_id().raw(),
            status,
            active_snapshot_digest_before,
            candidate_snapshot_digest,
            source_digest,
            touched_theme_token_count,
            theme_token_family_entry_count,
            registry_lookup_count,
            changed_facts,
        );
        let candidate_snapshot = match status {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => Some(candidate_snapshot),
            _ => None,
        };
        WorthUiCapabilityPreparedReload::new(self.instance_id().raw(), evidence, candidate_snapshot)
    }
}
