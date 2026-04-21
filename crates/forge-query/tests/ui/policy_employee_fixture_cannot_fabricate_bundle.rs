use forge_query::facade::EmployeeRecordCertificationBundle;

fn main() {
    let _ = EmployeeRecordCertificationBundle {
        employee_fixture_digest: "fixture".to_string(),
        scenario_digest: "scenario".to_string(),
        tenant_truth_basis_digest: "truth".to_string(),
        tenant_schema_basis_digest: "schema".to_string(),
        public_field_digest: "public".to_string(),
        masked_field_digest: "masked".to_string(),
    };
}
