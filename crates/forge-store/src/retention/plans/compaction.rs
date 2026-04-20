#![allow(dead_code)]

use forge_relational::facade::history::CommitId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionPlan {
    retained_basis_label: String,
    closure_witness: crate::RetentionClosureWitness,
    family_labels: Vec<String>,
    superseded_families: Vec<SupersededPhysicalFamily>,
    rewritten_range_count: u64,
}

impl CompactionPlan {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        closure_witness: crate::RetentionClosureWitness,
        family_labels: Vec<String>,
        superseded_families: Vec<SupersededPhysicalFamily>,
        rewritten_range_count: u64,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            closure_witness,
            family_labels,
            superseded_families,
            rewritten_range_count,
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn closure_witness(&self) -> &crate::RetentionClosureWitness {
        &self.closure_witness
    }

    pub fn family_labels(&self) -> &[String] {
        &self.family_labels
    }

    pub fn superseded_families(&self) -> &[SupersededPhysicalFamily] {
        &self.superseded_families
    }

    pub fn rewritten_range_count(&self) -> u64 {
        self.rewritten_range_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishedCompactionProduct {
    product_id: String,
    retained_basis_label: String,
    family_labels: Vec<String>,
}

impl PublishedCompactionProduct {
    pub(crate) fn new(
        product_id: impl Into<String>,
        retained_basis_label: impl Into<String>,
        family_labels: Vec<String>,
    ) -> Self {
        Self {
            product_id: product_id.into(),
            retained_basis_label: retained_basis_label.into(),
            family_labels,
        }
    }

    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_labels(&self) -> &[String] {
        &self.family_labels
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupersededPhysicalFamily {
    family_label: String,
    artifact_id: String,
    basis_commit_id: Option<CommitId>,
}

impl SupersededPhysicalFamily {
    pub(crate) fn new(
        family_label: impl Into<String>,
        artifact_id: impl Into<String>,
        basis_commit_id: Option<CommitId>,
    ) -> Self {
        Self {
            family_label: family_label.into(),
            artifact_id: artifact_id.into(),
            basis_commit_id,
        }
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.basis_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionPublicationReport {
    product: PublishedCompactionProduct,
    superseded_families: Vec<SupersededPhysicalFamily>,
    cost_surface: crate::RetainedReadCostSurface,
}

impl CompactionPublicationReport {
    pub(crate) fn new(
        product: PublishedCompactionProduct,
        superseded_families: Vec<SupersededPhysicalFamily>,
        cost_surface: crate::RetainedReadCostSurface,
    ) -> Self {
        Self {
            product,
            superseded_families,
            cost_surface,
        }
    }

    pub fn product(&self) -> &PublishedCompactionProduct {
        &self.product
    }

    pub fn superseded_families(&self) -> &[SupersededPhysicalFamily] {
        &self.superseded_families
    }

    pub fn cost_surface(&self) -> &crate::RetainedReadCostSurface {
        &self.cost_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionCutoverReport {
    witness: crate::CompactionCutoverWitness,
    superseded_families: Vec<SupersededPhysicalFamily>,
    cost_surface: crate::RetainedReadCostSurface,
}

impl CompactionCutoverReport {
    pub(crate) fn new(
        witness: crate::CompactionCutoverWitness,
        superseded_families: Vec<SupersededPhysicalFamily>,
        cost_surface: crate::RetainedReadCostSurface,
    ) -> Self {
        Self {
            witness,
            superseded_families,
            cost_surface,
        }
    }

    pub fn witness(&self) -> &crate::CompactionCutoverWitness {
        &self.witness
    }

    pub fn superseded_families(&self) -> &[SupersededPhysicalFamily] {
        &self.superseded_families
    }

    pub fn cost_surface(&self) -> &crate::RetainedReadCostSurface {
        &self.cost_surface
    }
}
