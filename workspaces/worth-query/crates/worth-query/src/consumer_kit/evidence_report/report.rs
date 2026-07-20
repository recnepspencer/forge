use std::collections::BTreeMap;

use crate::WorthQueryEvidenceIdentity;

use super::error::{EvidenceReportError, EvidenceReportErrorKind};
use super::field::EvidenceReportField;
use super::identity::EvidenceReportIdentities;
use super::scope::EvidenceReportScope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReport {
    scope: EvidenceReportScope,
    report_name: String,
    fields: Vec<EvidenceReportField>,
    field_index: BTreeMap<String, usize>,
    report_identity: WorthQueryEvidenceIdentity,
    field_inventory_identity: WorthQueryEvidenceIdentity,
    digest_participation_identity: WorthQueryEvidenceIdentity,
}

impl EvidenceReport {
    pub(crate) fn sealed(
        scope: EvidenceReportScope,
        report_name: String,
        fields: Vec<EvidenceReportField>,
        identities: EvidenceReportIdentities,
    ) -> Result<Self, EvidenceReportError> {
        let field_index = build_field_index(&fields)?;
        Ok(Self {
            scope,
            report_name,
            fields,
            field_index,
            report_identity: identities.report_identity,
            field_inventory_identity: identities.field_inventory_identity,
            digest_participation_identity: identities.digest_participation_identity,
        })
    }

    pub fn scope(&self) -> &EvidenceReportScope {
        &self.scope
    }

    pub fn report_name(&self) -> &str {
        &self.report_name
    }

    pub fn fields(&self) -> &[EvidenceReportField] {
        &self.fields
    }

    pub fn field(&self, name: &str) -> Result<&EvidenceReportField, EvidenceReportError> {
        self.field_index
            .get(name)
            .map(|index| &self.fields[*index])
            .ok_or_else(|| {
                EvidenceReportError::new(
                    EvidenceReportErrorKind::FieldNotFound,
                    format!("evidence report field `{name}` was not found"),
                )
            })
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn field_inventory_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.field_inventory_identity
    }

    pub fn digest_participation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.digest_participation_identity
    }

    pub fn indexed_field_count(&self) -> usize {
        self.field_index.len()
    }
}

fn build_field_index(
    fields: &[EvidenceReportField],
) -> Result<BTreeMap<String, usize>, EvidenceReportError> {
    let mut field_index = BTreeMap::new();
    for (index, field) in fields.iter().enumerate() {
        if field_index
            .insert(field.name().to_string(), index)
            .is_some()
        {
            return Err(EvidenceReportError::new(
                EvidenceReportErrorKind::DuplicateFieldName,
                format!("duplicate evidence report field `{}`", field.name()),
            ));
        }
    }
    Ok(field_index)
}
