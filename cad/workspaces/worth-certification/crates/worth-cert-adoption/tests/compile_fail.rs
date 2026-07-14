#[path = "support/boundary_check_specimen.rs"]
mod boundary_check_specimen;
#[path = "support/compiler_specimen.rs"]
mod compiler_specimen;
#[path = "support/corpus/mod.rs"]
mod corpus_contract;
#[path = "support/entry_governed_repository.rs"]
mod entry_governed_repository;
#[path = "support/governed_config.rs"]
mod governed_config;
#[path = "support/query_audience_repository.rs"]
mod query_audience_repository;
#[path = "support/synthetic_repository_filesystem.rs"]
mod synthetic_repository_filesystem;

#[test]
fn adoption_constitution_rejects_every_registered_hostile_case() {
    let corpus = corpus_contract::Corpus::load();
    corpus.assert_exact_files_and_facade_pairing();
    compiler_specimen::run_compiler_cases(&corpus);
    boundary_check_specimen::run_boundary_check_cases(&corpus);
}

#[test]
fn deleting_any_specimen_breaks_the_corpus_contract() {
    let corpus = corpus_contract::Corpus::load();
    for specimen in corpus.rows() {
        let inventory_error = corpus
            .validate_inventory_without(specimen.path)
            .expect_err("every registered specimen must be inventory-load-bearing");
        assert!(
            inventory_error.contains(specimen.path),
            "unexpected inventory error: {inventory_error}"
        );
        let error = corpus
            .validate_physical_corpus_without(specimen.path)
            .expect_err("every specimen must be constitutionally load-bearing");
        assert!(
            error.contains("constitutional obligation unpaired"),
            "deletion must fail constitutional pairing, not inventory first: {error}"
        );
    }
}
