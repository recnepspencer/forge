import re

files_to_fix = [
    'crates/forge-topo/src/topology/operations/euler/split_edge.rs',
    'crates/forge-topo/src/topology/operations/euler/kill_edge_vertex.rs',
    'crates/forge-topo/src/topology/operations/algorithms/bridge_edge.rs',
    'crates/forge-topo/src/topology/operations/euler/join_faces.rs'
]

# The remaining errors are primarily due to:
# let arena = draft.arena_mut();
# followed by draft.insert_face(...) or draft.arena().get_...
# Since we no longer use `arena.` but instead `draft.`, we should remove `let arena = draft.arena_mut();` completely and change `arena.` to `draft.` for mutations too if there are any.
# Wait, for mutations like `set_next`, we need `draft.arena_mut().set_next(...)`.
# Let's remove `let arena = draft.arena_mut();` and just replace `arena.get_half_edge_mut` with `draft.arena_mut().get_half_edge_mut` and `arena.remove_...` with `draft.remove_...`

for filepath in files_to_fix:
    with open(filepath, 'r') as f:
        content = f.read()

    # remove leftover binding
    content = re.sub(r'\s*let arena = draft\.arena_mut\(\);\n', '\n', content)
    content = re.sub(r'\s*let arena = arena;\n', '\n', content)

    # replace residual `arena.` calls
    content = re.sub(r'\barena\.(get_\w+_mut\([^)]+\)\?)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(remove_(face|half_edge|vertex|loop|shell|edge))', r'draft.\1', content)
    content = re.sub(r'\barena\.(bump_face_version)', r'draft.arena_mut().\1', content)
    content = re.sub(r'\barena\.(insert_(face|half_edge|half_edge_pair|vertex|loop|shell|edge))', r'draft.\1', content)

    # And some might still use , None for remove. Since the proxy doesn't take None:
    content = re.sub(r'\b(draft\.remove_\w+\([^\)]+?)(,\s*None)\)', r'\1)', content)

    with open(filepath, 'w') as f:
        f.write(content)

