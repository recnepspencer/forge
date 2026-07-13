use worth_query::facade::read::{current, declare};

fn main() {
    let request = declare(|_| unreachable!()).unwrap().using(current());
    request.run_with_plan("consumer-seeded-plan");
}
