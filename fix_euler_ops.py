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
    with open(filepath, 'r') as f:
        content = f.read()

    # Find the execute method body
    # pub fn execute(...) or fn execute(...)
    pattern = r'(fn execute\(&self,\s*draft:\s*&mut MutableDraft,\s*sig:\s*&OpSignature\)\s*->\s*Result<[^>]+>,\s*KernelError>\s*\{)'
    
    # Actually wait. some might not use sig. let's just find `fn execute`
    pattern = r'(fn execute\(&self,\s*draft:\s*&mut MutableDraft.*?\)\s*(?:->\s*[^\{]+)?\{)'

    def replacer(m):
        return m.group(1) + '\n        let (arena, store) = draft.unbundle_mut();'
    
    new_content = re.sub(pattern, replacer, content)

    # replace draft.arena_mut() with arena
    new_content = new_content.replace('draft.arena_mut()', 'arena')

    # replace `, None)` with `, Some(store))`
    new_content = re.sub(r',\s*None\)', ', Some(store))', new_content)

    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)

