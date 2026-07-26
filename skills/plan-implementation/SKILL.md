---
name: plan-implementation
description: Create an implementation plan for a specified WORTH milestone phase or implementation slice. Use when the user wants a concrete, architecture-grounded plan before coding begins.
---

# Plan Implementation

Create an implementation plan for the requested phase or slice.

Do not implement the plan or edit files during this turn.

As you plan:

- Follow the repository's engineering mentality and its architectural,
  performance, composition, domain-structure, testing, and DX laws.
- Review the relevant context, including the governing specification, current
  implementation, relevant public APIs, tests, upstream authorities, and
  downstream consumers.
- Identify the adversarial constraint the work must survive.
- Plan the appropriate causal scope. Include required foundations,
  integrations, cutovers, and proof work even when they cross files, crates, or
  phases.
- Plan the destination directory and module skeleton explicitly. State what
  each proposed file or module owns.
- Plan the intended DX as an actual code-block target whenever the work affects
  a caller-facing surface.
- Make implicit requirements, authority boundaries, failure behavior,
  lifecycle obligations, and performance expectations explicit.
- Identify existing paths that must be cut over, removed, or made insufficient.
- Include the phase-relevant tests, structural counters, compiler boundaries,
  and verification needed to prove the result.
- Build the complete plan inline in the chat. Do not create a separate plan
  file unless the user asks for one.

Write one coherent implementation path rather than a loose collection of
possibilities.

Order the plan by dependency and authority. For each step, explain:

- what the step requires
- what must change
- why that change belongs there
- what later work depends on it
- how the step will be proven complete

Do not produce shallow bullet points that leave implementation to rediscover
the architecture. Make the plan explicit enough for implementation to follow
directly while remaining proportional to the work.
