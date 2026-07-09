use crate::AllocationBudgetDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationScope {
    Foreground,
    Maintenance,
    Recovery,
    Scrub,
    ImportExport,
    Streaming,
}

impl AllocationScope {
    pub const ALL: [Self; 6] = [
        Self::Foreground,
        Self::Maintenance,
        Self::Recovery,
        Self::Scrub,
        Self::ImportExport,
        Self::Streaming,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationByteBudget {
    bytes: u64,
}

impl AllocationByteBudget {
    pub fn bytes(bytes: u64) -> Result<Self, AllocationBudgetDenial> {
        if bytes == 0 {
            Err(AllocationBudgetDenial::AllocationBudgetIsZero)
        } else {
            Ok(Self { bytes })
        }
    }

    pub const fn as_bytes(self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedMetadataReservation {
    bytes: u64,
}

impl FixedMetadataReservation {
    pub fn constant_bytes(bytes: u64) -> Result<Self, AllocationBudgetDenial> {
        if bytes == 0 {
            Err(AllocationBudgetDenial::FixedMetadataReservationIsZero)
        } else {
            Ok(Self { bytes })
        }
    }

    pub const fn as_bytes(self) -> u64 {
        self.bytes
    }

    pub const fn is_constant_size(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationEnvelopeSet {
    foreground: AllocationByteBudget,
    maintenance: AllocationByteBudget,
    recovery: AllocationByteBudget,
    scrub: AllocationByteBudget,
    import_export: AllocationByteBudget,
    streaming: AllocationByteBudget,
    fixed_metadata: FixedMetadataReservation,
}

impl AllocationEnvelopeSet {
    pub const fn budget(self, scope: AllocationScope) -> AllocationByteBudget {
        match scope {
            AllocationScope::Foreground => self.foreground,
            AllocationScope::Maintenance => self.maintenance,
            AllocationScope::Recovery => self.recovery,
            AllocationScope::Scrub => self.scrub,
            AllocationScope::ImportExport => self.import_export,
            AllocationScope::Streaming => self.streaming,
        }
    }

    pub const fn fixed_metadata(self) -> FixedMetadataReservation {
        self.fixed_metadata
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationEnvelopeDeclaration;

impl AllocationEnvelopeDeclaration {
    pub const fn declare() -> AllocationEnvelopeDeclarationBuilder {
        AllocationEnvelopeDeclarationBuilder::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationEnvelopeDeclarationBuilder {
    foreground: Option<AllocationByteBudget>,
    maintenance: Option<AllocationByteBudget>,
    recovery: Option<AllocationByteBudget>,
    scrub: Option<AllocationByteBudget>,
    import_export: Option<AllocationByteBudget>,
    streaming: Option<AllocationByteBudget>,
    fixed_metadata: Option<FixedMetadataReservation>,
}

impl AllocationEnvelopeDeclarationBuilder {
    pub const fn new() -> Self {
        Self {
            foreground: None,
            maintenance: None,
            recovery: None,
            scrub: None,
            import_export: None,
            streaming: None,
            fixed_metadata: None,
        }
    }

    pub const fn foreground(mut self, budget: AllocationByteBudget) -> Self {
        self.foreground = Some(budget);
        self
    }

    pub const fn maintenance(mut self, budget: AllocationByteBudget) -> Self {
        self.maintenance = Some(budget);
        self
    }

    pub const fn recovery(mut self, budget: AllocationByteBudget) -> Self {
        self.recovery = Some(budget);
        self
    }

    pub const fn scrub(mut self, budget: AllocationByteBudget) -> Self {
        self.scrub = Some(budget);
        self
    }

    pub const fn import_export(mut self, budget: AllocationByteBudget) -> Self {
        self.import_export = Some(budget);
        self
    }

    pub const fn streaming(mut self, budget: AllocationByteBudget) -> Self {
        self.streaming = Some(budget);
        self
    }

    pub const fn fixed_metadata(mut self, reservation: FixedMetadataReservation) -> Self {
        self.fixed_metadata = Some(reservation);
        self
    }

    pub fn seal(self) -> Result<AllocationEnvelopeSet, AllocationBudgetDenial> {
        Ok(AllocationEnvelopeSet {
            foreground: required(self.foreground, AllocationScope::Foreground)?,
            maintenance: required(self.maintenance, AllocationScope::Maintenance)?,
            recovery: required(self.recovery, AllocationScope::Recovery)?,
            scrub: required(self.scrub, AllocationScope::Scrub)?,
            import_export: required(self.import_export, AllocationScope::ImportExport)?,
            streaming: required(self.streaming, AllocationScope::Streaming)?,
            fixed_metadata: self
                .fixed_metadata
                .ok_or(AllocationBudgetDenial::MissingFixedMetadataReservation)?,
        })
    }
}

impl Default for AllocationEnvelopeDeclarationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn required(
    budget: Option<AllocationByteBudget>,
    scope: AllocationScope,
) -> Result<AllocationByteBudget, AllocationBudgetDenial> {
    budget.ok_or(AllocationBudgetDenial::MissingScopeBudget(scope))
}
