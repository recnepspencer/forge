use worth_relational::facade::transactions::TransactionOptions;

fn main() {
    let _ = TransactionOptions {
        allow_nested_savepoints: true,
    };
}
