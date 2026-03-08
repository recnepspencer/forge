# Domain Architecture & Structural Standards

## Purpose
This document defines the structural rules for introducing new domains and subsystems into the codebase. 
These standards exist to preserve:

* Architectural clarity
* Long-term maintainability
* Low coupling
* High cohesion
* Domain alignment

> **Structural discipline is not optional.** It is a prerequisite for building a system that can scale in size, contributors, and lifespan.

---

## 1. Organize by Components and Subsystems

### Principle
The top-level structure of the system must be organized around components, not file types. 
A component is a cohesive subsystem responsible for a clearly defined functional capability. It should be conceptually understandable and evolvable in isolation.

### Requirements
* The root directory must be divided by domain components.
* Each component folder must contain all code necessary for that subsystem.
* Cross-cutting layers (e.g., global `models/`, `controllers/`, `helpers/`) are prohibited unless they represent true application-wide infrastructure.

### Rationale
Organizing by file type creates horizontal coupling. Organizing by component preserves vertical slices aligned with business capabilities.

---

## 2. Enforce Separation of Concerns (N-Tier Architecture)

### Principle
Within each component, responsibilities must be separated into distinct architectural layers:

* **Presentation** — Interfaces, UI adapters, API boundaries, external input/output
* **Logic** — Domain rules, orchestration, use cases, application services
* **Data** — Persistence models, storage abstractions, schema definitions

### Required Structure
Each component should follow a consistent internal structure:

    component/
      presentation/
      logic/
      data/
      facade.rs

### Constraints
* Presentation must not contain business logic.
* Logic must not contain persistence details.
* Data must not contain orchestration behavior.
* Cross-layer imports should flow inward, not cyclically.
* Maintain vertical slices. Still create subdirectories for organization within each layer as appropriate.

---

## 3. Maximize Cohesion (Single Responsibility)

### Principle
Every module, file, and folder must represent a single, well-defined concept. 
A file or folder that performs multiple unrelated responsibilities must be split.

### Prohibited Patterns
* “misc” or “other” folders
* Generic “utils” folders that mix unrelated functionality
* Files named with conjunctions (e.g., `AuthAndRouting`)
* Multi-purpose classes that manage unrelated behaviors

### Enforcement Standard
If a name cannot be expressed as a singular, precise noun, the construct is likely not cohesive.

---

## 4. Maintain Complete Domain Alignment

### Principle
The structure of the codebase must mirror the real-world problem domain. 
Folder and file names must represent business concepts, not technical mechanics.

| Correct Examples | Incorrect Examples |
| :--- | :--- |
| Bodies | Helpers |
| Regions | Managers |
| Transactions | Stuff |
| Inventory | Common |
| Certification | StringManipulators |
| BooleanOperations | Test |

> **Goal:** If a subject matter expert unfamiliar with the implementation examines the directory structure, they should recognize domain concepts.

---

## 5. Restrict Scope via Packages and Namespaces

### Principle
Scope must reflect utility. Constructs should only be visible at the level where they are required.

### Requirements
* Treat folders as architectural boundaries.
* Default to internal visibility.
* Only expose what external components must use.
* Do not elevate modules to root scope unless the entire application depends on them.

---

## 6. Use Intent-Revealing Names

### Principle
Names are a navigational system. Imprecise names are structural debt.

### Standards
* Classes and files must be named as singular, descriptive nouns.
* Names must reflect domain meaning, not implementation detail.
* Avoid abbreviations unless they are industry-standard and unambiguous.
* Avoid conjunctions in names; they signal multiple responsibilities.
* Prefer accessible, intention-revealing names over compressed or overly technical names.
* If a slightly longer name is materially easier for a smart generalist to understand on first read, prefer the longer name.

### API Naming Guidance
Public API names should optimize for first-read comprehension, not technical elegance.

Examples:
* Prefer `depends_on_aspects` over a shorter but less obvious alternative.
* Prefer `condition` over jargon that hides what the field means.
* Prefer names that teach the model directly instead of requiring prior framework knowledge.

---

## 7. Minimize Coupling Through Façades

### Principle
Components must communicate through controlled entry points. 
Direct access into deep internal modules of another component is prohibited.

### Required Pattern
Each component must expose a single public façade file at its root:

    component/
      facade.rs

* External components must depend **only** on this façade.
* Internal complexity must remain hidden.

### Rationale
Façades prevent cross-component entanglement and preserve replaceability of subsystems.
