use std::any::TypeId;
use std::sync::Arc;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{
    WorthQueryExecutionGraphReadProduct, WorthQueryExecutionGraphReadStreamEvidence,
    WorthQueryGraphCommitCall, WorthQueryGraphProviderCall, WorthQueryGraphProviderCallKind,
    WorthQueryGraphReceiptAdmissionDenial, WorthQueryProviderWorkReport,
};

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryGraphProjectionEvidence {
    Materialized(Arc<WorthQueryExecutionGraphReadProduct>),
    Streamed(Arc<WorthQueryExecutionGraphReadStreamEvidence>),
}

impl WorthQueryGraphProjectionEvidence {
    fn authority_identity(&self) -> WorthQueryGraphCallAuthorityIdentity {
        match self {
            Self::Materialized(product) => product.authority_identity(),
            Self::Streamed(stream) => stream.authority_identity(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryGraphProviderReceipt {
    authority_identity: WorthQueryGraphCallAuthorityIdentity,
    provider_receipt: Arc<str>,
    projection: Option<WorthQueryGraphProjectionEvidence>,
    work_report: WorthQueryProviderWorkReport,
}

impl WorthQueryGraphProviderReceipt {
    pub(super) fn completed(
        authority_identity: WorthQueryGraphCallAuthorityIdentity,
        provider_receipt: impl Into<Arc<str>>,
        work_report: WorthQueryProviderWorkReport,
    ) -> Self {
        Self {
            authority_identity,
            provider_receipt: provider_receipt.into(),
            projection: None,
            work_report,
        }
    }

    pub(super) fn projected(
        authority_identity: WorthQueryGraphCallAuthorityIdentity,
        provider_receipt: impl Into<Arc<str>>,
        projection: Arc<WorthQueryExecutionGraphReadProduct>,
        work_report: WorthQueryProviderWorkReport,
    ) -> Self {
        Self {
            authority_identity,
            provider_receipt: provider_receipt.into(),
            projection: Some(WorthQueryGraphProjectionEvidence::Materialized(projection)),
            work_report,
        }
    }

    pub(super) fn streamed(
        authority_identity: WorthQueryGraphCallAuthorityIdentity,
        provider_receipt: impl Into<Arc<str>>,
        stream: Arc<WorthQueryExecutionGraphReadStreamEvidence>,
        work_report: WorthQueryProviderWorkReport,
    ) -> Self {
        Self {
            authority_identity,
            provider_receipt: provider_receipt.into(),
            projection: Some(WorthQueryGraphProjectionEvidence::Streamed(stream)),
            work_report,
        }
    }

    pub(super) fn admit_graph_call(
        self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphReceiptAdmissionDenial> {
        if self.authority_identity != call.authority_identity() {
            return Err(WorthQueryGraphReceiptAdmissionDenial::ForeignCall);
        }
        let projection = match (call.kind(), self.projection) {
            (WorthQueryGraphProviderCallKind::Project, None) => {
                return Err(WorthQueryGraphReceiptAdmissionDenial::MissingProjection)
            }
            (WorthQueryGraphProviderCallKind::Project, Some(projection))
                if projection.authority_identity() != call.authority_identity() =>
            {
                return Err(WorthQueryGraphReceiptAdmissionDenial::ProjectionAuthorityMismatch)
            }
            (WorthQueryGraphProviderCallKind::Project, Some(projection)) => Some(projection),
            (_, Some(_)) => {
                return Err(WorthQueryGraphReceiptAdmissionDenial::UnexpectedProjection)
            }
            (_, None) => None,
        };
        let evidence_identity = graph_evidence_identity(call, projection.as_ref());
        Ok(WorthQueryBoundGraphExecutionReceipt {
            role: call.graph_role().to_owned(),
            kind: call.kind(),
            provider_receipt: self.provider_receipt,
            evidence_identity,
            projection,
            work_report: self.work_report,
            commit_authority_identity: None,
            commit_graph_roles: Vec::new(),
        })
    }

    pub(super) fn admit_commit_call(
        self,
        call: &WorthQueryGraphCommitCall,
    ) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphReceiptAdmissionDenial> {
        if self.authority_identity != call.authority_identity() {
            return Err(WorthQueryGraphReceiptAdmissionDenial::ForeignCall);
        }
        if self.projection.is_some() {
            return Err(WorthQueryGraphReceiptAdmissionDenial::UnexpectedProjection);
        }
        let roles = call.graph_roles().to_vec();
        let evidence_identity = Arc::<str>::from(call.call_identity());
        Ok(WorthQueryBoundGraphExecutionReceipt {
            role: format!("commit({})", roles.join(",")),
            kind: WorthQueryGraphProviderCallKind::CommitAdmission,
            provider_receipt: self.provider_receipt,
            evidence_identity,
            projection: None,
            work_report: self.work_report,
            commit_authority_identity: Some(call.commit_authority_identity()),
            commit_graph_roles: roles,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryBoundGraphExecutionReceipt {
    role: String,
    kind: WorthQueryGraphProviderCallKind,
    provider_receipt: Arc<str>,
    evidence_identity: Arc<str>,
    projection: Option<WorthQueryGraphProjectionEvidence>,
    work_report: WorthQueryProviderWorkReport,
    commit_authority_identity: Option<(u64, TypeId)>,
    commit_graph_roles: Vec<String>,
}

impl WorthQueryBoundGraphExecutionReceipt {
    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn kind(&self) -> WorthQueryGraphProviderCallKind {
        self.kind
    }

    pub fn provider_receipt(&self) -> &str {
        &self.provider_receipt
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn graph_read_product(&self) -> Option<&WorthQueryExecutionGraphReadProduct> {
        match self.projection.as_ref() {
            Some(WorthQueryGraphProjectionEvidence::Materialized(product)) => Some(product),
            _ => None,
        }
    }

    pub fn graph_read_stream_evidence(
        &self,
    ) -> Option<&WorthQueryExecutionGraphReadStreamEvidence> {
        match self.projection.as_ref() {
            Some(WorthQueryGraphProjectionEvidence::Streamed(stream)) => Some(stream),
            _ => None,
        }
    }

    pub fn work_report(&self) -> WorthQueryProviderWorkReport {
        self.work_report
    }

    pub fn has_projection_material(&self) -> bool {
        self.projection.is_some()
    }

    pub fn commit_authority_identity(&self) -> Option<(u64, TypeId)> {
        self.commit_authority_identity
    }

    pub fn commit_graph_roles(&self) -> &[String] {
        &self.commit_graph_roles
    }
}

fn graph_evidence_identity(
    call: &WorthQueryGraphProviderCall,
    _projection: Option<&WorthQueryGraphProjectionEvidence>,
) -> Arc<str> {
    Arc::from(call.call_identity())
}
