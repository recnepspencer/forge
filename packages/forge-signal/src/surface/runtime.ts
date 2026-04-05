import {
  compositeKeyedId,
  decodeSignalValue,
  encodeSignalValue,
  normalizeRecipeFamilySpec,
  normalizeRecipeSpec,
  normalizeSourceFamilySpec,
  normalizeSourceSpec,
  normalizeTransactionOp,
} from "../internal/codec.ts";
import {
  RecipeFamilyHandle,
  RecipeHandle,
  SourceFamilyHandle,
  SourceHandle,
} from "./handles.ts";
import { SignalDiagnostics } from "./diagnostics.ts";
import { SignalHistory } from "./history.ts";
import { SignalSpecialist } from "./specialist.ts";
import { SignalAdapters } from "./adapters.ts";

export class SignalRuntime {
  inner: any;
  _watchers: Map<string, { read: () => unknown; listeners: Set<(value: unknown) => void>; version: number | null }>;
  _globalListeners: Set<() => void>;

  constructor(inner: any) {
    this.inner = inner;
    this._watchers = new Map();
    this._globalListeners = new Set();
  }

  setRuntimePolicy(policySpec: unknown) {
    this.inner.set_runtime_policy(policySpec);
    return this;
  }

  defineSource<T = unknown>(spec: any): SourceHandle<T> {
    const normalized = normalizeSourceSpec(spec);
    this.inner.define_source(normalized);
    return new SourceHandle<T>(this, normalized.id);
  }

  defineRecipe<T = unknown>(spec: any): RecipeHandle<T> {
    const normalized = normalizeRecipeSpec(spec);
    this.inner.define_recipe(normalized);
    return new RecipeHandle<T>(this, normalized.id);
  }

  defineSourceFamily<T = unknown>(spec: any): SourceFamilyHandle<T> {
    const normalized = normalizeSourceFamilySpec(spec);
    this.inner.define_source_family(normalized);
    return new SourceFamilyHandle<T>(this, normalized.familyId);
  }

  defineRecipeFamily<T = unknown>(spec: any): RecipeFamilyHandle<T> {
    const normalized = normalizeRecipeFamilySpec(spec);
    this.inner.define_recipe_family(normalized);
    return new RecipeFamilyHandle<T>(this, normalized.familyId);
  }

  transaction(ops: any[]) {
    const normalized = ops.map(normalizeTransactionOp);
    const packedIndex = normalized.findIndex((op: any) => op.kind === "setPackedGridRgba");
    if (packedIndex === -1) {
      const result = this.inner.transaction(normalized);
      this._drainDebugEvents();
      this._flushWatchers();
      return result;
    }

    const packed: any = normalized[packedIndex];
    const before = normalized.slice(0, packedIndex);
    const after = normalized.slice(packedIndex + 1);
    const laterPacked = after.find((op: any) => op.kind === "setPackedGridRgba");
    if (laterPacked) {
      throw new Error("Only one packed grid RGBA op is supported per transaction.");
    }

    const result = this.inner.transaction_with_packed_grid_rgba(
      before,
      packed.familyId,
      packed.width,
      packed.height,
      packed.rgba,
      after,
    );
    this._drainDebugEvents();
    this._flushWatchers();
    return result;
  }

  read<T = unknown>(id: string): T {
    return this._read(id);
  }

  readMany<T = unknown>(ids: string[]): T[] {
    if (typeof this.inner.read_many === "function") {
      try {
        return this.inner.read_many(ids).map(decodeSignalValue) as T[];
      } catch (error) {
        if (!isMissingWasmExportError(error, "read_many")) {
          throw error;
        }
        debugPackedSurface(`read-many count=${ids.length} falling back after missing export`);
      }
    }
    debugPackedSurface(`read-many count=${ids.length} using scalar fallback`);
    return ids.map((id) => this._read(id)) as T[];
  }

  handle<T = unknown>(id: string): SourceHandle<T> | RecipeHandle<T> {
    return new SourceHandle<T>(this, id);
  }

  readKeyed<T = unknown>(familyId: string, key: string): T {
    return this._readKeyed(familyId, key);
  }

