use worth_query::facade::inspection::declare;

fn cannot_inspect_digest(digest: &str) {
    let _declaration = declare(digest);
}

fn main() {}
