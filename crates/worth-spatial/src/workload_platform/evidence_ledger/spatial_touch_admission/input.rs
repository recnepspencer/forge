use std::any::type_name;

use super::denial::SpatialGeometryEvidenceTouchDenial;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialGeometryEvidenceTouchRejectedInputKind {
    RawId,
    RawString,
    ReceiptOnly,
    WorkloadEvidenceRow,
    BooleanReceiptLookupProduct,
    QueryDescriptor,
    TopologyProof,
    SchemaVocabulary,
    CopiedReceiptFields,
}

pub(crate) struct SpatialGeometryEvidenceTouchRejectedInput {
    kind: SpatialGeometryEvidenceTouchRejectedInputKind,
    type_name: &'static str,
}

impl SpatialGeometryEvidenceTouchRejectedInputKind {
    pub(crate) fn locality(self) -> &'static str {
        match self {
            Self::RawId => "raw id",
            Self::RawString => "raw string",
            Self::ReceiptOnly => "sealed boolean receipt without complete ledger",
            Self::WorkloadEvidenceRow => "workload evidence row",
            Self::BooleanReceiptLookupProduct => "boolean receipt lookup product",
            Self::QueryDescriptor => "forge-query descriptor",
            Self::TopologyProof => "topology proof",
            Self::SchemaVocabulary => "schema vocabulary",
            Self::CopiedReceiptFields => "copied receipt fields",
        }
    }
}

impl SpatialGeometryEvidenceTouchRejectedInput {
    pub(crate) fn raw_id<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::RawId,
            type_name::<T>(),
        )
    }

    pub(crate) fn raw_string(_: &str) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::RawString,
            type_name::<str>(),
        )
    }

    pub(crate) fn receipt_only<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::ReceiptOnly,
            type_name::<T>(),
        )
    }

    pub(crate) fn workload_evidence_row(_: &WorkloadEvidenceRow) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::WorkloadEvidenceRow,
            type_name::<WorkloadEvidenceRow>(),
        )
    }

    pub(crate) fn boolean_receipt_lookup_product(
        _: &WorkloadEvidenceBooleanReceiptLookupProduct,
    ) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::BooleanReceiptLookupProduct,
            type_name::<WorkloadEvidenceBooleanReceiptLookupProduct>(),
        )
    }

    pub(crate) fn query_descriptor<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::QueryDescriptor,
            type_name::<T>(),
        )
    }

    pub(crate) fn topology_proof<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::TopologyProof,
            type_name::<T>(),
        )
    }

    pub(crate) fn schema_vocabulary<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::SchemaVocabulary,
            type_name::<T>(),
        )
    }

    pub(crate) fn copied_receipt_fields<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::CopiedReceiptFields,
            type_name::<T>(),
        )
    }

    fn new(kind: SpatialGeometryEvidenceTouchRejectedInputKind, type_name: &'static str) -> Self {
        Self { kind, type_name }
    }

    pub(crate) fn kind(&self) -> SpatialGeometryEvidenceTouchRejectedInputKind {
        self.kind
    }

    pub(crate) fn deny(self) -> SpatialGeometryEvidenceTouchDenial {
        SpatialGeometryEvidenceTouchDenial::source_substitution(
            self.kind,
            format!(
                "{} cannot construct spatial touch authority",
                self.type_name
            ),
        )
    }
}
