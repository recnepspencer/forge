mod adoption;
mod audit;
mod detection;
mod error;
mod evidence;
mod finding;
mod registry;
mod report;
mod residue;

#[cfg(test)]
mod tests;

pub use adoption::{
    graph_read_bypass_adoption, WorthQueryGraphReadBypassAdoption,
    WorthQueryGraphReadBypassAdoptionError, WorthQueryGraphReadBypassAdoptionErrorKind,
    WorthQueryGraphReadBypassAdoptionManifest, WorthQueryGraphReadBypassAdoptionProof,
};
pub use audit::{graph_read_bypass_audit, WorthQueryGraphReadBypassAudit};
pub use error::{WorthQueryGraphReadBypassResidueError, WorthQueryGraphReadBypassResidueErrorKind};
pub use finding::WorthQueryGraphReadBypassFinding;
pub use registry::{
    worth_query_graph_read_bypass_registry, WorthQueryGraphReadBypassAuthorityViolation,
    WorthQueryGraphReadBypassClass, WorthQueryGraphReadBypassDetection,
    WorthQueryGraphReadBypassRegistryRow,
};
pub use report::{
    WorthQueryGraphReadBypassCounters, WorthQueryGraphReadBypassReport,
    WorthQueryGraphReadBypassReportResidueCertification,
};
pub use residue::{
    WorthQueryGraphReadBypassResidueCertification, WorthQueryGraphReadBypassResidueManifest,
    WorthQueryGraphReadBypassResidueRow,
};

pub(super) fn graph_read_bypass_digest<'a>(
    scope: &str,
    parts: impl IntoIterator<Item = &'a str>,
) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in scope.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for part in parts {
        hash ^= u64::from(b'|');
        hash = hash.wrapping_mul(0x100000001b3);
        for byte in part.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("worth-query-graph-read-bypass-{scope}:{hash:016x}")
}
