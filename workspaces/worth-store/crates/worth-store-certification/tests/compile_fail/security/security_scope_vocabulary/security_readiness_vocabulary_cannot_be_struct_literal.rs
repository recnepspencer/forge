use worth_store_security::{
    StoreSecurityReadinessVocabulary, StoreSecurityReadinessVocabularyTerm,
};

fn main() {
    let _forged = StoreSecurityReadinessVocabulary {
        term: StoreSecurityReadinessVocabularyTerm::SecurityFoundationReadiness,
    };
}
