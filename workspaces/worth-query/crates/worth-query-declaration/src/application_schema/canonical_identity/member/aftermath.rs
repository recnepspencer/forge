use crate::application_aftermath::{
    DeclaredCorrectionAuthority, PortableApplicationAftermathContract, PortableCorrectionMechanism,
};
use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;

pub(super) fn append_declared_aftermath(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &PortableApplicationAftermathContract,
) {
    append_authority(basis, prefix, contract.authority());
    append_mechanism(basis, prefix, contract.mechanism());
    basis.text(
        format!("{prefix}.reconciliation"),
        contract
            .reconciliation()
            .map_or("none", |procedure| procedure.procedure_slot()),
    );
}

fn append_authority(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    authority: DeclaredCorrectionAuthority,
) {
    basis.text(
        format!("{prefix}.authority"),
        match authority {
            DeclaredCorrectionAuthority::RuntimeAlone => "runtime-alone",
            DeclaredCorrectionAuthority::RuntimeWithExternalOwner => "runtime-with-external-owner",
            DeclaredCorrectionAuthority::NotCorrectable => "not-correctable",
        },
    );
}

fn append_mechanism(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    mechanism: Option<&PortableCorrectionMechanism>,
) {
    match mechanism {
        Some(PortableCorrectionMechanism::RecordedInverse(inverse)) => {
            basis.text(format!("{prefix}.mechanism"), "recorded-inverse");
            basis.text(
                format!("{prefix}.inverse-operation"),
                inverse.inverse_operation_slot(),
            );
            basis.text(
                format!("{prefix}.lowering"),
                inverse.lowering_correspondence().correspondence_slot(),
            );
            basis.usize(
                format!("{prefix}.preimage-byte-bound"),
                inverse.preimage_demand().maximum_encoded_bytes(),
            );
            for (index, locus) in inverse.preimage_demand().loci().iter().enumerate() {
                basis.text(format!("{prefix}.preimage-{index}.entity"), locus.entity());
                basis.text(format!("{prefix}.preimage-{index}.aspect"), locus.aspect());
                basis.text(format!("{prefix}.preimage-{index}.field"), locus.field());
            }
        }
        Some(PortableCorrectionMechanism::Compensation(compensation)) => {
            basis.text(format!("{prefix}.mechanism"), "compensation");
            basis.text(
                format!("{prefix}.compensating-operation"),
                compensation.compensating_operation_slot(),
            );
        }
        None => basis.text(format!("{prefix}.mechanism"), "none"),
    }
}
