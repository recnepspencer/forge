use crate::facade::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeHistoricalResolvedLineageIdentity, BridgeHistoricalResolvedRecordIdentity,
    BridgeLineageSourceError, ContinuityLineageSource,
};

#[derive(Debug, Clone, Default)]
pub(super) struct DiagnosticsContinuityLineageSource;

impl ContinuityLineageSource for DiagnosticsContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![BridgeHistoricalResolvedLineageIdentity::new(
                "lineage:diagnostics-successor",
            )],
            vec![BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2")],
            vec![7],
        )
    }
}
