use worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation;
use worth_store_io_scheduler::SecureIoPreservationReceipt;

use super::{pressure_class, require_lane, PhysicalSchedulerDemand, PhysicalSchedulerDenial};
use crate::physical_runtime::work::{PhysicalWorkOperationFamily, ReadyPhysicalWork};

impl PhysicalSchedulerDemand {
    /// Accepts only queue evidence derived from an exact dirty-frame claim.
    ///
    /// A read grant can never be substituted for writeback authority:
    ///
    /// ```compile_fail
    /// use worth_store::physical_runtime::{
    ///     PhysicalSchedulerDemand, ReadyPhysicalWork,
    /// };
    /// use worth_store_buffer_pool::BufferPoolReadQueueExecutionDeclaration;
    /// use worth_store_io_scheduler::{
    ///     foreground_reservation::PhysicalInstanceForegroundReservation,
    ///     SecureIoPreservationReceipt,
    /// };
    ///
    /// fn substitute_read_declaration(
    ///     ready: ReadyPhysicalWork,
    ///     declaration: BufferPoolReadQueueExecutionDeclaration,
    ///     reservation: PhysicalInstanceForegroundReservation,
    ///     secure_io: SecureIoPreservationReceipt,
    /// ) {
    ///     let _ = PhysicalSchedulerDemand::residency_writeback(
    ///         ready,
    ///         declaration,
    ///         reservation,
    ///         secure_io,
    ///     );
    /// }
    /// ```
    pub fn residency_writeback(
        ready: ReadyPhysicalWork,
        declaration: worth_store_buffer_pool::BufferPoolWritebackQueueExecutionDeclaration,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: SecureIoPreservationReceipt,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        ready
            .require_consumer_active()
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let intent = ready.intent();
        let [coordinate] = intent.scope().coordinates() else {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        };
        if intent.operation() != PhysicalWorkOperationFamily::ArtifactRangeWrite
            || declaration.store() != intent.identity().store()
            || declaration.frame() != *coordinate
            || declaration.grouping_scope().security_scope_identity() != intent.security()
        {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        }
        require_lane(intent.operation(), reservation.receipt().lane())?;
        ready
            .admit_scheduler_pressure(pressure_class(reservation.receipt().lane()))
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let (receipt, capacity) = reservation.into_parts();
        let work = worth_store_io_scheduler::lower_buffer_pool_writeback_queue_declaration(
            declaration,
            receipt,
        )
        .map_err(PhysicalSchedulerDenial::Queue)?
        .with_secure_io_scope(secure_io);
        Ok(Self {
            ready,
            work,
            capacity: Some(capacity),
        })
    }

    /// Lowers a pool-authorized prefetch into scheduler demand only when the
    /// caller also carries the exact secure-I/O preservation proof.
    ///
    /// ```compile_fail
    /// use worth_store::physical_runtime::{PhysicalSchedulerDemand, ReadyPhysicalWork};
    /// use worth_store_buffer_pool::PrefetchResidencyGrant;
    /// use worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation;
    ///
    /// fn without_secure_io(
    ///     ready: ReadyPhysicalWork,
    ///     grant: &PrefetchResidencyGrant,
    ///     reservation: PhysicalInstanceForegroundReservation,
    /// ) {
    ///     let _ = PhysicalSchedulerDemand::residency_prefetch(ready, grant, reservation);
    /// }
    /// ```
    pub fn residency_prefetch(
        ready: ReadyPhysicalWork,
        grant: &worth_store_buffer_pool::PrefetchResidencyGrant,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: SecureIoPreservationReceipt,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        let intent = ready.intent();
        let resources = intent.resources();
        let context = worth_store_buffer_pool::BufferPoolQueueDeclarationContext::new(
            worth_store_buffer_pool::BufferPoolQueueGroupingScope::new(
                reservation.receipt().security_scope_identity(),
            ),
            resources.flush_epoch(),
            speculative_read_shape(resources.queue_shape()),
        );
        let declaration =
            worth_store_buffer_pool::BufferPoolReadQueueExecutionDeclaration::prefetch(
                grant, context,
            );
        Self::residency_read(ready, declaration, reservation, secure_io)
    }

    /// Lowers one frame of an aggregate read-ahead grant only when the caller
    /// also carries the exact secure-I/O preservation proof.
    ///
    /// ```compile_fail
    /// use worth_store::physical_runtime::{PhysicalSchedulerDemand, ReadyPhysicalWork};
    /// use worth_store_buffer_pool::ReadAheadFrameGrant;
    /// use worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation;
    ///
    /// fn without_secure_io(
    ///     ready: ReadyPhysicalWork,
    ///     grant: &ReadAheadFrameGrant<'_, '_>,
    ///     reservation: PhysicalInstanceForegroundReservation,
    /// ) {
    ///     let _ = PhysicalSchedulerDemand::residency_read_ahead(ready, grant, reservation);
    /// }
    /// ```
    pub fn residency_read_ahead(
        ready: ReadyPhysicalWork,
        grant: &worth_store_buffer_pool::ReadAheadFrameGrant<'_, '_>,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: SecureIoPreservationReceipt,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        let intent = ready.intent();
        let resources = intent.resources();
        let context = worth_store_buffer_pool::BufferPoolQueueDeclarationContext::new(
            worth_store_buffer_pool::BufferPoolQueueGroupingScope::new(
                reservation.receipt().security_scope_identity(),
            ),
            resources.flush_epoch(),
            speculative_read_shape(resources.queue_shape()),
        );
        let declaration =
            worth_store_buffer_pool::BufferPoolReadQueueExecutionDeclaration::read_ahead(
                grant, context,
            );
        Self::residency_read(ready, declaration, reservation, secure_io)
    }

    fn residency_read(
        ready: ReadyPhysicalWork,
        declaration: worth_store_buffer_pool::BufferPoolReadQueueExecutionDeclaration,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: SecureIoPreservationReceipt,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        ready
            .require_consumer_active()
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let intent = ready.intent();
        let [coordinate] = intent.scope().coordinates() else {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        };
        if intent.operation() != PhysicalWorkOperationFamily::ArtifactRangeRead
            || declaration.store() != intent.identity().store()
            || declaration.frame() != *coordinate
            || declaration.grouping_scope().security_scope_identity() != intent.security()
        {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        }
        require_lane(intent.operation(), reservation.receipt().lane())?;
        ready
            .admit_scheduler_pressure(pressure_class(reservation.receipt().lane()))
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let (receipt, capacity) = reservation.into_parts();
        let work = worth_store_io_scheduler::lower_buffer_pool_read_queue_declaration(
            declaration,
            receipt,
        )
        .map_err(PhysicalSchedulerDenial::Queue)?
        .with_secure_io_scope(secure_io);
        Ok(Self {
            ready,
            work,
            capacity: Some(capacity),
        })
    }
}

const fn speculative_read_shape(
    shape: worth_store_contracts::QueueProducerResourceShape,
) -> worth_store_contracts::QueueProducerResourceShape {
    shape
        .with_read_ahead_windows(1)
        .with_cache_residency_hints(1)
}
