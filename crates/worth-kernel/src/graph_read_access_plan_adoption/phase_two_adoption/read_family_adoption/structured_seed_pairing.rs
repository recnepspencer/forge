use crate::graph_read_access_declarations::{
    WorthGraphReadDeclarationReadFamilyIdentity, WorthGraphReadRequirementRowDigestProjection,
};

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionSeedPairing {
    milestone_seven_closeout_digest: String,
    declaration_catalog_digest: String,
    source_catalog_record_digest: String,
    source_requirement_record_digest: String,
    read_family_identity_digest: String,
    requirement_row_digest: String,
    query_family_name: String,
    query_family_digest_seed: String,
    touched_authority_input: String,
    read_family_target: String,
    pairing_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionSeedPairing {
    pub(crate) fn from_seed_rows(
        milestone_seven_closeout_digest: &str,
        declaration_catalog_digest: &str,
        read_family_identity: &WorthGraphReadDeclarationReadFamilyIdentity,
        requirement_row: &WorthGraphReadRequirementRowDigestProjection,
    ) -> Option<Self> {
        if read_family_identity.source_catalog_record_digest()
            != requirement_row.source_catalog_record_digest()
            || read_family_identity.query_family_digest_seed()
                != requirement_row.query_family_digest_seed()
        {
            return None;
        }

        let pairing_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_seed_pairing_v1".to_string(),
            format!("closeout:{milestone_seven_closeout_digest}"),
            format!("catalog:{declaration_catalog_digest}"),
            format!(
                "catalog_record:{}",
                read_family_identity.source_catalog_record_digest()
            ),
            format!("read_family:{}", read_family_identity.identity_digest()),
            format!(
                "requirement_row:{}",
                requirement_row.requirement_row_digest()
            ),
            format!(
                "query_family:{}",
                read_family_identity.query_family_digest_seed()
            ),
            format!(
                "touched_authority:{}",
                read_family_identity.touched_authority_input()
            ),
        ]);

        Some(Self {
            milestone_seven_closeout_digest: milestone_seven_closeout_digest.to_string(),
            declaration_catalog_digest: declaration_catalog_digest.to_string(),
            source_catalog_record_digest: read_family_identity
                .source_catalog_record_digest()
                .to_string(),
            source_requirement_record_digest: requirement_row
                .source_requirement_record_digest()
                .to_string(),
            read_family_identity_digest: read_family_identity.identity_digest().to_string(),
            requirement_row_digest: requirement_row.requirement_row_digest().to_string(),
            query_family_name: read_family_identity.query_family_name().to_string(),
            query_family_digest_seed: read_family_identity.query_family_digest_seed().to_string(),
            touched_authority_input: read_family_identity.touched_authority_input().to_string(),
            read_family_target: read_family_identity.read_family_target().to_string(),
            pairing_digest,
        })
    }

    pub fn milestone_seven_closeout_digest(&self) -> &str {
        &self.milestone_seven_closeout_digest
    }

    pub fn declaration_catalog_digest(&self) -> &str {
        &self.declaration_catalog_digest
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn read_family_identity_digest(&self) -> &str {
        &self.read_family_identity_digest
    }

    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    pub fn query_family_name(&self) -> &str {
        &self.query_family_name
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub fn pairing_digest(&self) -> &str {
        &self.pairing_digest
    }
}