  setKeyed<T = unknown>(familyId: string, key: string, value: T) {
    return this._setKeyed(familyId, key, value);
  }

  markChangedWithRegions(id: string, changedRegions: Array<{ partition: string; detail?: string | null }>) {
    const result = this.inner.mark_changed_with_regions(id, changedRegions);
    this._flushWatchers();
    return result;
  }

  markKeyedChangedWithRegions(
    familyId: string,
    key: string,
    changedRegions: Array<{ partition: string; detail?: string | null }>,
  ) {
    const result = this.inner.mark_keyed_changed_with_regions(familyId, key, changedRegions);
    this._flushWatchers();
    return result;
  }

  readKeyedMany<T = unknown>(familyId: string, keys: string[]): T[] {
    return this._readKeyedMany(familyId, keys);
  }

  readKeyedManyPackedFields(
    familyId: string,
    keys: string[],
    fields: string[],
  ): Float32Array {
    if (typeof this.inner.read_keyed_many_packed_fields === "function") {
      return Float32Array.from(this.inner.read_keyed_many_packed_fields(familyId, keys, fields));
    }
    const values = this._readKeyedMany<Record<string, unknown>>(familyId, keys);
    const packed = new Float32Array(values.length * fields.length);
    for (let valueIndex = 0; valueIndex < values.length; valueIndex += 1) {
      const value = values[valueIndex] ?? {};
      for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex += 1) {
        const field = fields[fieldIndex];
        const entry = Number((value as Record<string, unknown>)[field] ?? 0);
        packed[valueIndex * fields.length + fieldIndex] = Number.isFinite(entry) ? entry : 0;
      }
    }
    return packed;
  }

  readKeyedManyPackedFieldsInto(
    familyId: string,
    keys: string[],
    fields: string[],
    target: Float32Array,
    targetOffset = 0,
  ): Float32Array {
    const packed = this.readKeyedManyPackedFields(familyId, keys, fields);
    target.set(packed, targetOffset);
    return target;
  }

  readKeyedGridPackedFields(
    familyId: string,
    columns: number,
    rows: number,
    fields: string[],
  ): Float32Array {
    debugPackedSurface(
      `grid family=${familyId} size=${columns}x${rows} native=${typeof this.inner.read_keyed_grid_packed_fields === "function"}`,
    );
    if (typeof this.inner.read_keyed_grid_packed_fields === "function") {
      try {
        const values = Float32Array.from(
          this.inner.read_keyed_grid_packed_fields(familyId, columns, rows, fields),
        );
        this._drainDebugEvents();
        return values;
      } catch (error) {
        if (!isMissingWasmExportError(error)) {
          throw error;
        }
        debugPackedSurface(`grid family=${familyId} falling back after missing export`);
      }
    }
    debugPackedSurface(`grid family=${familyId} using keyed-many fallback`);
    return this.readKeyedManyPackedFields(familyId, buildFallbackTileKeys(columns, rows), fields);
  }

  readKeyedRectPackedFields(
    familyId: string,
    columns: number,
    rows: number,
    row: number,
    startColumn: number,
    width: number,
    height: number,
    fields: string[],
  ): Float32Array {
    debugPackedSurface(
      `rect family=${familyId} row=${row} start=${startColumn} size=${width}x${height} native=${typeof this.inner.read_keyed_rect_packed_fields === "function"}`,
    );
    if (typeof this.inner.read_keyed_rect_packed_fields === "function") {
      try {
        const values = Float32Array.from(
          this.inner.read_keyed_rect_packed_fields(
            familyId,
            columns,
            rows,
            row,
            startColumn,
            width,
            height,
            fields,
          ),
        );
        this._drainDebugEvents();
        return values;
      } catch (error) {
        if (!isMissingWasmExportError(error)) {
          throw error;
        }
        debugPackedSurface(`rect family=${familyId} falling back after missing export`);
      }
    }
    debugPackedSurface(`rect family=${familyId} using keyed-many fallback`);
    return this.readKeyedManyPackedFields(
      familyId,
      buildFallbackTileRectKeys(columns, row, startColumn, width, height),
      fields,
    );
  }

  prewarmKeyedGrid(familyId: string, columns: number, rows: number) {
    debugPackedSurface(
      `prewarm family=${familyId} size=${columns}x${rows} native=${typeof this.inner.prewarm_keyed_grid === "function"}`,
    );
    if (typeof this.inner.prewarm_keyed_grid === "function") {
      const startedAt = typeof performance !== "undefined" ? performance.now() : 0;
      this.inner.prewarm_keyed_grid(familyId, columns, rows);
      this._drainDebugEvents();
      const elapsedMs = typeof performance !== "undefined" ? performance.now() - startedAt : 0;
      debugPackedSurface(
        `prewarm family=${familyId} completed in ${elapsedMs.toFixed(2)} ms`,
      );
    } else {
      debugPackedSurface(`prewarm family=${familyId} unavailable; skipping`);
    }
  }

  seedKeyedGridCoords(familyId: string, columns: number, rows: number) {
    debugPackedSurface(
      `seed-grid family=${familyId} size=${columns}x${rows} native=${typeof this.inner.seed_keyed_grid_coords === "function"}`,
    );
    if (typeof this.inner.seed_keyed_grid_coords === "function") {
      try {
        const startedAt = typeof performance !== "undefined" ? performance.now() : 0;
        this.inner.seed_keyed_grid_coords(familyId, columns, rows);
        this._drainDebugEvents();
        const elapsedMs = typeof performance !== "undefined" ? performance.now() - startedAt : 0;
        debugPackedSurface(
          `seed-grid family=${familyId} completed in ${elapsedMs.toFixed(2)} ms`,
        );
        return;
      } catch (error) {
        if (!isMissingWasmExportError(error, "seed_keyed_grid_coords")) {
          throw error;
        }
        debugPackedSurface(`seed-grid family=${familyId} falling back after missing export`);
      }
    }
    debugPackedSurface(`seed-grid family=${familyId} using keyed-many fallback`);
    this.setKeyedMany(
      familyId,
      buildFallbackTileKeys(columns, rows).map((key, index) => ({
        key,
        value: {
          column: index % columns,
          row: Math.floor(index / columns),
        },
      })),
    );
  }

  _drainDebugEvents() {
    if (typeof this.inner.take_debug_events !== "function") {
      return;
    }
    const events = this.inner.take_debug_events();
    if (!Array.isArray(events)) {
      return;
    }
    for (const event of events) {
      if (typeof event === "string") {
        console.log(event);
      }
    }
  }

  setKeyedMany<T = unknown>(familyId: string, values: Array<{ key: string; value: T }>) {
    return this._setKeyedMany(familyId, values);
  }

  subscribe(listener: () => void) {
    this._globalListeners.add(listener);
    return () => {
      this._globalListeners.delete(listener);
    };
  }

  watch<T = unknown>(
    id: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean },
  ) {
    let entry = this._watchers.get(id);
    if (!entry) {
      entry = {
        read: () => this._read(id),
        listeners: new Set(),
        version: this._readVersions([id]).get(id) ?? null,
      };
      this._watchers.set(id, entry);
    }
    const typedListener = listener as (value: unknown) => void;
    entry.listeners.add(typedListener);
    if (options?.emitCurrent ?? true) {
      typedListener(entry.read());
    }
    return () => {
      const current = this._watchers.get(id);
      if (!current) return;
      current.listeners.delete(typedListener);
      if (current.listeners.size === 0) {
        this._watchers.delete(id);
      }
    };
  }

  watchKeyed<T = unknown>(
    familyId: string,
    key: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean },
  ) {
    const id = compositeKeyedId(familyId, key);
    let entry = this._watchers.get(id);
    if (!entry) {
      entry = {
        read: () => this._readKeyed(familyId, key),
        listeners: new Set(),
        version: this._readVersions([id]).get(id) ?? null,
      };
      this._watchers.set(id, entry);
    }
    const typedListener = listener as (value: unknown) => void;
    entry.listeners.add(typedListener);
    if (options?.emitCurrent ?? true) {
      typedListener(entry.read());
    }
    return () => {
      const current = this._watchers.get(id);
      if (!current) return;
      current.listeners.delete(typedListener);
      if (current.listeners.size === 0) {
        this._watchers.delete(id);
      }
    };
  }

  diagnostics() {
    return new SignalDiagnostics(this.inner.diagnostics());
  }

  history() {
    return new SignalHistory(this.inner.history());
  }

  specialist() {
    return new SignalSpecialist(this.inner.specialist());
  }

  adapters() {
    return new SignalAdapters(this.inner.adapters());
  }

  _read<T = unknown>(id: string): T {
    return decodeSignalValue(this.inner.read(id)) as T;
  }

  _set<T = unknown>(id: string, value: T) {
    return this.transaction([{ kind: "set", id, value }]);
  }

  _readKeyed<T = unknown>(familyId: string, key: string): T {
    return decodeSignalValue(this.inner.read_keyed(familyId, key)) as T;
  }

  _setKeyed<T = unknown>(familyId: string, key: string, value: T) {
    const result = this.inner.set_keyed(familyId, key, encodeSignalValue(value as any));
    this._flushWatchers();
    return result;
  }

  _readKeyedMany<T = unknown>(familyId: string, keys: string[]): T[] {
    return this.inner.read_keyed_many(familyId, keys).map(decodeSignalValue) as T[];
  }

  _setKeyedMany<T = unknown>(familyId: string, values: Array<{ key: string; value: T }>) {
    const result = this.inner.set_keyed_many(
      familyId,
      values.map(({ key, value }) => ({
        key,
        value: encodeSignalValue(value as any),
      })),
    );
    this._flushWatchers();
    return result;
  }

  clearKeyedFamilyCache(familyId: string) {
    this.inner.clear_keyed_family_cache(familyId);
    this._flushWatchers();
  }

  _readVersions(ids: string[]): Map<string, number> {
    const summaries = this.specialist().readVersions(ids) ?? [];
    return new Map(summaries.map((summary: any) => [summary.id, Number(summary.version)]));
  }

  _flushWatchers() {
    if (this._watchers.size > 0) {
      const ids = Array.from(this._watchers.keys());
      const versions = this._readVersions(ids);
      for (const [id, entry] of this._watchers.entries()) {
        const nextVersion = versions.get(id) ?? null;
        if (entry.version !== nextVersion) {
          entry.version = nextVersion;
          const value = entry.read();
          for (const listener of entry.listeners) {
            listener(value);
          }
        }
      }
    }
    for (const listener of this._globalListeners) {
      listener();
    }
  }
}

