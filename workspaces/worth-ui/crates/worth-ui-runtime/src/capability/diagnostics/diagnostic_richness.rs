/// Diagnostic materialization richness for registration reports.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityDiagnosticRichness {
    Minimal,
    #[default]
    Rich,
}
