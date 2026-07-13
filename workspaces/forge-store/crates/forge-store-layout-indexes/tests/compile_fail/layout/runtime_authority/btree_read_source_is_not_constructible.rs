use forge_store_layout_indexes::BaselineBTreeReadSource;

fn forge() -> BaselineBTreeReadSource {
    BaselineBTreeReadSource {
        witness: panic!(),
        plan: panic!(),
        protected: panic!(),
    }
}

fn main() {
    let _ = forge();
}
