use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_integrity::{
    IntegrityValidatedCheckpointBinding, IntegrityValidatedCheckpointBindingCompaction,
    IntegrityValidatedCheckpointDirtyBasis, IntegrityValidatedCheckpointFooter,
    IntegrityValidatedCheckpointStreamHeader, PhysicalByteRange,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::families::checkpoint::{
    IntegrityAdmittedCheckpointBinding, IntegrityAdmittedCheckpointBindingCompaction,
    IntegrityAdmittedCheckpointDirtyBasis, IntegrityAdmittedCheckpointFooter,
    IntegrityAdmittedCheckpointStreamHeader,
};
use super::super::{ObservedRecoverySource, RecoveryIntegrityIngressCounters};
use super::{recorded, RecoveryIntegrityIngressAttempt};

macro_rules! checkpoint_record_binding {
    ($name:ident, $validated:ty, $wrapper:ty, $variant:ident) => {
        pub(crate) fn $name(
            observed: &'media ObservedRecoveryArtifact,
            relative_range: PhysicalByteRange,
            validated: $validated,
            counters: &mut RecoveryIntegrityIngressCounters,
        ) -> RecoveryIntegrityIngressAttempt<'media> {
            let scope = validated.scope();
            recorded(
                scope,
                <$wrapper>::bind(
                    ObservedRecoverySource::bounded_subrange(observed, scope, relative_range),
                    validated,
                )
                .map(Self::$variant),
                counters,
            )
        }
    };
}

impl<'media> IntegrityAdmittedRecoveryArtifact<'media> {
    checkpoint_record_binding!(
        bind_checkpoint_stream_header,
        IntegrityValidatedCheckpointStreamHeader<'media>,
        IntegrityAdmittedCheckpointStreamHeader<'media>,
        CheckpointStreamHeader
    );
    checkpoint_record_binding!(
        bind_checkpoint_dirty_basis,
        IntegrityValidatedCheckpointDirtyBasis<'media>,
        IntegrityAdmittedCheckpointDirtyBasis<'media>,
        CheckpointDirtyBasis
    );
    checkpoint_record_binding!(
        bind_checkpoint_binding_compaction,
        IntegrityValidatedCheckpointBindingCompaction<'media>,
        IntegrityAdmittedCheckpointBindingCompaction<'media>,
        CheckpointBindingCompaction
    );
    checkpoint_record_binding!(
        bind_checkpoint_binding,
        IntegrityValidatedCheckpointBinding<'media>,
        IntegrityAdmittedCheckpointBinding<'media>,
        CheckpointBinding
    );
    checkpoint_record_binding!(
        bind_checkpoint_footer,
        IntegrityValidatedCheckpointFooter<'media>,
        IntegrityAdmittedCheckpointFooter<'media>,
        CheckpointFooter
    );
}
