#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAdmittedReconstructedLoop {
    reconstructed_loop_identity: String,
    loop_candidate_identity: String,
    source_loop_identity: String,
    source_face_identity: String,
    local_frame_identity: String,
    precision_basis_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBornLoop {
    born_loop_identity: String,
    loop_candidate_identity: String,
    source_loop_identities: Vec<String>,
    contributing_chain_identities: Vec<String>,
    local_frame_identity: String,
    precision_basis_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
}

impl PlanarBooleanAdmittedReconstructedLoop {
    pub(crate) fn new(
        reconstructed_loop_identity: String,
        loop_candidate_identity: String,
        source_loop_identity: String,
        source_face_identity: String,
        local_frame_identity: String,
        precision_basis_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
    ) -> Self {
        Self {
            reconstructed_loop_identity,
            loop_candidate_identity,
            source_loop_identity,
            source_face_identity,
            local_frame_identity,
            precision_basis_identity,
            fragment_identities,
            split_vertex_identities,
        }
    }

    pub fn reconstructed_loop_identity(&self) -> &str {
        &self.reconstructed_loop_identity
    }

    pub fn loop_candidate_identity(&self) -> &str {
        &self.loop_candidate_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }
}

impl PlanarBooleanBornLoop {
    pub(crate) fn new(
        born_loop_identity: String,
        loop_candidate_identity: String,
        source_loop_identities: Vec<String>,
        contributing_chain_identities: Vec<String>,
        local_frame_identity: String,
        precision_basis_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
    ) -> Self {
        Self {
            born_loop_identity,
            loop_candidate_identity,
            source_loop_identities,
            contributing_chain_identities,
            local_frame_identity,
            precision_basis_identity,
            fragment_identities,
            split_vertex_identities,
        }
    }

    pub fn born_loop_identity(&self) -> &str {
        &self.born_loop_identity
    }

    pub fn loop_candidate_identity(&self) -> &str {
        &self.loop_candidate_identity
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn contributing_chain_identities(&self) -> &[String] {
        &self.contributing_chain_identities
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }
}
