import os
import re

files_to_fix = [
    'crates/forge-topo/src/topology/operations/euler/make_edge_face.rs',
    'crates/forge-topo/src/topology/operations/euler/make_vertex_face.rs',
    'crates/forge-topo/src/topology/operations/euler/split_edge.rs',
    'crates/forge-topo/src/topology/operations/euler/kill_edge_vertex.rs',
    'crates/forge-topo/src/topology/operations/algorithms/bridge_edge.rs',
    'crates/forge-topo/src/topology/operations/euler/join_faces.rs'
]

for filepath in files_to_fix:
    if not os.path.exists(filepath):
        continue
    with open(filepath, 'r') as f:
        content = f.read()

    # The issue: we removed `let arena = arena;` and `let (arena, store) = draft.unbundle_mut();`
    # and replaced `arena.insert_face` with `draft.insert_face`.
    # But there are many calls to `arena.get_face`, `arena.get_half_edge`, `arena.bump_face_version`, etc.
    # Because `arena` was the local variable holding `&mut TopologyArena` or `&TopologyArena`.
    # So wherever it says `arena.get_...`, it should say `draft.arena().get_...` for immutable,
    # and `draft.arena_mut().get_..._mut` for mutable, or `draft.arena_mut().bump_face_version`.
    
    # We already did some of this, let's fix the remaining ones.
    
    # Immutable gets:
    content = re.sub(r'\barena\.(get_face\([^)]+\)\?)', r'draft.arena().\1', content)
    content = re.sub(r'\barena\.(get_half_edge\([^)]+\)\?)', r'draft.arena().\1', content)
    content = re.sub(r'\barena\.(get_vertex\([^)]+\)\?)', r'draft.arena().\1', content)
    content = re.sub(r'\barena\.(get_loop\([^)]+\)\?)', r'draft.arena().\1', content)
    content = re.sub(r'\barena\.(get_shell\([^)]+\)\?)', r'draft.arena().\1', content)
    content = re.sub(r'\barena\.(get_edge\([^)]+\)\?)', r'draft.arena().\1', content)
    
    # Mutable gets (we already did this, but let's be sure):
    content = re.sub(r'\barena\.(get_face_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(get_half_edge_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(get_vertex_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(get_loop_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(get_shell_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(get_edge_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    
    # bump_face_version:
    content = re.sub(r'\barena\.(bump_face_version\([^)]+\)\?)', r'draft.arena_mut().\1', content)

    with open(filepath, 'w') as f:
        f.write(content)

