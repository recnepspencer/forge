#[cfg(test)]
use std::any::type_name;

#[cfg(test)]
use super::denial::SpatialGeometryEvidenceTouchDenial;
#[cfg(test)]
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialGeometryEvidenceTouchRejectedInputKind {
    #[cfg(test)]
    RawId,
    #[cfg(test)]
    RawString,
    #[cfg(test)]
    ReceiptOnly,
    WorkloadEvidenceRow,
    #[cfg(test)]
    BooleanReceiptLookupProduct,
    #[cfg(test)]
    QueryDescriptor,
    #[cfg(test)]
    TopologyProof,
    #[cfg(test)]
    SchemaVocabulary,
    #[cfg(test)]
    CopiedReceiptFields,
}

#[cfg(test)]
pub(crate) struct SpatialGeometryEvidenceTouchRejectedInput {
    kind: SpatialGeometryEvidenceTouchRejectedInputKind,
    type_name: &'static str,
}

impl SpatialGeometryEvidenceTouchRejectedInputKind {
    pub(crate) fn locality(self) -> &'static str {
        match self {
            #[cfg(test)]
            Self::RawId => "raw id",
            #[cfg(test)]
            Self::RawString => "raw string",
            #[cfg(test)]
            Self::ReceiptOnly => "sealed boolean receipt without complete ledger",
            Self::WorkloadEvidenceRow => "workload evidence row",
            #[cfg(test)]
            Self::BooleanReceiptLookupProduct => "boolean receipt lookup product",
            #[cfg(test)]
            Self::QueryDescriptor => "forge-query descriptor",
            #[cfg(test)]
            Self::TopologyProof => "topology proof",
            #[cfg(test)]
            Self::SchemaVocabulary => "schema vocabulary",
            #[cfg(test)]
            Self::CopiedReceiptFields => "copied receipt fields",
        }
    }
}

#[cfg(test)]
impl SpatialGeometryEvidenceTouchRejectedInput {
    #[cfg(test)]
    pub(crate) fn raw_id<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::RawId,
            type_name::<T>(),
        )
    }

    #[cfg(test)]
    pub(crate) fn raw_string(_: &str) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::RawString,
            type_name::<str>(),
        )
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub(crate) fn boolean_receipt_lookup_product(
        _: &WorkloadEvidenceBooleanReceiptLookupProduct,
    ) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::BooleanReceiptLookupProduct,
            type_name::<WorkloadEvidenceBooleanReceiptLookupProduct>(),
        )
    }

    #[cfg(test)]
    pub(crate) fn query_descriptor<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::QueryDescriptor,
            type_name::<T>(),
        )
    }

    #[cfg(test)]
    pub(crate) fn topology_proof<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::TopologyProof,
            type_name::<T>(),
        )
    }

    #[cfg(test)]
    pub(crate) fn schema_vocabulary<T: 'static>(_: &T) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchRejectedInputKind::SchemaVocabulary,
            type_name::<T>(),
        )
    }

    #[cfg(test)]
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
