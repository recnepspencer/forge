use super::registry_coverage::ForgeQueryBoundaryAuditCoverageMechanism;
use super::source_site::ForgeQueryBoundaryAuditSourceSite;
use crate::ForgeQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBoundaryAuditSyntaxClass {
    MethodCall,
    AssociatedPathCall,
}

impl ForgeQueryBoundaryAuditSyntaxClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MethodCall => "method-call",
            Self::AssociatedPathCall => "associated-path-call",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBoundaryAuditFindingKind {
    ProhibitedSeamUsage,
}

impl ForgeQueryBoundaryAuditFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProhibitedSeamUsage => "prohibited-seam-usage",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditFinding {
    seam: ForgeQueryProhibitedSeam,
    site: ForgeQueryBoundaryAuditSourceSite,
    syntax_class: ForgeQueryBoundaryAuditSyntaxClass,
    mechanism: ForgeQueryBoundaryAuditCoverageMechanism,
}

impl ForgeQueryBoundaryAuditFinding {
    pub(crate) fn prohibited_seam_usage(
        seam: ForgeQueryProhibitedSeam,
        site: ForgeQueryBoundaryAuditSourceSite,
        syntax_class: ForgeQueryBoundaryAuditSyntaxClass,
    ) -> Self {
        Self {
            seam,
            site,
            syntax_class,
            mechanism: ForgeQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved,
        }
    }

    pub fn kind(&self) -> ForgeQueryBoundaryAuditFindingKind {
        ForgeQueryBoundaryAuditFindingKind::ProhibitedSeamUsage
    }

    pub fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn source_label(&self) -> &str {
        self.site.source_label()
    }

    pub fn site(&self) -> &ForgeQueryBoundaryAuditSourceSite {
        &self.site
    }

    pub fn syntax_class(&self) -> ForgeQueryBoundaryAuditSyntaxClass {
        self.syntax_class
    }

    pub fn mechanism(&self) -> ForgeQueryBoundaryAuditCoverageMechanism {
        self.mechanism
    }

    pub fn line(&self) -> usize {
        self.site.line()
    }

    pub fn column(&self) -> usize {
        self.site.column()
    }
}
