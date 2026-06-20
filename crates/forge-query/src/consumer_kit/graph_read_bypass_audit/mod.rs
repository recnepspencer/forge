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
    graph_read_bypass_adoption, ForgeQueryGraphReadBypassAdoption,
    ForgeQueryGraphReadBypassAdoptionError, ForgeQueryGraphReadBypassAdoptionErrorKind,
    ForgeQueryGraphReadBypassAdoptionManifest, ForgeQueryGraphReadBypassAdoptionProof,
};
pub use audit::{graph_read_bypass_audit, ForgeQueryGraphReadBypassAudit};
pub use error::{ForgeQueryGraphReadBypassResidueError, ForgeQueryGraphReadBypassResidueErrorKind};
pub use finding::ForgeQueryGraphReadBypassFinding;
pub use registry::{
    forge_query_graph_read_bypass_registry, ForgeQueryGraphReadBypassAuthorityViolation,
    ForgeQueryGraphReadBypassClass, ForgeQueryGraphReadBypassDetection,
    ForgeQueryGraphReadBypassRegistryRow,
};
pub use report::{
    ForgeQueryGraphReadBypassCounters, ForgeQueryGraphReadBypassReport,
    ForgeQueryGraphReadBypassReportResidueCertification,
};
pub use residue::{
    ForgeQueryGraphReadBypassResidueCertification, ForgeQueryGraphReadBypassResidueManifest,
    ForgeQueryGraphReadBypassResidueRow,
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
    format!("forge-query-graph-read-bypass-{scope}:{hash:016x}")
}
