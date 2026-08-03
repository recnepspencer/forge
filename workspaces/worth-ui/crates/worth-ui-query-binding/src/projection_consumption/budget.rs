#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionConsumptionBudgetError {
    ZeroBindings,
    ZeroScalarFields,
    ZeroNativeBytes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionProjectionBudgetError {
    ZeroRows,
    ZeroNativeBytes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionBudget {
    max_rows: u32,
    max_change_operations: usize,
    max_continuation_operations: usize,
    max_native_bytes: usize,
}

impl UiCollectionProjectionBudget {
    pub fn new(
        max_rows: u32,
        max_change_operations: usize,
        max_continuation_operations: usize,
        max_native_bytes: usize,
    ) -> Result<Self, UiCollectionProjectionBudgetError> {
        if max_rows == 0 {
            return Err(UiCollectionProjectionBudgetError::ZeroRows);
        }
        if max_native_bytes == 0 {
            return Err(UiCollectionProjectionBudgetError::ZeroNativeBytes);
        }
        Ok(Self {
            max_rows,
            max_change_operations,
            max_continuation_operations,
            max_native_bytes,
        })
    }

    pub fn max_rows(self) -> u32 {
        self.max_rows
    }

    pub fn max_change_operations(self) -> usize {
        self.max_change_operations
    }

    pub fn max_continuation_operations(self) -> usize {
        self.max_continuation_operations
    }

    pub fn max_native_bytes(self) -> usize {
        self.max_native_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProjectionConsumptionLimits {
    bindings_admitted: usize,
    scalar_fields_accessed: usize,
    collection_rows: usize,
    collection_change_operations: usize,
    continuation_operations: usize,
    native_bytes_retained: usize,
    diagnostic_summary_bytes: usize,
    rich_diagnostic_bytes: usize,
}

impl UiProjectionConsumptionLimits {
    pub fn new(
        bindings_admitted: usize,
        scalar_fields_accessed: usize,
        native_bytes_retained: usize,
    ) -> Self {
        Self {
            bindings_admitted,
            scalar_fields_accessed,
            collection_rows: 0,
            collection_change_operations: 0,
            continuation_operations: 0,
            native_bytes_retained,
            diagnostic_summary_bytes: 0,
            rich_diagnostic_bytes: 0,
        }
    }

    pub fn with_collection(
        mut self,
        rows: usize,
        change_operations: usize,
        continuation_operations: usize,
    ) -> Self {
        self.collection_rows = rows;
        self.collection_change_operations = change_operations;
        self.continuation_operations = continuation_operations;
        self
    }

    pub fn with_diagnostics(mut self, summary_bytes: usize, rich_bytes: usize) -> Self {
        self.diagnostic_summary_bytes = summary_bytes;
        self.rich_diagnostic_bytes = rich_bytes;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProjectionConsumptionBudget {
    bindings_admitted: usize,
    scalar_fields_accessed: usize,
    collection_rows: usize,
    collection_change_operations: usize,
    continuation_operations: usize,
    native_bytes_retained: usize,
    diagnostic_summary_bytes: usize,
    rich_diagnostic_bytes: usize,
}

impl UiProjectionConsumptionBudget {
    pub fn platform_pulse() -> Self {
        Self::bounded(
            UiProjectionConsumptionLimits::new(1, 1, 65_536).with_diagnostics(4_096, 262_144),
        )
        .expect("the frozen Platform Pulse budget is valid")
    }

    pub fn bounded(
        limits: UiProjectionConsumptionLimits,
    ) -> Result<Self, UiProjectionConsumptionBudgetError> {
        if limits.bindings_admitted == 0 {
            return Err(UiProjectionConsumptionBudgetError::ZeroBindings);
        }
        if limits.scalar_fields_accessed == 0 {
            return Err(UiProjectionConsumptionBudgetError::ZeroScalarFields);
        }
        if limits.native_bytes_retained == 0 {
            return Err(UiProjectionConsumptionBudgetError::ZeroNativeBytes);
        }
        Ok(Self {
            bindings_admitted: limits.bindings_admitted,
            scalar_fields_accessed: limits.scalar_fields_accessed,
            collection_rows: limits.collection_rows,
            collection_change_operations: limits.collection_change_operations,
            continuation_operations: limits.continuation_operations,
            native_bytes_retained: limits.native_bytes_retained,
            diagnostic_summary_bytes: limits.diagnostic_summary_bytes,
            rich_diagnostic_bytes: limits.rich_diagnostic_bytes,
        })
    }

    pub fn bindings_admitted(&self) -> usize {
        self.bindings_admitted
    }

    pub fn scalar_fields_accessed(&self) -> usize {
        self.scalar_fields_accessed
    }

    pub fn collection_rows(&self) -> usize {
        self.collection_rows
    }

    pub fn collection_change_operations(&self) -> usize {
        self.collection_change_operations
    }

    pub fn continuation_operations(&self) -> usize {
        self.continuation_operations
    }

    pub fn native_bytes_retained(&self) -> usize {
        self.native_bytes_retained
    }

    pub fn diagnostic_summary_bytes(&self) -> usize {
        self.diagnostic_summary_bytes
    }

    pub fn rich_diagnostic_bytes(&self) -> usize {
        self.rich_diagnostic_bytes
    }
}
