use forge_query::facade::{
    AdmittedPolicyTenantContext, PolicyBasis, PolicyTenantAdmissionBundle, TenantSchemaBasis,
    TenantTruthBasis,
};

fn policy_basis() -> PolicyBasis {
    todo!()
}

fn tenant_truth_basis() -> TenantTruthBasis {
    todo!()
}

fn tenant_schema_basis() -> TenantSchemaBasis {
    todo!()
}

fn bundle() -> PolicyTenantAdmissionBundle {
    todo!()
}

fn main() {
    let _ = AdmittedPolicyTenantContext {
        policy_basis: policy_basis(),
        tenant_truth_basis: tenant_truth_basis(),
        tenant_schema_basis: tenant_schema_basis(),
        bundle: bundle(),
    };
}
