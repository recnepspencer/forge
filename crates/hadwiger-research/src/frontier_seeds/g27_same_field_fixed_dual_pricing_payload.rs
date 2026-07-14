use crate::domain_artifacts::digest_basis::HadwigerArtifactPayloadEntry;

use super::g27_same_field_fixed_dual_pricing::{
    G27FixedDualPricingChannel, G27FixedDualPricingPosture,
};

pub(super) fn conclusion(
    posture: G27FixedDualPricingPosture,
    top: Option<&G27FixedDualPricingChannel>,
) -> String {
    let top = top
        .map(G27FixedDualPricingChannel::stable_token)
        .unwrap_or_else(|| "none".to_string());
    match posture {
        G27FixedDualPricingPosture::FundMasterDualScaleAudit => format!(
            "fund master-dual scale audit: exact compatible-W MWIS reaches the retained global W alpha-weight, so contact incidence does not collapse under W independence ({top})"
        ),
        G27FixedDualPricingPosture::RetiredMwisCollapse => format!(
            "retire same-field fixed-dual route at this interface: clique-cover upper bounds put every priced compatible-W channel below the retained global W alpha-weight ({top})"
        ),
        G27FixedDualPricingPosture::NeedsStrongerMwisCertificate => format!(
            "continue only with a stronger MWIS certificate: compatible-W lower bounds do not reach global alpha, but current upper bounds do not retire the route ({top})"
        ),
    }
}

pub(super) fn payload(
    priced_tight_atom_count: usize,
    g27_slack_denominator_digits: usize,
    w_global_alpha_weight: i128,
    top_channels: &[G27FixedDualPricingChannel],
    posture: G27FixedDualPricingPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_same_field_fixed_dual_pricing.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("g27_anchor", 23),
        HadwigerArtifactPayloadEntry::unsigned("w_anchor", 254),
        HadwigerArtifactPayloadEntry::unsigned(
            "priced_tight_atom_count",
            priced_tight_atom_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "g27_slack_denominator_digits",
            g27_slack_denominator_digits as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "w_global_alpha_weight",
            w_global_alpha_weight as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for channel in top_channels {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "fixed_dual_channel",
            channel.stable_token(),
        ));
    }
    payload
}
