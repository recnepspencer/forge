use crate::workload_composition::PlanarBooleanEntryBasis;

use super::super::boolean_outcome::PlanarBooleanOutcomeReceipt;
use super::query::query_backed_planar_boolean_declaration;
use super::support::{PlanarBooleanEntryError, PlanarBooleanSupportReceipt};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanDeclaration {
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    operand_pair_identity: PlanarBooleanOperandPairIdentity,
    requested_lane: PlanarBooleanExecutionLane,
    basis: Option<PlanarBooleanEntryBasis>,
    query_intent: String,
}

impl PlanarBooleanDeclaration {
    pub fn new(
        family: PlanarBooleanFamily,
        operation: PlanarBooleanOperation,
        operand_pair_identity: PlanarBooleanOperandPairIdentity,
        requested_lane: PlanarBooleanExecutionLane,
    ) -> Self {
        Self {
            family,
            operation,
            operand_pair_identity,
            requested_lane,
            basis: None,
            query_intent: String::new(),
        }
    }

    pub fn from_basis(mut self, basis: PlanarBooleanEntryBasis) -> Self {
        self.basis = Some(basis);
        self
    }

    pub fn declared_by_query(mut self, query_intent: impl Into<String>) -> Self {
        self.query_intent = query_intent.into();
        self
    }

    pub fn bind(self) -> Result<PlanarBooleanDeclarationReceipt, PlanarBooleanEntryError> {
        PlanarBooleanDeclarationReceipt::new(
            self.family,
            self.operation,
            self.operand_pair_identity,
            self.requested_lane,
            self.basis,
            self.query_intent,
        )
    }

    pub fn inspect_support(&self) -> Result<PlanarBooleanSupportReceipt, PlanarBooleanEntryError> {
        PlanarBooleanSupportReceipt::for_declaration(&self.clone().bind()?)
    }

    pub fn classify_outcome(&self) -> Result<PlanarBooleanOutcomeReceipt, PlanarBooleanEntryError> {
        PlanarBooleanOutcomeReceipt::classify(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanDeclarationReceipt {
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    operand_pair_identity: PlanarBooleanOperandPairIdentity,
    requested_lane: PlanarBooleanExecutionLane,
    basis: PlanarBooleanEntryBasis,
    basis_query_declaration_digest: String,
    basis_query_envelope_digest: String,
    basis_query_handle_digest: String,
    readiness_workload_digest: String,
    query_intent: String,
    query_declaration_digest: String,
    query_envelope_digest: String,
    query_handle_digest: String,
}

impl PlanarBooleanDeclarationReceipt {
    fn new(
        family: PlanarBooleanFamily,
        operation: PlanarBooleanOperation,
        operand_pair_identity: PlanarBooleanOperandPairIdentity,
        requested_lane: PlanarBooleanExecutionLane,
        basis: Option<PlanarBooleanEntryBasis>,
        query_intent: String,
    ) -> Result<Self, PlanarBooleanEntryError> {
        if query_intent.trim().is_empty() {
            return Err(PlanarBooleanEntryError::MissingQueryDeclaration);
        }
        let basis = basis.ok_or(PlanarBooleanEntryError::MissingEntryBasis)?;
        let query_receipt = query_backed_planar_boolean_declaration(
            family,
            operation,
            &operand_pair_identity,
            requested_lane,
            basis.readiness_receipt_identity(),
            basis.readiness_workload_digest(),
            query_intent.trim(),
        )?;
        Ok(Self {
            family,
            operation,
            operand_pair_identity,
            requested_lane,
            basis_query_declaration_digest: basis.query_declaration_digest().to_string(),
            basis_query_envelope_digest: basis.query_envelope_digest().to_string(),
            basis_query_handle_digest: basis.query_handle_digest().to_string(),
            readiness_workload_digest: basis.readiness_workload_digest().to_string(),
            basis,
            query_intent: query_intent.trim().to_string(),
            query_declaration_digest: query_receipt.declaration_digest().to_string(),
            query_envelope_digest: query_receipt.envelope_digest().to_string(),
            query_handle_digest: query_receipt.handle_digest().to_string(),
        })
    }

    pub fn family(&self) -> PlanarBooleanFamily {
        self.family
    }

    pub fn operation(&self) -> PlanarBooleanOperation {
        self.operation
    }

    pub fn operand_pair_identity(&self) -> &PlanarBooleanOperandPairIdentity {
        &self.operand_pair_identity
    }

    pub fn requested_lane(&self) -> PlanarBooleanExecutionLane {
        self.requested_lane
    }

    pub fn basis(&self) -> &PlanarBooleanEntryBasis {
        &self.basis
    }

    pub fn readiness_basis_digest(&self) -> &str {
        self.basis.readiness_receipt_identity()
    }

    pub fn readiness_workload_digest(&self) -> &str {
        &self.readiness_workload_digest
    }

    pub fn basis_query_declaration_digest(&self) -> &str {
        &self.basis_query_declaration_digest
    }

    pub fn basis_query_envelope_digest(&self) -> &str {
        &self.basis_query_envelope_digest
    }

    pub fn basis_query_handle_digest(&self) -> &str {
        &self.basis_query_handle_digest
    }

    pub fn query_intent(&self) -> &str {
        &self.query_intent
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn query_envelope_digest(&self) -> &str {
        &self.query_envelope_digest
    }

    pub fn query_handle_digest(&self) -> &str {
        &self.query_handle_digest
    }

    pub fn classify_outcome(&self) -> Result<PlanarBooleanOutcomeReceipt, PlanarBooleanEntryError> {
        PlanarBooleanOutcomeReceipt::from_declaration_receipt(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanFamily {
    PlanarRegions,
}

impl PlanarBooleanFamily {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::PlanarRegions => "planar region boolean declaration",
        }
    }

    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::PlanarRegions => "planar_regions",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOperation {
    Union,
    Intersect,
    Subtract,
}

impl PlanarBooleanOperation {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::Union => "union",
            Self::Intersect => "intersect",
            Self::Subtract => "subtract",
        }
    }

    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::Union => "union",
            Self::Intersect => "intersect",
            Self::Subtract => "subtract",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanExecutionLane {
    BRepNow,
    EmberFuture,
}

impl PlanarBooleanExecutionLane {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::BRepNow => "brep_now",
            Self::EmberFuture => "ember_future",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOperandPairIdentity {
    identity: String,
}

impl PlanarBooleanOperandPairIdentity {
    pub fn new(identity: impl Into<String>) -> Result<Self, PlanarBooleanEntryError> {
        let identity = identity.into();
        if identity.trim().is_empty() {
            return Err(PlanarBooleanEntryError::InvalidOperandPairIdentity);
        }
        Ok(Self {
            identity: identity.trim().to_string(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.identity
    }
}
