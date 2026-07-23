use crate::basis_lifecycle::BasisOperationLane;

use super::WorthQuerySettledDomainProjection;

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledDomainProjection<D, O, F, L> {
    pub(crate) fn collection_delivery_contract_identity(&self) -> Option<String> {
        let canonical = self.consumer_contract().canonical_projection();
        let ordering_identity = crate::identity::hash_parts(
            &canonical
                .query()
                .ordering()
                .iter()
                .map(|entry| entry.digest_part())
                .collect::<Vec<_>>(),
        );
        Some(crate::identity::hash_parts(&[
            "worth_query_collection_delivery_contract_v1".to_string(),
            format!("binding:{}", self.bound_operation().binding_identity()),
            format!("basis:{}", self.consumer_contract().basis_identity()),
            format!("shape:{}", canonical.result_shape().digest().as_str()),
            format!("ordering:{ordering_identity}"),
            format!(
                "native:{}",
                self.native_access_layout()?.semantic_identity()
            ),
        ]))
    }
}
