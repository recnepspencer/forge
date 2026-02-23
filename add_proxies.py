import re

with open('crates/forge-topo/src/topology/state.rs', 'r') as f:
    eval_content = f.read()

proxies = """
    // ── Proxy CRUD Methods (Option B Lineage Hooks) ────────────────

    pub fn insert_face(&mut self, data: crate::arena::FaceData) -> FaceId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_face(data, Some(store))
    }

    pub fn remove_face(&mut self, id: FaceId) -> Result<crate::arena::FaceData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_face(id, Some(store))
    }

    pub fn insert_half_edge(&mut self, data: crate::arena::HalfEdgeData) -> HalfEdgeId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_half_edge(data, Some(store))
    }

    pub fn insert_half_edge_pair(&mut self, data_a: crate::arena::HalfEdgeData, data_b: crate::arena::HalfEdgeData) -> (HalfEdgeId, HalfEdgeId) {
        let (arena, store) = self.unbundle_mut();
        arena.insert_half_edge_pair(data_a, data_b, Some(store))
    }

    pub fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<crate::arena::HalfEdgeData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_half_edge(id, Some(store))
    }

    pub fn insert_vertex(&mut self, data: crate::arena::VertexData) -> VertexId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_vertex(data, Some(store))
    }

    pub fn remove_vertex(&mut self, id: VertexId) -> Result<crate::arena::VertexData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_vertex(id, Some(store))
    }

    pub fn insert_loop(&mut self, data: crate::arena::LoopData) -> LoopId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_loop(data, Some(store))
    }

    pub fn remove_loop(&mut self, id: LoopId) -> Result<crate::arena::LoopData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_loop(id, Some(store))
    }

    pub fn insert_shell(&mut self, data: crate::arena::ShellData) -> ShellId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_shell(data, Some(store))
    }

    pub fn remove_shell(&mut self, id: ShellId) -> Result<crate::arena::ShellData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_shell(id, Some(store))
    }

    pub fn insert_edge(&mut self, data: crate::arena::EdgeData) -> EdgeId {
        let (arena, store) = self.unbundle_mut();
        arena.insert_edge(data, Some(store))
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> Result<crate::arena::EdgeData, KernelError> {
        let (arena, store) = self.unbundle_mut();
        arena.remove_edge(id, Some(store))
    }
}
"""

# Replace the closing brace of `impl MutableDraft { ... }` with these proxies.
# We look for `pub(crate) fn compute_topology_hash(&self) -> u128 { ... }` and replace the following `}`.
pattern = r'(pub\(crate\) fn compute_topology_hash\(&self\) -> u128 \{\s*compute_arena_topology_hash\(&self\.arena\)\s*\})\s*\}'
eval_content = re.sub(pattern, r'\1' + proxies, eval_content)

# Add handles import to state.rs if missing
if 'FaceId' not in eval_content:
    # Actually FaceId is already imported? Let's check imports.
    # The file has: use crate::handles::{FaceId, VertexId}; maybe not all handles.
    pass

with open('crates/forge-topo/src/topology/state.rs', 'w') as f:
    f.write(eval_content)

