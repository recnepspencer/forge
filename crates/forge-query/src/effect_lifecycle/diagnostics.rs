use crate::identity::hash_parts;

use super::envelope::SelfDescribingEffectEnvelope;
use super::receipt::EffectExecutionReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectDiagnosticsRequest {
    include_lowered_digest: bool,
    include_counter_snapshot: bool,
    include_integrity_markers: bool,
    include_transition_rules: bool,
    include_source_refs: bool,
}

impl EffectDiagnosticsRequest {
    pub fn forensic() -> Self {
        Self {
            include_lowered_digest: true,
            include_counter_snapshot: true,
            include_integrity_markers: true,
            include_transition_rules: true,
            include_source_refs: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectDiagnosticsMaterialization {
    receipt_digest: String,
    envelope_digest: String,
    detail_sections: Vec<String>,
    diagnostics_digest: String,
}

impl EffectDiagnosticsMaterialization {
    pub(super) fn from_receipt(
        receipt: &EffectExecutionReceipt,
        envelope: &SelfDescribingEffectEnvelope,
        request: EffectDiagnosticsRequest,
    ) -> Self {
        let mut detail_sections = vec![
            format!("family:{}", receipt.declared_effect_family().as_str()),
            format!("authority_lane:{}", receipt.authority_lane().as_str()),
            format!("basis_lane:{}", receipt.basis_lane().as_str()),
            format!("trace:{}", receipt.decision_trace().decision_trace_digest()),
        ];
        if request.include_lowered_digest {
            detail_sections.push(format!(
                "lowered:{}",
                receipt.decision_trace().lowered_digest()
            ));
        }
        if request.include_counter_snapshot {
            detail_sections.push(format!("counters:{}", receipt.delivery_counters().digest()));
        }
        if request.include_integrity_markers {
            detail_sections.push(format!(
                "integrity:{}",
                receipt.integrity_markers().integrity_digest()
            ));
        }
        if request.include_transition_rules {
            detail_sections.push(format!(
                "transitions:{}",
                receipt.transition_rules().rules_digest()
            ));
        }
        if request.include_source_refs {
            detail_sections.push(format!("sources:{}", envelope.sources().sources_digest()));
        }
        let diagnostics_digest = hash_parts(
            &std::iter::once("effect_diagnostics_materialization_v1".to_string())
                .chain(std::iter::once(format!(
                    "receipt:{}",
                    receipt.receipt_digest()
                )))
                .chain(std::iter::once(format!(
                    "envelope:{}",
                    envelope.envelope_digest()
                )))
                .chain(
                    detail_sections
                        .iter()
                        .map(|section| format!("section:{section}")),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            receipt_digest: receipt.receipt_digest().to_string(),
            envelope_digest: envelope.envelope_digest().to_string(),
            detail_sections,
            diagnostics_digest,
        }
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn detail_sections(&self) -> &[String] {
        &self.detail_sections
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }
}
