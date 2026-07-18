#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionCompileFailProof {
    CertificationBundleConstructorPrivate,
    ContractConstructorPrivate,
    ContractHasNoGenericExtract,
    DeclarationConstructorPrivate,
    EnvelopeConstructorPrivate,
    FactSetConstructorPrivate,
    NonAdmittedCannotBindContract,
    RawSourceHasNoConsumedFactAccessors,
    ReceiptConstructorPrivate,
}

impl ProjectionConsumptionCompileFailProof {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CertificationBundleConstructorPrivate => {
                "projection_consumption_certification_bundle_constructor_private"
            }
            Self::ContractConstructorPrivate => {
                "projection_consumption_contract_constructor_private"
            }
            Self::ContractHasNoGenericExtract => {
                "projection_consumption_contract_has_no_generic_extract"
            }
            Self::DeclarationConstructorPrivate => {
                "projection_consumption_declaration_constructor_private"
            }
            Self::EnvelopeConstructorPrivate => {
                "projection_consumption_envelope_constructor_private"
            }
            Self::FactSetConstructorPrivate => {
                "projection_consumption_fact_set_constructor_private"
            }
            Self::NonAdmittedCannotBindContract => {
                "projection_consumption_non_admitted_cannot_bind_contract"
            }
            Self::RawSourceHasNoConsumedFactAccessors => {
                "projection_consumption_raw_source_has_no_consumed_fact_accessors"
            }
            Self::ReceiptConstructorPrivate => "projection_consumption_receipt_constructor_private",
        }
    }
}
