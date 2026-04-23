use crate::identity::hash_parts;

use super::active_digest::ActiveSubscriptionLaneDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionFanoutPlan {
    lane_digest: ActiveSubscriptionLaneDigest,
    affected_consumer_attachment_width: u64,
    fanout_plan_digest: String,
}

impl SubscriptionFanoutPlan {
    pub(super) fn new(
        lane_digest: ActiveSubscriptionLaneDigest,
        affected_consumer_attachment_width: u64,
    ) -> Self {
        let fanout_plan_digest = hash_parts(&[
            "subscription_fanout_plan_v1".to_string(),
            format!("lane:{}", lane_digest.as_str()),
            format!(
                "affected_consumer_attachment_width:{}",
                affected_consumer_attachment_width
            ),
        ]);
        Self {
            lane_digest,
            affected_consumer_attachment_width,
            fanout_plan_digest,
        }
    }

    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn affected_consumer_attachment_width(&self) -> u64 {
        self.affected_consumer_attachment_width
    }

    pub fn fanout_plan_digest(&self) -> &str {
        &self.fanout_plan_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionFanoutReport {
    plan: SubscriptionFanoutPlan,
    shared_lane_count: u64,
    fanout_width: u64,
    report_digest: String,
}

impl SubscriptionFanoutReport {
    pub(super) fn new(plan: SubscriptionFanoutPlan, shared_lane_count: u64) -> Self {
        let fanout_width = plan.affected_consumer_attachment_width();
        let report_digest = hash_parts(&[
            "subscription_fanout_report_v1".to_string(),
            format!("plan:{}", plan.fanout_plan_digest()),
            format!("shared_lane_count:{}", shared_lane_count),
            format!("fanout_width:{}", fanout_width),
        ]);
        Self {
            plan,
            shared_lane_count,
            fanout_width,
            report_digest,
        }
    }

    pub fn plan(&self) -> &SubscriptionFanoutPlan {
        &self.plan
    }

    pub fn shared_lane_count(&self) -> u64 {
        self.shared_lane_count
    }

    pub fn fanout_width(&self) -> u64 {
        self.fanout_width
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
