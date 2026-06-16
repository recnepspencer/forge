use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::super::evidence_projection::subscription_evidence_projection;
use super::bundle::{
    CertificationCoverageReceipt, QuerySubscriptionRuntimeCertificationBundle,
    SubscriptionCertificationCoverageWidth,
};
use super::coverage::QuerySubscriptionFamilyCoverageRow;
use super::scope::QuerySubscriptionRuntimeCertificationScope;

macro_rules! row_projection {
    ($name:ident, $field:ident) => {
        pub fn $name(&self) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
            subscription_evidence_projection(&self.$field)
        }
    };
}

impl QuerySubscriptionRuntimeCertificationScope {
    pub fn scope_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.scope_identity())
    }
}

impl QuerySubscriptionFamilyCoverageRow {
    row_projection!(
        subscription_declaration_projection,
        subscription_declaration_identity
    );
    row_projection!(bridge_declaration_projection, bridge_declaration_identity);
    row_projection!(signal_strategy_projection, signal_strategy_identity);
    row_projection!(support_report_projection, support_report_identity);
    row_projection!(bridge_parity_projection, bridge_parity_identity);
    row_projection!(
        lifecycle_certification_projection,
        lifecycle_certification_identity
    );
    row_projection!(diagnostic_bundle_projection, diagnostic_bundle_identity);
    row_projection!(basis_projection, basis_identity);
    row_projection!(policy_projection, policy_identity);
    row_projection!(tenant_basis_projection, tenant_basis_identity);
    row_projection!(relationship_proof_projection, relationship_proof_identity);
    row_projection!(view_shape_projection, view_shape_identity);
    row_projection!(row_projection, row_identity);

    pub fn query_scope_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.query_scope_identity)
    }

    pub fn failure_projection(
        &self,
    ) -> Option<QueryProjectionIdentity<String, QuerySubscriptionIdentityKind>> {
        self.failure_identity
            .as_ref()
            .map(subscription_evidence_projection)
    }
}

impl SubscriptionCertificationCoverageWidth {
    pub fn width_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.width_identity)
    }
}

impl CertificationCoverageReceipt {
    pub fn receipt_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.receipt_identity)
    }
}

impl QuerySubscriptionRuntimeCertificationBundle {
    row_projection!(
        subscription_declaration_projection,
        subscription_declaration_identity
    );
    row_projection!(bridge_declaration_projection, bridge_declaration_identity);
    row_projection!(signal_strategy_projection, signal_strategy_identity);
    row_projection!(support_report_projection, support_report_identity);
    row_projection!(bridge_parity_projection, bridge_parity_identity);
    row_projection!(diagnostic_bundle_projection, diagnostic_bundle_identity);
    row_projection!(
        lifecycle_certification_projection,
        lifecycle_certification_identity
    );
    row_projection!(hostile_coverage_projection, hostile_coverage_identity);
    row_projection!(
        runtime_certification_bundle_projection,
        runtime_certification_bundle_identity
    );
    row_projection!(counter_snapshot_projection, counter_identity);

    pub fn query_scope_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.query_scope_identity)
    }

    pub fn family_coverage_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.family_coverage_identity)
    }
}
