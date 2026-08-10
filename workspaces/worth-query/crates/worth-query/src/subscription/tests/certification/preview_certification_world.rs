use super::*;

pub(super) fn preview_certification<'a>(
    preview: &'a SubscriptionLifecyclePreviewCertificationArtifacts,
) -> SubscriptionLifecyclePreviewCertification<'a> {
    match preview {
        SubscriptionLifecyclePreviewCertificationArtifacts::None => {
            SubscriptionLifecyclePreviewCertification::None
        }
        SubscriptionLifecyclePreviewCertificationArtifacts::Discard {
            isolation,
            residue_report,
            discard_closeout,
        } => SubscriptionLifecyclePreviewCertification::Discard {
            isolation,
            residue_report,
            discard_closeout,
        },
        SubscriptionLifecyclePreviewCertificationArtifacts::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        } => SubscriptionLifecyclePreviewCertification::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        },
    }
}
