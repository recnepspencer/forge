use super::CompositeRuntimeWorldBasis;

/// Ordered component axis used by exact basis comparison diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeBasisAxis {
    Relational,
    Signal,
    Correspondence,
    Owner,
    BasisIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeBasisMismatch {
    axes: Vec<CompositeBasisAxis>,
}

impl CompositeBasisMismatch {
    pub fn axes(&self) -> &[CompositeBasisAxis] {
        &self.axes
    }
}

pub(crate) fn compare_exact(
    expected: &CompositeRuntimeWorldBasis,
    observed: &CompositeRuntimeWorldBasis,
) -> Result<(), CompositeBasisMismatch> {
    let mut axes = Vec::new();
    if expected.owner_identity() != observed.owner_identity() {
        axes.push(CompositeBasisAxis::Owner);
    }
    if expected.relational_basis() != observed.relational_basis() {
        axes.push(CompositeBasisAxis::Relational);
    }
    if expected.signal_basis().descriptor() != observed.signal_basis().descriptor() {
        axes.push(CompositeBasisAxis::Signal);
    }
    if expected.correspondence_basis() != observed.correspondence_basis() {
        axes.push(CompositeBasisAxis::Correspondence);
    }
    if expected.identity() != observed.identity() {
        axes.push(CompositeBasisAxis::BasisIdentity);
    }
    if axes.is_empty() {
        Ok(())
    } else {
        Err(CompositeBasisMismatch { axes })
    }
}
