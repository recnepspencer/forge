use worth_query_decl::facade::{
    worth_query_capability, worth_query_capability_context,
    worth_query_capability_context_entity_slot, worth_query_capability_provenance,
    worth_query_operation,
};

use crate::estate::EstateAction;
use crate::schema::{BankSchema, EmergencyAccess, LegalAuthority, MandatoryReview};

worth_query_capability_context!(pub EstateActionContext in BankSchema);
worth_query_capability_context_entity_slot!(
    pub EstateLegalAuthoritySlot in BankSchema,
    EstateActionContext => LegalAuthority
);
worth_query_capability_context_entity_slot!(
    pub EstateEmergencyAccessSlot in BankSchema,
    EstateActionContext => EmergencyAccess
);
worth_query_capability_context_entity_slot!(
    pub EstateMandatoryReviewSlot in BankSchema,
    EstateActionContext => MandatoryReview
);
worth_query_capability_provenance!(pub EstateGrantChainProvenance in BankSchema);

worth_query_operation!(pub NotifyDeathEstateOperation(EstateAction) in BankSchema);
worth_query_operation!(pub FreezeEstateAccountOperation(EstateAction) in BankSchema);
worth_query_operation!(pub OpenEstateCaseOperation(EstateAction) in BankSchema);
worth_query_operation!(pub RecognizeEstateExecutorOperation(EstateAction) in BankSchema);
worth_query_operation!(pub DelegateEstateCapabilityOperation(EstateAction) in BankSchema);
worth_query_operation!(pub RevokeEstateCapabilityOperation(EstateAction) in BankSchema);
worth_query_operation!(pub RequestEstateEmergencyAccessOperation(EstateAction) in BankSchema);
worth_query_operation!(pub ApproveEstateEmergencyAccessOperation(EstateAction) in BankSchema);
worth_query_operation!(pub RevokeEstateEmergencyAccessOperation(EstateAction) in BankSchema);
worth_query_operation!(pub CompleteEstateMandatoryReviewOperation(EstateAction) in BankSchema);
worth_query_operation!(pub ReleaseEstateOperation(EstateAction) in BankSchema);
worth_query_operation!(pub DisburseEstateOperation(EstateAction) in BankSchema);
worth_query_operation!(pub ViewRestrictedEstateOperation(EstateAction) in BankSchema);

worth_query_capability!(pub NotifyDeathEstateCapability in BankSchema);
worth_query_capability!(pub FreezeEstateAccountCapability in BankSchema);
worth_query_capability!(pub OpenEstateCaseCapability in BankSchema);
worth_query_capability!(pub RecognizeEstateExecutorCapability in BankSchema);
worth_query_capability!(pub DelegateEstateCapability in BankSchema);
worth_query_capability!(pub RevokeEstateCapability in BankSchema);
worth_query_capability!(pub RequestEstateEmergencyAccessCapability in BankSchema);
worth_query_capability!(pub ApproveEstateEmergencyAccessCapability in BankSchema);
worth_query_capability!(pub RevokeEstateEmergencyAccessCapability in BankSchema);
worth_query_capability!(pub CompleteEstateMandatoryReviewCapability in BankSchema);
worth_query_capability!(pub ReleaseEstateCapability in BankSchema);
worth_query_capability!(pub DisburseEstateCapability in BankSchema);
worth_query_capability!(pub ViewEstateAdministrationCapability in BankSchema);
worth_query_capability!(pub ViewEstateIdentityVerificationCapability in BankSchema);
worth_query_capability!(pub ViewEstateLegalComplianceCapability in BankSchema);
worth_query_capability!(pub ViewEstateEmergencyProtectionCapability in BankSchema);
worth_query_capability!(pub ViewEstateMandatoryReviewCapability in BankSchema);
