use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, InvalidationTargetIdentityTag};
use crate::mapping::CoarseRoutingMode;
use crate::routing::surfaces::TruthDeltaSurfaceIdentity;

pub type BridgeInvalidationTargetIdentity = BridgeIdentity<InvalidationTargetIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeInvalidationTarget {
    target_identity: BridgeInvalidationTargetIdentity,
    signal_scope: Arc<str>,
    routing_mode: CoarseRoutingMode,
    native_target_basis: Arc<str>,
    surface_identity: TruthDeltaSurfaceIdentity,
}

impl BridgeInvalidationTarget {
    pub(crate) fn new(
        signal_scope: Arc<str>,
        routing_mode: CoarseRoutingMode,
        native_target_basis: impl Into<Arc<str>>,
        surface_identity: TruthDeltaSurfaceIdentity,
    ) -> Self {
        let native_target_basis = native_target_basis.into();
        let canonical_basis = invalidation_target_canonical_basis(
            signal_scope.as_ref(),
            routing_mode,
            surface_identity.as_str(),
        );
        let target_identity = BridgeInvalidationTargetIdentity::admit_bridge_owned(digest_value(
            "invalidation-target",
            &canonical_basis,
        ));
        Self {
            target_identity,
            signal_scope,
            routing_mode,
            native_target_basis,
            surface_identity,
        }
    }

    pub fn target_identity(&self) -> &BridgeInvalidationTargetIdentity {
        &self.target_identity
    }

    pub fn signal_scope(&self) -> &str {
        self.signal_scope.as_ref()
    }

    pub fn routing_mode(&self) -> CoarseRoutingMode {
        self.routing_mode
    }

    pub fn native_target_basis(&self) -> &str {
        self.native_target_basis.as_ref()
    }

    pub fn surface_identity(&self) -> &TruthDeltaSurfaceIdentity {
        &self.surface_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalInvalidationTargets {
    targets: Arc<[BridgeInvalidationTarget]>,
}

impl CanonicalInvalidationTargets {
    pub(crate) fn new(targets: Vec<BridgeInvalidationTarget>) -> Self {
        Self {
            targets: Arc::from(targets),
        }
    }

    pub fn targets(&self) -> &[BridgeInvalidationTarget] {
        &self.targets
    }

    pub(crate) fn shared(&self) -> &Arc<[BridgeInvalidationTarget]> {
        &self.targets
    }

    pub fn len(&self) -> usize {
        self.targets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }
}

fn invalidation_target_canonical_basis(
    signal_scope: &str,
    routing_mode: CoarseRoutingMode,
    surface_identity: &str,
) -> String {
    format!(
        "invalidation-target|signal-scope={}|routing-mode={}|surface-identity={}",
        signal_scope,
        routing_mode_label(routing_mode),
        surface_identity,
    )
}

fn routing_mode_label(mode: CoarseRoutingMode) -> &'static str {
    match mode {
        CoarseRoutingMode::Direct => "direct",
    }
}

fn digest_value(kind: &str, basis: &str) -> Arc<str> {
    let digest = Sha256::digest(basis.as_bytes());
    format!("{kind}:sha256:{digest:x}").into()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::BridgeInvalidationTarget;
    use crate::mapping::CoarseRoutingMode;
    use crate::routing::surfaces::TruthDeltaSurfaceIdentity;

    #[test]
    fn invalidation_target_identity_consumes_surface_proof_not_native_target_basis() {
        let surface_identity =
            TruthDeltaSurfaceIdentity::admit_bridge_owned("truth-delta-surface:sha256:surface-a");
        let field_target = BridgeInvalidationTarget::new(
            Arc::from("signal.profile"),
            CoarseRoutingMode::Direct,
            "committed-patch-target|kind=entity-field|projection-mask=name",
            surface_identity.clone(),
        );
        let same_surface_with_different_native_basis = BridgeInvalidationTarget::new(
            Arc::from("signal.profile"),
            CoarseRoutingMode::Direct,
            "committed-patch-target|kind=entity-region|projection-mask=whole",
            surface_identity,
        );
        let different_surface = BridgeInvalidationTarget::new(
            Arc::from("signal.profile"),
            CoarseRoutingMode::Direct,
            "committed-patch-target|kind=entity-field|projection-mask=name",
            TruthDeltaSurfaceIdentity::admit_bridge_owned("truth-delta-surface:sha256:surface-b"),
        );

        assert_eq!(
            field_target.target_identity(),
            same_surface_with_different_native_basis.target_identity(),
            "native target basis is retained evidence, not target-identity authority"
        );
        assert_ne!(
            field_target.target_identity(),
            different_surface.target_identity(),
            "target identity must still change through the typed truth-delta surface proof"
        );
        assert_eq!(
            field_target.native_target_basis(),
            "committed-patch-target|kind=entity-field|projection-mask=name"
        );
    }
}
