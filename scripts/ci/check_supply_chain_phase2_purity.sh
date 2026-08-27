#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
pure_root="$workspace_root/crates/worth-relational/tests/relational_certification"

if [[ ! -d "$pure_root" ]]; then
  echo "[supply-chain-phase2-purity] missing pure certification root: $pure_root" >&2
  exit 1
fi

forbidden='worth_relational|RelationalRuntime|relational::facade|crate::facade|CompiledSupplyChainProgram|SupplyChainSemanticHandles|CertifiedSupplyChainBaseline|canonical_digest|query::|history::|branch_head|\bmvcc\b|crate::[^[:space:]]*(visibility|transaction|snapshot)|worth_relational::'
adapter_files='(^|[/\\])production_(world|failures)\.rs$|(^|[/\\])world[/\\]supply_chain[/\\](mod|program|program_schema|schema_vocabulary|compiler|production_world|baseline_audit|observation|handles)\.rs$'

mapfile -t pure_files < <(rg --files "$pure_root" --glob '*.rs' | rg -v "$adapter_files" || true)
if ((${#pure_files[@]} > 0)) && rg -n -i -e "$forbidden" "${pure_files[@]}"; then
  echo "[supply-chain-phase2-purity] forbidden production dependency or authority residue found" >&2
  exit 1
fi

if rg --files "$pure_root" | rg -n '(^|[/\\])runtime_driver\.rs$'; then
  echo "[supply-chain-phase2_purity] runtime driver is deferred and must not enter the Phase 2 subtree" >&2
  exit 1
fi

echo "[supply-chain-phase2-purity] PASS: Phase 2 subtree is production-independent"
