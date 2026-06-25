use crate::graph_read_access_declarations::WorthGraphReadAdmissionPostureRecord;

use super::super::proof_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationReadFamilyIdentity {
    source_catalog_record_digest: String,
    query_family_name: String,
    query_family_digest_seed: String,
    touched_authority_input: String,
    read_family_target: String,
    identity_digest: String,
}

impl WorthGraphReadDeclarationReadFamilyIdentity {
    pub(crate) fn from_posture_record(record: &WorthGraphReadAdmissionPostureRecord) -> Self {
        let source_catalog_record_digest = record.source_catalog_record_digest().to_string();
        let query_family_name = record.query_family_name().to_string();
        let query_family_digest_seed = record.query_family_digest_seed().to_string();
        let touched_authority_input = record.touched_authority_input().to_string();
        let read_family_target = record.read_family_target().to_string();
        let identity_digest = stable_digest(&[
            "worth_graph_read_declaration_read_family_identity_v1".to_string(),
            format!("catalog_record:{source_catalog_record_digest}"),
            format!("query_family:{query_family_digest_seed}"),
            format!("touched_authority:{touched_authority_input}"),
            format!("read_family_target:{read_family_target}"),
        ]);
        Self {
            source_catalog_record_digest,
            query_family_name,
            query_family_digest_seed,
            touched_authority_input,
            read_family_target,
            identity_digest,
        }
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
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

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    #[cfg(test)]
    pub(crate) fn for_access_plan_adoption_test(
        source_catalog_record_digest: impl Into<String>,
        query_family_name: impl Into<String>,
        query_family_digest_seed: impl Into<String>,
        touched_authority_input: impl Into<String>,
        read_family_target: impl Into<String>,
    ) -> Self {
        let source_catalog_record_digest = source_catalog_record_digest.into();
        let query_family_name = query_family_name.into();
        let query_family_digest_seed = query_family_digest_seed.into();
        let touched_authority_input = touched_authority_input.into();
        let read_family_target = read_family_target.into();
        let identity_digest = stable_digest(&[
            "worth_graph_read_declaration_read_family_identity_v1".to_string(),
            format!("catalog_record:{source_catalog_record_digest}"),
            format!("query_family:{query_family_digest_seed}"),
            format!("touched_authority:{touched_authority_input}"),
            format!("read_family_target:{read_family_target}"),
        ]);
        Self {
            source_catalog_record_digest,
            query_family_name,
            query_family_digest_seed,
            touched_authority_input,
            read_family_target,
            identity_digest,
        }
    }
}
