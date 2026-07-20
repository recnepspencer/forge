use super::registry_coverage::WorthQueryBoundaryAuditCoverageMechanism;
use super::source_site::WorthQueryBoundaryAuditSourceSite;
use crate::WorthQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundaryAuditSyntaxClass {
    MethodCall,
    AssociatedPathCall,
}

impl WorthQueryBoundaryAuditSyntaxClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MethodCall => "method-call",
            Self::AssociatedPathCall => "associated-path-call",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundaryAuditFindingKind {
    ProhibitedSeamUsage,
}

impl WorthQueryBoundaryAuditFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProhibitedSeamUsage => "prohibited-seam-usage",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditFinding {
    seam: WorthQueryProhibitedSeam,
    site: WorthQueryBoundaryAuditSourceSite,
    syntax_class: WorthQueryBoundaryAuditSyntaxClass,
    mechanism: WorthQueryBoundaryAuditCoverageMechanism,
}

impl WorthQueryBoundaryAuditFinding {
    pub(crate) fn prohibited_seam_usage(
        seam: WorthQueryProhibitedSeam,
        site: WorthQueryBoundaryAuditSourceSite,
        syntax_class: WorthQueryBoundaryAuditSyntaxClass,
    ) -> Self {
        Self {
            seam,
            site,
            syntax_class,
            mechanism: WorthQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved,
        }
    }

    pub fn kind(&self) -> WorthQueryBoundaryAuditFindingKind {
        WorthQueryBoundaryAuditFindingKind::ProhibitedSeamUsage
    }

    pub fn seam(&self) -> WorthQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn source_label(&self) -> &str {
        self.site.source_label()
    }

    pub fn site(&self) -> &WorthQueryBoundaryAuditSourceSite {
        &self.site
    }

    pub fn syntax_class(&self) -> WorthQueryBoundaryAuditSyntaxClass {
        self.syntax_class
    }

    pub fn mechanism(&self) -> WorthQueryBoundaryAuditCoverageMechanism {
        self.mechanism
    }

    pub fn line(&self) -> usize {
        self.site.line()
    }

    pub fn column(&self) -> usize {
        self.site.column()
    }
}
