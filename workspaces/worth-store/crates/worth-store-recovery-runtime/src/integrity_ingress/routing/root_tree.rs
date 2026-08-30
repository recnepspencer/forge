use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_integrity::{
    IntegrityValidatedBootstrapCatalog, IntegrityValidatedCurrentRootSelector,
    IntegrityValidatedPreviousRootSelector, IntegrityValidatedRootManifest,
    IntegrityValidatedRootRoutingBlock,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::families::{
    bootstrap::IntegrityAdmittedBootstrapCatalog,
    root::{
        IntegrityAdmittedCurrentRootSelector, IntegrityAdmittedPreviousRootSelector,
        IntegrityAdmittedRootManifest, IntegrityAdmittedRootRoutingBlock,
    },
};
use super::super::{ObservedRecoverySource, RecoveryIntegrityIngressCounters};
use super::{recorded, RecoveryIntegrityIngressAttempt};

impl<'media> IntegrityAdmittedRecoveryArtifact<'media> {
    pub(crate) fn bind_bootstrap_catalog(
        observed: &'media ObservedRecoveryArtifact,
        validated: IntegrityValidatedBootstrapCatalog<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedBootstrapCatalog::bind(
                ObservedRecoverySource::complete(observed, scope),
                validated,
            )
            .map(Self::BootstrapCatalog),
            counters,
        )
    }

    pub(crate) fn bind_current_selector(
        observed: &'media ObservedRecoveryArtifact,
        validated: IntegrityValidatedCurrentRootSelector<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedCurrentRootSelector::bind(
                ObservedRecoverySource::complete(observed, scope),
                validated,
            )
            .map(Self::CurrentSelector),
            counters,
        )
    }

    pub(crate) fn bind_previous_selector(
        observed: &'media ObservedRecoveryArtifact,
        validated: IntegrityValidatedPreviousRootSelector<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedPreviousRootSelector::bind(
                ObservedRecoverySource::complete(observed, scope),
                validated,
            )
            .map(Self::PreviousSelector),
            counters,
        )
    }

    pub(crate) fn bind_root_manifest(
        observed: &'media ObservedRecoveryArtifact,
        validated: IntegrityValidatedRootManifest<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedRootManifest::bind(
                ObservedRecoverySource::complete(observed, scope),
                validated,
            )
            .map(Self::RootManifest),
            counters,
        )
    }

    pub(crate) fn bind_root_routing_block(
        observed: &'media ObservedRecoveryArtifact,
        validated: IntegrityValidatedRootRoutingBlock<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedRootRoutingBlock::bind(
                ObservedRecoverySource::complete(observed, scope),
                validated,
            )
            .map(Self::RootRoutingBlock),
            counters,
        )
    }
}
