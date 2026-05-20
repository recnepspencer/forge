#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceBudgetKind {
    Breadth,
    Density,
    Locality,
    FreshnessSensitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceBudgetDefinition {
    kind: FoundationalPerformanceBudgetKind,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalPerformanceBudgetDefinition {
    pub const fn new(
        kind: FoundationalPerformanceBudgetKind,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            kind,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn kind(&self) -> FoundationalPerformanceBudgetKind {
        self.kind
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceBudgetDecision {
    kind: FoundationalPerformanceBudgetKind,
    requested_units: u32,
    admitted_units: u32,
}

impl FoundationalPerformanceBudgetDecision {
    pub const fn new(
        kind: FoundationalPerformanceBudgetKind,
        requested_units: u32,
        admitted_units: u32,
    ) -> Self {
        Self {
            kind,
            requested_units,
            admitted_units,
        }
    }

    pub const fn kind(&self) -> FoundationalPerformanceBudgetKind {
        self.kind
    }

    pub const fn requested_units(&self) -> u32 {
        self.requested_units
    }

    pub const fn admitted_units(&self) -> u32 {
        self.admitted_units
    }
}

pub fn foundational_performance_budget_definitions() -> [FoundationalPerformanceBudgetDefinition; 4]
{
    [
        FoundationalPerformanceBudgetDefinition::new(
            FoundationalPerformanceBudgetKind::Breadth,
            "breadth",
            "requested versus admitted breadth scope for the named lane",
            "proof that density, freshness, or execution already happened",
        ),
        FoundationalPerformanceBudgetDefinition::new(
            FoundationalPerformanceBudgetKind::Density,
            "density",
            "requested versus admitted density pressure for adaptive work",
            "layout equivalence or executed counter truth",
        ),
        FoundationalPerformanceBudgetDefinition::new(
            FoundationalPerformanceBudgetKind::Locality,
            "locality",
            "requested versus admitted locality expansion across partitions, batches, or basis scopes",
            "fresh execution evidence or one required storage model",
        ),
        FoundationalPerformanceBudgetDefinition::new(
            FoundationalPerformanceBudgetKind::FreshnessSensitive,
            "freshness_sensitive",
            "requested versus admitted work gated by freshness, replay, or retention posture",
            "current-basis execution truth or a completed recovery",
        ),
    ]
}
