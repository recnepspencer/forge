use worth_query::facade::read::{current, declare};

fn main() {
    let request = declare(|_| unreachable!()).unwrap().using(current());
    request.execute_parallel();
}
