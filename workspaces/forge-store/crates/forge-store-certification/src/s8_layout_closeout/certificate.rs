use super::handoffs::preserve_s8_layout_handoff_grammar;
use super::{
    classify_s8_layout_closeout_sources, verify_s8_layout_closeout, S8LayoutCloseoutClassifier,
    S8LayoutCloseoutDenial, S8LayoutCloseoutSources, S8LayoutCloseoutVerifier,
};
use crate::courtroom::replay::s8_layout::S8LayoutReplayBundle;
use forge_store_readiness::S8LayoutHandoffReadiness;

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutCloseoutCertificate {
    sources: S8LayoutCloseoutSources,
    classifier: S8LayoutCloseoutClassifier,
    verifier: S8LayoutCloseoutVerifier,
}

pub fn certify_s8_layout_closeout(
    replay: S8LayoutReplayBundle,
) -> Result<S8LayoutCloseoutCertificate, S8LayoutCloseoutDenial> {
    let sources = super::s8_layout_closeout_sources(replay);
    let classifier = classify_s8_layout_closeout_sources(&sources);
    let verifier = verify_s8_layout_closeout(&sources)?;
    Ok(S8LayoutCloseoutCertificate {
        sources,
        classifier,
        verifier,
    })
}

/// Non-certifying courtroom projection of lower-owned S.9 grammar.
pub fn project_s8_layout_handoff_grammar(
    readiness: S8LayoutHandoffReadiness,
) -> Result<super::S8LayoutCourtroomGrammar, S8LayoutCloseoutDenial> {
    preserve_s8_layout_handoff_grammar(readiness)
}

impl S8LayoutCloseoutCertificate {
    pub const fn sources(&self) -> &S8LayoutCloseoutSources {
        &self.sources
    }
    pub const fn classifier(&self) -> S8LayoutCloseoutClassifier {
        self.classifier
    }
    pub const fn verifier(&self) -> S8LayoutCloseoutVerifier {
        self.verifier
    }
}