function buildFallbackTileKeys(columns: number, rows: number): string[] {
  const keys: string[] = [];
  for (let row = 0; row < rows; row += 1) {
    for (let column = 0; column < columns; column += 1) {
      keys.push(`tile-${column}-${row}`);
    }
  }
  return keys;
}

function buildFallbackTileRectKeys(
  columns: number,
  row: number,
  startColumn: number,
  width: number,
  height: number,
): string[] {
  const keys: string[] = [];
  for (let rowOffset = 0; rowOffset < height; rowOffset += 1) {
    const currentRow = row + rowOffset;
    for (let columnOffset = 0; columnOffset < width; columnOffset += 1) {
      const currentColumn = startColumn + columnOffset;
      keys.push(`tile-${currentColumn}-${currentRow}`);
    }
  }
  return keys;
}

function isMissingWasmExportError(error: unknown, exportName?: string): boolean {
  if (!(error instanceof TypeError)) {
    return false;
  }
  if (exportName) {
    return new RegExp(`wasm\\..*${exportName}.*is not a function`).test(error.message);
  }
  return /wasm\..*(read_keyed_(grid|rect)_packed_fields|seed_keyed_grid_coords|read_many).*is not a function/.test(error.message);
}

function debugPackedSurface(message: string) {
  if (typeof console !== "undefined") {
    console.log(`[forge-signal] packed-surface ${message}`);
  }
}
