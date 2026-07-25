use std::collections::{BTreeMap, BTreeSet};

use super::{require_text, PhysicalWorkRunProvenanceDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkFeatureNodeEvidence {
    package: Box<str>,
    features: Box<[Box<str>]>,
    dependencies: Box<[Box<str>]>,
}

impl PhysicalWorkFeatureNodeEvidence {
    pub fn new(
        package: impl Into<Box<str>>,
        features: impl IntoIterator<Item = impl Into<Box<str>>>,
        dependencies: impl IntoIterator<Item = impl Into<Box<str>>>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let package = package.into();
        require_text(&package, PhysicalWorkRunProvenanceDenial::EmptyFeatureNode)?;
        let features = canonical_text(
            features,
            PhysicalWorkRunProvenanceDenial::EmptyFeatureName,
            PhysicalWorkRunProvenanceDenial::DuplicateFeatureName,
        )?;
        let dependencies = canonical_text(
            dependencies,
            PhysicalWorkRunProvenanceDenial::EmptyDependencyNode,
            PhysicalWorkRunProvenanceDenial::DuplicateDependencyNode,
        )?;
        Ok(Self {
            package,
            features,
            dependencies,
        })
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub const fn features(&self) -> &[Box<str>] {
        &self.features
    }

    pub const fn dependencies(&self) -> &[Box<str>] {
        &self.dependencies
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkFeatureGraphEvidence {
    roots: Box<[Box<str>]>,
    nodes: Box<[PhysicalWorkFeatureNodeEvidence]>,
}

impl PhysicalWorkFeatureGraphEvidence {
    pub fn new(
        roots: impl IntoIterator<Item = impl Into<Box<str>>>,
        nodes: impl IntoIterator<Item = PhysicalWorkFeatureNodeEvidence>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let roots = canonical_text(
            roots,
            PhysicalWorkRunProvenanceDenial::EmptyFeatureNode,
            PhysicalWorkRunProvenanceDenial::DuplicateFeatureRoot,
        )?;
        let mut nodes = nodes.into_iter().collect::<Vec<_>>();
        if nodes.is_empty() {
            return Err(PhysicalWorkRunProvenanceDenial::EmptyFeatureGraph);
        }
        nodes.sort_by(|left, right| left.package().cmp(right.package()));
        if nodes
            .windows(2)
            .any(|pair| pair[0].package() == pair[1].package())
        {
            return Err(PhysicalWorkRunProvenanceDenial::DuplicateFeatureNode);
        }
        validate_graph(&roots, &nodes)?;
        Ok(Self {
            roots,
            nodes: nodes.into_boxed_slice(),
        })
    }

    pub const fn roots(&self) -> &[Box<str>] {
        &self.roots
    }

    pub const fn nodes(&self) -> &[PhysicalWorkFeatureNodeEvidence] {
        &self.nodes
    }
}

fn canonical_text(
    values: impl IntoIterator<Item = impl Into<Box<str>>>,
    empty: PhysicalWorkRunProvenanceDenial,
    duplicate: PhysicalWorkRunProvenanceDenial,
) -> Result<Box<[Box<str>]>, PhysicalWorkRunProvenanceDenial> {
    let mut values = values
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Box<str>>>();
    for value in &values {
        require_text(value, empty)?;
    }
    values.sort();
    if values.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(duplicate);
    }
    Ok(values.into_boxed_slice())
}

fn validate_graph(
    roots: &[Box<str>],
    nodes: &[PhysicalWorkFeatureNodeEvidence],
) -> Result<(), PhysicalWorkRunProvenanceDenial> {
    let packages = nodes
        .iter()
        .map(|node| (node.package(), node))
        .collect::<BTreeMap<_, _>>();
    if roots
        .iter()
        .any(|root| !packages.contains_key(root.as_ref()))
    {
        return Err(PhysicalWorkRunProvenanceDenial::MissingFeatureRoot);
    }
    let dependencies = nodes
        .iter()
        .flat_map(PhysicalWorkFeatureNodeEvidence::dependencies)
        .map(Box::as_ref)
        .collect::<BTreeSet<_>>();
    if dependencies
        .iter()
        .any(|dependency| !packages.contains_key(dependency))
    {
        return Err(PhysicalWorkRunProvenanceDenial::MissingDependencyNode);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::PhysicalWorkRunProvenanceDenial;
    use super::{PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence};

    #[test]
    fn graph_rejects_a_dependency_outside_the_bound_closure() {
        let node =
            PhysicalWorkFeatureNodeEvidence::new("worth-store", ["certification"], ["missing"])
                .unwrap();
        assert_eq!(
            PhysicalWorkFeatureGraphEvidence::new(["worth-store"], [node]),
            Err(PhysicalWorkRunProvenanceDenial::MissingDependencyNode)
        );
    }

    #[test]
    fn graph_rejects_duplicate_feature_activation() {
        assert_eq!(
            PhysicalWorkFeatureNodeEvidence::new(
                "worth-store",
                ["certification", "certification"],
                Vec::<String>::new()
            ),
            Err(PhysicalWorkRunProvenanceDenial::DuplicateFeatureName)
        );
    }
}
