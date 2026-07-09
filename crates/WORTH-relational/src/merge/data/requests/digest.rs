use sha2::{Digest, Sha256};

use crate::merge::data::{
    MergeIntent, NormalizedRelationalMergeRequest, RelationalMergeCorrespondencePosture,
    RelationalMergeRequestFamily, RelationalMergeSchemaReconciliationPosture, RelationalMergeScope,
    RelationalMergeTopologyIntent,
};

pub(crate) fn normalized_merge_request_digest(
    request: &NormalizedRelationalMergeRequest,
) -> String {
    let mut bytes = DigestBytes::new("WORTH.relational.merge.normalized-request.v1");
    bytes.normalized_request(request);
    bytes.finish()
}

struct DigestBytes {
    bytes: Vec<u8>,
}

impl DigestBytes {
    fn new(domain: &'static str) -> Self {
        let mut bytes = Self { bytes: Vec::new() };
        bytes.str(domain);
        bytes
    }

    fn finish(self) -> String {
        let digest = Sha256::digest(self.bytes);
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn tag(&mut self, tag: u8) {
        self.bytes.push(tag);
    }

    fn usize(&mut self, value: usize) {
        self.bytes.extend_from_slice(&(value as u64).to_be_bytes());
    }

    fn str(&mut self, value: &str) {
        self.usize(value.len());
        self.bytes.extend_from_slice(value.as_bytes());
    }

    fn normalized_request(&mut self, request: &NormalizedRelationalMergeRequest) {
        self.request_family(request.family());
        self.str(&request.target_branch().0);
        self.str(&request.source_branch().0);
        self.merge_intent(request.merge_intent());
        self.correspondence_posture(request.correspondence_posture());
        self.schema_posture(request.schema_reconciliation_posture());
        self.topology_intent(request.topology_intent());
    }

    fn request_family(&mut self, family: RelationalMergeRequestFamily) {
        match family {
            RelationalMergeRequestFamily::FullBranchReconciliation => self.tag(1),
        }
    }

    fn merge_intent(&mut self, intent: MergeIntent) {
        match intent {
            MergeIntent::ReconcileIntoTarget => self.tag(1),
        }
    }

    fn correspondence_posture(&mut self, posture: RelationalMergeCorrespondencePosture) {
        match posture {
            RelationalMergeCorrespondencePosture::Advisory => self.tag(1),
            RelationalMergeCorrespondencePosture::Strict => self.tag(2),
        }
    }

    fn schema_posture(&mut self, posture: RelationalMergeSchemaReconciliationPosture) {
        match posture {
            RelationalMergeSchemaReconciliationPosture::Participate => self.tag(1),
            RelationalMergeSchemaReconciliationPosture::RequireCompatibility => self.tag(2),
        }
    }

    fn topology_intent(&mut self, intent: RelationalMergeTopologyIntent) {
        match intent {
            RelationalMergeTopologyIntent::PreserveTopologySemantics => self.tag(1),
            RelationalMergeTopologyIntent::RequireStrictTopologyStability => self.tag(2),
        }
    }

    #[allow(dead_code)]
    fn scope(&mut self, scope: RelationalMergeScope) {
        match scope {
            RelationalMergeScope::FullBranch => self.tag(1),
        }
    }
}
