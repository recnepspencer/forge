use worth_store_buffer_pool::{
    BufferPoolQueueWriteDurability, BufferPoolReadQueueExecutionDeclaration,
    BufferPoolWritebackQueueExecutionDeclaration,
};
use worth_store_contracts::{QueueProducerKind, QueueProducerResourceShape};
use worth_store_security::{StoreAuthenticityRequirement, StoreKeyScope, StoreTenantScope};
use worth_store_wal::WalQueueExecutionDeclaration;

use crate::foreground_reservation::ForegroundReservationReceipt;

use super::resource_budget::budget_from_shape;
use super::{
    QueueDurabilityClass, QueueExecutionAdmissionDenial, QueueGroupingBasis, QueueRecoveryOrdering,
    QueueWorkClass, QueueWorkDeclaration, QueueWritebackPolicy,
};

struct QueueProducerExecutionDeclaration<Evidence> {
    evidence: Evidence,
    kind: QueueProducerKind,
    durability: QueueDurabilityClass,
    resource_shape: QueueProducerResourceShape,
    flush_epoch: u64,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
}

pub fn lower_wal_queue_declaration(
    declaration: WalQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    QueueProducerExecutionDeclaration::from_wal(declaration).lower_foreground(reservation)
}

pub fn lower_buffer_pool_read_queue_declaration(
    declaration: BufferPoolReadQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    require_matching_security_scope(declaration.grouping_scope(), &reservation)?;
    QueueProducerExecutionDeclaration::from_buffer_pool_read(declaration)
        .lower_foreground(reservation)
}

pub fn lower_buffer_pool_writeback_queue_declaration(
    declaration: BufferPoolWritebackQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    require_matching_security_scope(declaration.grouping_scope(), &reservation)?;
    QueueProducerExecutionDeclaration::from_buffer_pool_writeback(declaration)
        .lower_foreground(reservation)
}

fn require_matching_security_scope(
    grouping_scope: worth_store_buffer_pool::BufferPoolQueueGroupingScope,
    reservation: &ForegroundReservationReceipt,
) -> Result<(), QueueExecutionAdmissionDenial> {
    if grouping_scope.security_scope_identity() != reservation.security_scope_identity() {
        return Err(QueueExecutionAdmissionDenial::ProducerSecurityScopeMismatch);
    }
    Ok(())
}

impl QueueProducerExecutionDeclaration<WalQueueExecutionDeclaration> {
    const fn from_wal(declaration: WalQueueExecutionDeclaration) -> Self {
        let grouping = declaration.grouping_scope();
        Self {
            evidence: declaration,
            kind: declaration.producer_kind(),
            durability: QueueDurabilityClass::WalCommit,
            resource_shape: declaration.resource_shape(),
            flush_epoch: declaration.flush_epoch(),
            tenant_scope: grouping.tenant_scope(),
            key_scope: grouping.key_scope(),
            authenticity_requirement: grouping.authenticity_requirement(),
        }
    }

    fn lower_foreground(
        self,
        reservation: ForegroundReservationReceipt,
    ) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
        let (_, durability, budget, grouping) = self.into_foreground_parts(reservation)?;
        Ok(
            QueueWorkDeclaration::foreground(reservation.execution_ready(), durability, budget)
                .with_grouping_basis(grouping),
        )
    }
}

impl QueueProducerExecutionDeclaration<BufferPoolReadQueueExecutionDeclaration> {
    const fn from_buffer_pool_read(declaration: BufferPoolReadQueueExecutionDeclaration) -> Self {
        let grouping = declaration.grouping_scope();
        Self {
            evidence: declaration,
            kind: declaration.producer_kind(),
            durability: QueueDurabilityClass::ReadOnly,
            resource_shape: declaration.resource_shape(),
            flush_epoch: declaration.flush_epoch(),
            tenant_scope: grouping.tenant_scope(),
            key_scope: grouping.key_scope(),
            authenticity_requirement: grouping.authenticity_requirement(),
        }
    }

