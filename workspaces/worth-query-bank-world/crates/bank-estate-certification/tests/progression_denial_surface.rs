use bank_server::{
    BankCapabilityDelegationProjectionDenial, BankCapabilityRevocationProjectionDenial,
    BankEstateLifecycleProjectionDenial, BankEstateProgressionDenial,
};

#[test]
fn nested_progression_denial_payloads_are_nameable_by_external_consumers() {
    fn require_public_type<T>() {}

    require_public_type::<BankEstateProgressionDenial>();
    require_public_type::<BankEstateLifecycleProjectionDenial>();
    require_public_type::<BankCapabilityDelegationProjectionDenial>();
    require_public_type::<BankCapabilityRevocationProjectionDenial>();
}
