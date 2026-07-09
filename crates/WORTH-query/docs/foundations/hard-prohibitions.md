# WORTH Query Hard Prohibitions

This reference is generated from the hard prohibition registry. Do not edit it without updating the registry-owned projection test.

| Seam | Forbidden symbol | Enforcement | Replacement lane | Rationale |
| --- | --- | --- | --- | --- |
| workspace.direct-write | `WorthQueryWorkspace::write` | sealed-by-visibility | WorthQueryWorkspace::submissions | direct workspace writes bypass the explicit submission/admission lane |
| workspace.direct-batch | `WorthQueryWorkspace::batch` | sealed-by-visibility | WorthQueryWorkspace::submissions | direct workspace batches bypass the explicit submission/admission lane |
| workspace.existing-truth.bind-entity | `WorthQueryWorkspace::bind_existing_entity` | sealed-by-visibility | typed existing-truth binding artifact plus graph composition or probe intent lane | workspace binding helpers hide the typed binding artifact boundary |
| workspace.existing-truth.bind-relation | `WorthQueryWorkspace::bind_existing_relation` | sealed-by-visibility | typed existing-truth binding artifact plus graph composition or probe intent lane | workspace binding helpers hide the typed binding artifact boundary |
| workspace.existing-truth.probe | `WorthQueryWorkspace::probe_existing` | sealed-by-visibility | typed existing-truth binding artifact plus graph composition or probe intent lane | existing-truth probes must pass through intent admission before execution |
| workspace.existing-truth.update | `WorthQueryWorkspace::update_existing` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | existing-truth mutation must not be caller-assembled from a direct binding |
| workspace.existing-truth.assert | `WorthQueryWorkspace::assert_existing` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | existing-truth assertion must stay inside the admitted runtime lane |
| workspace.existing-truth.verify | `WorthQueryWorkspace::verify_existing` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | existing-truth verification must stay inside the admitted runtime lane |
| workspace.existing-truth.update-verified | `WorthQueryWorkspace::update_existing_verified` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | verified existing-truth mutation must be planned by the owning lane |
| workspace.existing-truth.delete | `WorthQueryWorkspace::delete_existing` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | existing-truth deletion must not be caller-assembled from a direct binding |
| workspace.existing-truth.delete-with | `WorthQueryWorkspace::delete_existing_with` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | existing-truth deletion must not be caller-assembled from a direct binding |
| workspace.existing-truth.delete-verified | `WorthQueryWorkspace::delete_existing_verified` | sealed-by-visibility | graph composition or admitted existing-truth mutation lane | verified existing-truth deletion must be planned by the owning lane |
