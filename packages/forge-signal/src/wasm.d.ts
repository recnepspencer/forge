declare module "@forge/signal/wasm" {
  export const SignalApp: new () => {
    source(spec: unknown): void;
    recipe(spec: unknown): void;
    source_family(spec: unknown): void;
    recipe_family(spec: unknown): void;
    batch(ops: unknown): unknown;
    transaction_with_packed_grid_rgba(
      prefixOps: unknown,
      familyId: string,
      width: number,
      height: number,
      rgba: Uint8Array | Uint8ClampedArray | unknown,
      suffixOps: unknown,
    ): unknown;
    read(id: string): unknown;
    read_keyed(familyId: string, key: string): unknown;
    set_keyed(familyId: string, key: string, value: unknown): unknown;
    read_keyed_many(familyId: string, keys: unknown): unknown;
    read_keyed_many_packed_fields(familyId: string, keys: unknown, fields: unknown): Float32Array;
    read_keyed_grid_packed_fields(familyId: string, columns: number, rows: number, fields: unknown): unknown;
    read_keyed_rect_packed_fields(
      familyId: string,
      columns: number,
      rows: number,
      row: number,
      startColumn: number,
      width: number,
      height: number,
      fields: unknown,
    ): unknown;
    prewarm_keyed_grid(familyId: string, columns: number, rows: number): void;
    seed_keyed_grid_coords(familyId: string, columns: number, rows: number): void;
    take_debug_events(): unknown;
    set_keyed_many(familyId: string, values: unknown): unknown;
    mark_changed_with_regions(id: string, changedRegions: unknown): unknown;
    mark_keyed_changed_with_regions(familyId: string, key: string, changedRegions: unknown): unknown;
    diagnostics(): unknown;
    history(): unknown;
    specialist(): unknown;
    adapters(): unknown;
  };

  export const SignalRuntime: new () => {
    set_runtime_policy(policy: unknown): void;
    define_source(spec: unknown): void;
    define_recipe(spec: unknown): void;
    define_source_family(spec: unknown): void;
    define_recipe_family(spec: unknown): void;
    transaction(ops: unknown): unknown;
    transaction_with_packed_grid_rgba(
      prefixOps: unknown,
      familyId: string,
      width: number,
      height: number,
      rgba: Uint8Array | Uint8ClampedArray | unknown,
      suffixOps: unknown,
    ): unknown;
    read(id: string): unknown;
    read_keyed(familyId: string, key: string): unknown;
    set_keyed(familyId: string, key: string, value: unknown): unknown;
    read_keyed_many(familyId: string, keys: unknown): unknown;
    read_keyed_many_packed_fields(familyId: string, keys: unknown, fields: unknown): Float32Array;
    read_keyed_grid_packed_fields(familyId: string, columns: number, rows: number, fields: unknown): unknown;
    read_keyed_rect_packed_fields(
      familyId: string,
      columns: number,
      rows: number,
      row: number,
      startColumn: number,
      width: number,
      height: number,
      fields: unknown,
    ): unknown;
    prewarm_keyed_grid(familyId: string, columns: number, rows: number): void;
    seed_keyed_grid_coords(familyId: string, columns: number, rows: number): void;
    take_debug_events(): unknown;
    set_keyed_many(familyId: string, values: unknown): unknown;
    mark_changed_with_regions(id: string, changedRegions: unknown): unknown;
    mark_keyed_changed_with_regions(familyId: string, key: string, changedRegions: unknown): unknown;
    clear_keyed_family_cache(familyId: string): void;
    diagnostics(): unknown;
    history(): unknown;
    specialist(): unknown;
    adapters(): unknown;
  };

  export const SignalHistory: new () => {
    replay_for(id: string): unknown;
    lineage_for(id: string): unknown;
    snapshot(): unknown;
    restore_snapshot(snapshot: unknown): unknown;
    current_branch(): unknown;
    branches(): unknown;
    create_branch(name: string): unknown;
    switch_branch(branchId: bigint | number): unknown;
    replay_for_branch(branchId: bigint | number): unknown;
    branch_snapshot(branchId: bigint | number): unknown;
    branch_snapshot_id(branchId: bigint | number): bigint | number;
    branch_snapshot_envelope(branchId: bigint | number): unknown;
    restore_branch_snapshot(branchId: bigint | number, snapshot: unknown): unknown;
    restore_branch_snapshot_by_id(branchId: bigint | number, snapshotId: bigint | number): unknown;
    plan_merge_branches(sourceBranchId: bigint | number, targetBranchId: bigint | number): unknown;
    plan_merge_branches_with_proof(sourceBranchId: bigint | number, targetBranchId: bigint | number): unknown;
    merge_branches(sourceBranchId: bigint | number, targetBranchId: bigint | number): unknown;
    merge_branches_with_proof(sourceBranchId: bigint | number, targetBranchId: bigint | number): unknown;
    branch_state_proof(branchId: bigint | number): unknown;
    replay_parity_proof(expectedBranchId: bigint | number, replayedBranchId: bigint | number): unknown;
    replay_artifact_proof(expected: unknown, replayedBranchId: bigint | number): unknown;
  };

  export const SignalAdapters: new () => {
    export_definitions(): unknown;
    export_runtime_envelope(): unknown;
    runtime_proof_report(): unknown;
    replace_runtime_envelope(envelope: unknown): unknown;
  };
}

declare module "../pkg/forge_signal_wasm.js" {
  export * from "@forge/signal/wasm";
}
