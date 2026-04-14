#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalizationWarning {
    DuplicateProjectionCollapsed { aspect: String, field: String },
    DuplicateTraversalCollapsed { relation: String, depth: u8 },
    DuplicateResultFieldCollapsed { delivered_name: String },
    NonIdentityBindingMetadataIgnored { key: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NormalizationEvent {
    ProjectionRetained {
        aspect: String,
        field: String,
    },
    ProjectionCollapsedDuplicate {
        aspect: String,
        field: String,
    },
    TraversalRetained {
        relation: String,
        depth: u8,
    },
    TraversalCollapsedDuplicate {
        relation: String,
        depth: u8,
    },
    ResultFieldRetained {
        source_aspect: String,
        source_field: String,
        delivered_name: String,
    },
    ResultFieldCollapsedDuplicate {
        delivered_name: String,
    },
    IdentityBindingRetained {
        slot: String,
    },
    IdentityBindingCollapsedDuplicate {
        slot: String,
    },
    NonIdentityBindingIgnored {
        key: String,
    },
    CompatibilityEstablished,
    IdentityFrozen {
        query_digest: String,
        result_shape_digest: String,
    },
}
