use super::*;

const MAXIMUM_CAPABILITY_CONTRACTS: usize = 1_024;

#[test]
fn capability_contract_population_has_an_exact_installation_ceiling() {
    assert_eq!(
        build_from_members(members_with_contracts(MAXIMUM_CAPABILITY_CONTRACTS)),
        Ok(())
    );
    assert_eq!(
        build_from_members(members_with_contracts(MAXIMUM_CAPABILITY_CONTRACTS + 1)),
        Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
    );
}

fn members_with_contracts(count: usize) -> Vec<ApplicationSchemaMember> {
    let mut members = members(contract(false, false, true));
    members.retain(|member| {
        !matches!(
            member,
            ApplicationSchemaMember::ApplicationCapability { .. }
        )
    });
    members.extend(
        (0..count).map(|ordinal| ApplicationSchemaMember::ApplicationCapability {
            contract: contract_with_name_and_composition(
                Box::leak(format!("Capability{ordinal}").into_boxed_str()),
                false,
                false,
                composition(true),
            ),
        }),
    );
    members
}
