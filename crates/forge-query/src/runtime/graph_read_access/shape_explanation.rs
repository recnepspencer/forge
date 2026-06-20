use super::{ForgeQueryAdmittedQuerySchemaReferences, ForgeQueryGraphReadAccessShape};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessShapeDerivationCounters {
    schema_reference_rows_admitted: usize,
    operations_resolved: usize,
    access_shapes_derived: usize,
    derivation_only: bool,
}

impl ForgeQueryGraphReadAccessShapeDerivationCounters {
    pub fn schema_reference_rows_admitted(&self) -> usize {
        self.schema_reference_rows_admitted
    }

    pub fn operations_resolved(&self) -> usize {
        self.operations_resolved
    }

    pub fn access_shapes_derived(&self) -> usize {
        self.access_shapes_derived
    }

    pub fn is_derivation_only(&self) -> bool {
        self.derivation_only
    }

    pub(crate) fn from_shape(access_shape: &ForgeQueryGraphReadAccessShape) -> Self {
        Self {
            schema_reference_rows_admitted: access_shape
                .operation_resolution()
                .admitted_reference_count(),
            operations_resolved: access_shape.operation_resolution().operations().len(),
            access_shapes_derived: 1,
            derivation_only: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessShapeExplanation {
    read_family_digest: String,
    access_shape: ForgeQueryGraphReadAccessShape,
    derivation_counters: ForgeQueryGraphReadAccessShapeDerivationCounters,
}

impl ForgeQueryGraphReadAccessShapeExplanation {
    pub fn read_family_digest(&self) -> &str {
        &self.read_family_digest
    }

    pub fn access_shape(&self) -> &ForgeQueryGraphReadAccessShape {
        &self.access_shape
    }

    pub fn derivation_counters(&self) -> &ForgeQueryGraphReadAccessShapeDerivationCounters {
        &self.derivation_counters
    }

    pub fn admitted_schema_references(&self) -> &ForgeQueryAdmittedQuerySchemaReferences {
        self.access_shape.operation_resolution().references()
    }

    pub fn explain(&self) -> String {
        format!(
            "family={} shape={} root={} scope={} operators={} max_depth={} fanout={} predicates={} ordering={} result={} relationship_proof={}",
            self.read_family_digest,
            self.access_shape.digest().as_str(),
            self.access_shape.root_posture().as_str(),
            self.access_shape.scope_class().as_str(),
            self.access_shape
                .traversal_operators()
                .iter()
                .map(|operator| operator.as_str())
                .collect::<Vec<_>>()
                .join(","),
            self.access_shape.max_depth(),
            self.access_shape.fanout_posture().as_str(),
            self.access_shape.predicate_family().as_str(),
            self.access_shape.ordering_posture().as_str(),
            self.access_shape.result_pressure().as_str(),
            self.access_shape.relationship_proof_posture().as_str()
        )
    }

    pub(crate) fn from_shape(
        read_family_digest: impl Into<String>,
        access_shape: ForgeQueryGraphReadAccessShape,
    ) -> Self {
        let derivation_counters =
            ForgeQueryGraphReadAccessShapeDerivationCounters::from_shape(&access_shape);
        Self {
            read_family_digest: read_family_digest.into(),
            access_shape,
            derivation_counters,
        }
    }
}
