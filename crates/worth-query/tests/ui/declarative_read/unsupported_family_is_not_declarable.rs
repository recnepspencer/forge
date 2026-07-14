use worth_query::facade::read::declare;

fn main() {
    let _ = declare(|read| read.unbounded_stream("user"));
}
