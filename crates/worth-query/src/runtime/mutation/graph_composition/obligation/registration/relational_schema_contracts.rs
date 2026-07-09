use worth_relational::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};

use crate::runtime::{
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationDenial, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphTouchSelector,
};

pub(crate) fn registrations_from_relational_invariant_catalog(
    catalog: &InvariantCatalog,
) -> Result<Vec<WorthQueryGraphObligationRegistration>, WorthQueryGraphObligationRegistrationDenial>
{
    catalog
        .canonicalized()
        .registrations
        .iter()
        .filter(|registration| is_schema_lowered_registration(registration))
        .map(registration_from_schema_lowered_invariant)
        .collect()
}

fn is_schema_lowered_registration(registration: &InvariantRegistration) -> bool {
    matches!(
        registration.rule,
        InvariantRule::EndpointKindContract(_)
            | InvariantRule::CardinalityMaximumContract(_)
            | InvariantRule::CardinalityMinimumContract(_)
            | InvariantRule::UniquenessContract(_)
            | InvariantRule::SymmetryContract(_)
            | InvariantRule::EndpointDeletionIntegrityContract(_)
            | InvariantRule::AcyclicityContract(_)
            | InvariantRule::PartitionIsolationContract(_)
            | InvariantRule::ConnectivityMinimumContract(_)
    )
}

fn registration_from_schema_lowered_invariant(
    registration: &InvariantRegistration,
) -> Result<WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRegistrationDenial> {
    Ok(
        WorthQueryGraphObligationRegistration::schema_contract_validator(
            schema_contract_rule_identity(&registration.rule),
            schema_contract_touch_selector(&registration.rule)?,
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
    )
}

fn schema_contract_rule_identity(rule: &InvariantRule) -> WorthQueryGraphObligationRuleIdentity {
    WorthQueryGraphObligationRuleIdentity::new(
        "relational-schema-contract",
        schema_contract_rule_name(rule),
        "v1",
    )
    .expect("relational schema contract rule identity is static and non-empty")
}

fn schema_contract_rule_name(rule: &InvariantRule) -> String {
    format!(
        "{}:{}",
        schema_contract_rule_family(rule),
        schema_contract_id(rule)
    )
}

fn schema_contract_touch_selector(
    rule: &InvariantRule,
) -> Result<WorthQueryGraphTouchSelector, WorthQueryGraphObligationRegistrationDenial> {
    Ok(WorthQueryGraphTouchSelector::relation_kind_id(
        schema_contract_relation_kind_id(rule),
    ))
}

fn schema_contract_rule_family(rule: &InvariantRule) -> &'static str {
    match rule {
        InvariantRule::EndpointKindContract(_) => "endpoint-kind",
        InvariantRule::CardinalityMaximumContract(_) => "cardinality-maximum",
        InvariantRule::CardinalityMinimumContract(_) => "cardinality-minimum",
        InvariantRule::UniquenessContract(_) => "uniqueness",
        InvariantRule::SymmetryContract(_) => "symmetry",
        InvariantRule::EndpointDeletionIntegrityContract(_) => "endpoint-deletion-integrity",
        InvariantRule::AcyclicityContract(_) => "acyclicity",
        InvariantRule::PartitionIsolationContract(_) => "partition-isolation",
        InvariantRule::ConnectivityMinimumContract(_) => "connectivity-minimum",
        _ => "non-schema-contract",
    }
}

fn schema_contract_id(rule: &InvariantRule) -> &str {
    match rule {
        InvariantRule::EndpointKindContract(contract) => contract.contract_id.as_str(),
        InvariantRule::CardinalityMaximumContract(contract) => contract.contract_id.as_str(),
        InvariantRule::CardinalityMinimumContract(contract) => contract.contract_id.as_str(),
        InvariantRule::UniquenessContract(contract) => contract.contract_id.as_str(),
        InvariantRule::SymmetryContract(contract) => contract.contract_id.as_str(),
        InvariantRule::EndpointDeletionIntegrityContract(contract) => contract.contract_id.as_str(),
        InvariantRule::AcyclicityContract(contract) => contract.contract_id.as_str(),
        InvariantRule::PartitionIsolationContract(contract) => contract.contract_id.as_str(),
        InvariantRule::ConnectivityMinimumContract(contract) => contract.contract_id.as_str(),
        _ => "non-schema-contract",
    }
}

fn schema_contract_relation_kind_id(rule: &InvariantRule) -> u32 {
    match rule {
        InvariantRule::EndpointKindContract(contract) => contract.relation_kind_id.0,
        InvariantRule::CardinalityMaximumContract(contract) => contract.relation_kind_id.0,
        InvariantRule::CardinalityMinimumContract(contract) => contract.relation_kind_id.0,
        InvariantRule::UniquenessContract(contract) => contract.relation_kind_id.0,
        InvariantRule::SymmetryContract(contract) => contract.relation_kind_id.0,
        InvariantRule::EndpointDeletionIntegrityContract(contract) => contract.relation_kind_id.0,
        InvariantRule::AcyclicityContract(contract) => contract.relation_kind_id.0,
        InvariantRule::PartitionIsolationContract(contract) => contract.relation_kind_id.0,
        InvariantRule::ConnectivityMinimumContract(contract) => contract.relation_kind_id.0,
        _ => 0,
    }
}
