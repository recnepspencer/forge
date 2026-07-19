use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EmployeeRecordTenantVariant {
    TenantAlpha,
    TenantBeta,
}

impl EmployeeRecordTenantVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TenantAlpha => "tenant_alpha",
            Self::TenantBeta => "tenant_beta",
        }
    }

    pub fn tenant_truth_basis_digest(&self) -> String {
        hash_parts(&[format!("employee_record_truth:{}", self.as_str())])
    }

    pub fn tenant_schema_basis_digest(&self) -> String {
        hash_parts(&[format!("employee_record_schema:{}", self.as_str())])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EmployeeRecordQueryFamily {
    DirectDetail,
    CollectionOrderedByDisplayName,
    FilterBySalaryBand,
    OrderBySalaryBand,
    GroupBySalaryBand,
    AggregateSalaryBand,
    CursorBySalaryBand,
    ViewMembershipBySalaryBand,
    LiveRelevanceBySalaryBand,
    SavedQueryReuse,
    RuntimeHistoricalRead,
}

impl EmployeeRecordQueryFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectDetail => "direct_detail",
            Self::CollectionOrderedByDisplayName => "collection_ordered_by_display_name",
            Self::FilterBySalaryBand => "filter_by_salary_band",
            Self::OrderBySalaryBand => "order_by_salary_band",
            Self::GroupBySalaryBand => "group_by_salary_band",
            Self::AggregateSalaryBand => "aggregate_salary_band",
            Self::CursorBySalaryBand => "cursor_by_salary_band",
            Self::ViewMembershipBySalaryBand => "view_membership_by_salary_band",
            Self::LiveRelevanceBySalaryBand => "live_relevance_by_salary_band",
            Self::SavedQueryReuse => "saved_query_reuse",
            Self::RuntimeHistoricalRead => "runtime_historical_read",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeRecordPolicyScenario {
    tenant_variant: EmployeeRecordTenantVariant,
    query_family: EmployeeRecordQueryFamily,
    masked_field: &'static str,
}

impl EmployeeRecordPolicyScenario {
    pub fn new(
        tenant_variant: EmployeeRecordTenantVariant,
        query_family: EmployeeRecordQueryFamily,
    ) -> Self {
        Self {
            tenant_variant,
            query_family,
            masked_field: "compensation.salary_band",
        }
    }

    pub fn tenant_variant(&self) -> EmployeeRecordTenantVariant {
        self.tenant_variant
    }

    pub fn query_family(&self) -> EmployeeRecordQueryFamily {
        self.query_family
    }

    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("tenant:{}", self.tenant_variant.as_str()),
            format!("query_family:{}", self.query_family.as_str()),
            format!("masked:{}", self.masked_field),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeRecordCertificationBundle {
    employee_fixture_digest: String,
    scenario_digest: String,
    tenant_truth_basis_digest: String,
    tenant_schema_basis_digest: String,
    public_field_digest: String,
    masked_field_digest: String,
}

impl EmployeeRecordCertificationBundle {
    pub(crate) fn new(
        fixture: &EmployeeRecordPolicyFixture,
        scenario: &EmployeeRecordPolicyScenario,
    ) -> Self {
        Self {
            employee_fixture_digest: fixture.digest().to_string(),
            scenario_digest: scenario.digest(),
            tenant_truth_basis_digest: scenario.tenant_variant().tenant_truth_basis_digest(),
            tenant_schema_basis_digest: scenario.tenant_variant().tenant_schema_basis_digest(),
            public_field_digest: hash_parts(fixture.public_fields()),
            masked_field_digest: hash_parts(&[fixture.masked_field().to_string()]),
        }
    }

    pub fn employee_fixture_digest(&self) -> &str {
        &self.employee_fixture_digest
    }

    pub fn scenario_digest(&self) -> &str {
        &self.scenario_digest
    }

    pub fn tenant_truth_basis_digest(&self) -> &str {
        &self.tenant_truth_basis_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn public_field_digest(&self) -> &str {
        &self.public_field_digest
    }

    pub fn masked_field_digest(&self) -> &str {
        &self.masked_field_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeRecordPolicyFixture {
    public_fields: Vec<String>,
    masked_field: String,
    relationship_proofs: Vec<String>,
    digest: String,
}

impl EmployeeRecordPolicyFixture {
    pub(crate) fn new() -> Self {
        let public_fields = vec![
            "employee.employee_id".to_string(),
            "profile.display_name".to_string(),
            "organization.department".to_string(),
            "organization.manager_id".to_string(),
        ];
        let masked_field = "compensation.salary_band".to_string();
        let relationship_proofs = vec![
            "Team -> owns -> EmployeeRecord".to_string(),
            "Reviewer -> may_review -> EmployeeRecord".to_string(),
        ];
        let mut parts = vec!["employee_record_policy_fixture".to_string()];
        parts.extend(public_fields.iter().map(|field| format!("public:{field}")));
        parts.push(format!("masked:{masked_field}"));
        parts.extend(
            relationship_proofs
                .iter()
                .map(|proof| format!("proof:{proof}")),
        );
        let digest = hash_parts(&parts);
        Self {
            public_fields,
            masked_field,
            relationship_proofs,
            digest,
        }
    }

    pub fn public_fields(&self) -> &[String] {
        &self.public_fields
    }

    pub fn masked_field(&self) -> &str {
        &self.masked_field
    }

    pub fn relationship_proofs(&self) -> &[String] {
        &self.relationship_proofs
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn certify(
        &self,
        scenario: EmployeeRecordPolicyScenario,
    ) -> EmployeeRecordCertificationBundle {
        EmployeeRecordCertificationBundle::new(self, &scenario)
    }
}

pub fn employee_record_policy_fixture() -> EmployeeRecordPolicyFixture {
    EmployeeRecordPolicyFixture::new()
}
