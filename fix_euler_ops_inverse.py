import os
import re

files_to_fix = [
    'crates/forge-topo/src/topology/operations/euler/make_edge_face.rs',
    'crates/forge-topo/src/topology/operations/euler/make_vertex_face.rs',
    'crates/forge-topo/src/topology/operations/euler/split_edge.rs',
    'crates/forge-topo/src/topology/operations/euler/kill_edge_vertex.rs',
    'crates/forge-topo/src/topology/operations/algorithms/bridge_edge.rs'
]

for filepath in files_to_fix:
    if not os.path.exists(filepath):
        continue
    with open(filepath, 'r') as f:
        content = f.read()

    # remove the unbundle_mut line
    content = re.sub(r'^\s*let \(arena, store\) = draft\.unbundle_mut\(\);\n', '', content, flags=re.MULTILINE)

    # replace arena -> draft.arena_mut() ? Wait, some places might legitimately use arena.
    # Actually, yes, I unconditionally replaced draft.arena_mut() with arena.
    content = content.replace('let arena = arena;', '')
    
    # We should look where insert_face is called. 
    # Just replacing arena.insert_ with draft.insert_ 
    content = re.sub(r'\barena\.(insert_(face|half_edge|half_edge_pair|vertex|loop|shell|edge))', r'draft.\1', content)
    content = re.sub(r'\barena\.(remove_(face|half_edge|vertex|loop|shell|edge))', r'draft.\1', content)

    # And for get_*_mut, it should be draft.arena_mut().get_*_mut
    content = re.sub(r'\barena\.(get_\w+_mut)', r'draft.arena_mut().\1', content)
    
    # And for half_edge_count, etc
    content = re.sub(r'\barena\.(half_edge_count)', r'draft.arena().\1', content)

    # replace , Some(store)) with "" because we want `draft.insert_face(data)` directly. 
    # Wait, in the proxy we can just NOT take `Some(store)` because `MutableDraft` already owns it!
    # So draft.insert_face(data) takes ONE argument.
    content = re.sub(r',\s*Some\(\s*store\s*\)\s*\)', ')', content)
    
    # The previous fix_inserts.py might have left `, None)` for some removes or inserts. 
    # Wait, the operators had `, None)` earlier, which we replaced with `, Some(store))`.
    # So if they had `, None)`, we should remove it because the proxy takes just `data`.
    # Let's remove `, None)` from `draft.insert_` calls.
    content = re.sub(r'\b(draft\.insert_\w+\([^\)]+?)(,\s*None)\)', r'\1)', content)
    content = re.sub(r'\b(draft\.remove_\w+\([^\)]+?)(,\s*None)\)', r'\1)', content)

    # Also fix any remaining syntax errors `, None)`
    content = content.replace(', None)', ')')

    with open(filepath, 'w') as f:
        f.write(content)

