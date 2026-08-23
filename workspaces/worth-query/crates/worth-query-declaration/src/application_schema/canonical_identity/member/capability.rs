use worth_foundational::facade::CanonicalBasisEntryKind;

use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;
use crate::application_schema::canonical_capability_identity::append_capability_contract;
use crate::application_schema::ApplicationSchemaMember;

use super::super::APPLICATION_SCHEMA_DOMAIN;

pub(super) fn append_capability_member(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    match member {
        ApplicationSchemaMember::ApplicationQuery { definition } => {
            basis.text(format!("{prefix}.kind"), "application-query");
            basis.extend(definition.canonical_basis().embedded_entries(
                APPLICATION_SCHEMA_DOMAIN,
                &format!("{prefix}.query-meaning"),
                CanonicalBasisEntryKind::Identity,
            ));
        }
        ApplicationSchemaMember::ApplicationCapability { contract } => {
            basis.text(format!("{prefix}.kind"), "application-capability");
            append_capability_contract(basis, &format!("{prefix}.contract"), contract);
        }
        ApplicationSchemaMember::ApplicationCapabilityContext {
            context,
            context_type,
        } => {
            basis.text(format!("{prefix}.kind"), "application-capability-context");
            basis.text(format!("{prefix}.context"), context);
            basis.text(format!("{prefix}.context-type"), context_type);
        }
        ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
            context,
            context_type,
            slot,
            slot_type,
            entity,
        } => {
            basis.text(
                format!("{prefix}.kind"),
                "application-capability-context-entity-slot",
            );
            basis.text(format!("{prefix}.context"), context);
            basis.text(format!("{prefix}.context-type"), context_type);
            basis.text(format!("{prefix}.slot"), slot);
            basis.text(format!("{prefix}.slot-type"), slot_type);
            basis.text(format!("{prefix}.entity"), entity);
        }
        ApplicationSchemaMember::ApplicationCapabilityProvenance {
            provenance,
            provenance_type,
        } => {
            basis.text(
                format!("{prefix}.kind"),
                "application-capability-provenance",
            );
            basis.text(format!("{prefix}.provenance"), provenance);
            basis.text(format!("{prefix}.provenance-type"), provenance_type);
        }
        _ => unreachable!("capability member router supplied another member family"),
    }
}
