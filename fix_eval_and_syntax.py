import re

# 1. Fix eval.rs Loop tracking: Loops do not have lineage.
with open('crates/forge-topo/src/arena/eval.rs', 'r') as f:
    eval_content = f.read()

# For insert_loop:
pattern_insert_loop = r'''let id = LoopId::new\(index, gen\);\s*if let Some\(store\) = ls\.as_deref_mut\(\) \{\s*if let Some\(lin\) = self\.loop_slots\[index as usize\]\.data\.as_ref\(\)\.unwrap\(\)\.lineage\(\)\.cloned\(\) \{\s*store\.record_creation\(EntityRef::new\(EntityKind::Loop, id\.index\(\)\), lin\);\s*\}\s*\}\s*id'''
eval_content = re.sub(pattern_insert_loop, r'LoopId::new(index, gen)', eval_content)

# For remove_loop:
pattern_remove_loop = r'''if let Some\(store\) = ls\.as_deref_mut\(\) \{\s*let _ = store\.record_deletion\(EntityRef::new\(EntityKind::Loop, id\.index\(\)\)\);\s*\}\s*Ok\(data\)'''
eval_content = re.sub(pattern_remove_loop, r'Ok(data)', eval_content)

with open('crates/forge-topo/src/arena/eval.rs', 'w') as f:
    f.write(eval_content)


# 2. Fix the `, None)` syntax errors in euler operator files
def fix_syntax(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Replace anything like `, None);` or `\n        , None);` that might have replaced `);` incorrectly.
    # The previous script did `content[:curr] + ', None' + content[curr:]` where `curr` was `)`.
    # But if the line ended with a trailing comma before the `)`, we'd get `, , None)`.
    # Wait, the error is `expected expression, found `,``. This means `foo(arg1, , None)`.
    content = re.sub(r',\s*,\s*None\)', ', None)', content)
    
    with open(filepath, 'w') as f:
        f.write(content)

import os
for root, _, files in os.walk('crates/forge-topo/src'):
    for file in files:
        if file.endswith('.rs'):
            fix_syntax(os.path.join(root, file))