    fn lower_foreground(
        self,
        reservation: ForegroundReservationReceipt,
    ) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
        let (declaration, _, budget, grouping) = self.into_foreground_parts(reservation)?;
        Ok(QueueWorkDeclaration::foreground_buffer_pool_read(
            reservation.execution_ready(),
            budget,
            declaration,
        )
        .with_grouping_basis(grouping))
    }
}

impl QueueProducerExecutionDeclaration<BufferPoolWritebackQueueExecutionDeclaration> {
    const fn from_buffer_pool_writeback(
        declaration: BufferPoolWritebackQueueExecutionDeclaration,
    ) -> Self {
        let grouping = declaration.grouping_scope();
        Self {
            evidence: declaration,
            kind: declaration.producer_kind(),
            durability: buffer_pool_write_durability(declaration.durability()),
            resource_shape: declaration.resource_shape(),
            flush_epoch: declaration.flush_epoch(),
            tenant_scope: grouping.tenant_scope(),
            key_scope: grouping.key_scope(),
            authenticity_requirement: grouping.authenticity_requirement(),
        }
    }

    fn lower_foreground(
        self,
        reservation: ForegroundReservationReceipt,
    ) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
        let (declaration, durability, budget, grouping) =
            self.into_foreground_parts(reservation)?;
        Ok(QueueWorkDeclaration::foreground_buffer_pool_writeback(
            reservation.execution_ready(),
            durability,
            budget,
            declaration,
        )
        .with_grouping_basis(grouping))
    }
}

impl<Evidence> QueueProducerExecutionDeclaration<Evidence> {
    fn into_foreground_parts(
        self,
        reservation: ForegroundReservationReceipt,
    ) -> Result<
        (
            Evidence,
            QueueDurabilityClass,
            crate::BackgroundResourceBudget,
            QueueGroupingBasis,
        ),
        QueueExecutionAdmissionDenial,
    > {
        let budget = budget_from_shape(self.resource_shape)?;
        let grouping = QueueGroupingBasis::new(
            reservation.security_scope_identity(),
            self.tenant_scope,
            self.key_scope,
            self.authenticity_requirement,
            self.durability,
            self.flush_epoch,
            QueueWorkClass::Foreground(reservation.lane()),
            recovery_ordering_for_producer(self.kind),
            writeback_policy_for_producer(self.kind),
        );
        Ok((self.evidence, self.durability, budget, grouping))
    }
}

const fn buffer_pool_write_durability(
    durability: BufferPoolQueueWriteDurability,
) -> QueueDurabilityClass {
    match durability {
        BufferPoolQueueWriteDurability::BufferedWrite => QueueDurabilityClass::BufferedWrite,
        BufferPoolQueueWriteDurability::FileDataSynchronization => {
            QueueDurabilityClass::PlatformDurable
        }
    }
}

const fn recovery_ordering_for_producer(kind: QueueProducerKind) -> QueueRecoveryOrdering {
    match kind {
        QueueProducerKind::WalCommitRecord | QueueProducerKind::WalCheckpointRecord => {
            QueueRecoveryOrdering::WalBeforeData
        }
        QueueProducerKind::BufferPoolReadAhead | QueueProducerKind::BufferPoolWriteBack => {
            QueueRecoveryOrdering::NotRecoveryCritical
        }
    }
}

const fn writeback_policy_for_producer(kind: QueueProducerKind) -> QueueWritebackPolicy {
    match kind {
        QueueProducerKind::WalCommitRecord | QueueProducerKind::WalCheckpointRecord => {
            QueueWritebackPolicy::Immediate
        }
        QueueProducerKind::BufferPoolReadAhead => QueueWritebackPolicy::None,
        QueueProducerKind::BufferPoolWriteBack => QueueWritebackPolicy::DeferredWithinFlushEpoch,
    }
}
