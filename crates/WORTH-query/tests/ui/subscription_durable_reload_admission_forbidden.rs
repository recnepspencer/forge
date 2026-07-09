use worth_query::facade::QuerySubscriptionAdmissionBudget;

fn main() {
    let _budget =
        QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1).with_durable_reload_request();
}
