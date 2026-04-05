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

export class SignalApp {
  inner: any;
  _watchers: Map<string, { read: () => unknown; listeners: Set<(value: unknown) => void>; version: number | null }>;
  _globalListeners: Set<() => void>;

  constructor(inner: any) {
    this.inner = inner;
    this._watchers = new Map();
    this._globalListeners = new Set();
  }

  source<T = unknown>(spec: any): SourceHandle<T> {
    const normalized = normalizeSourceSpec(spec);
    this.inner.source(normalized);
    return new SourceHandle<T>(this, normalized.id);
  }

  recipe<T = unknown>(spec: any): RecipeHandle<T> {
    const normalized = normalizeRecipeSpec(spec);
    this.inner.recipe(normalized);
    return new RecipeHandle<T>(this, normalized.id);
  }

  sourceFamily<T = unknown>(spec: any): SourceFamilyHandle<T> {
    const normalized = normalizeSourceFamilySpec(spec);
    this.inner.source_family(normalized);
    return new SourceFamilyHandle<T>(this, normalized.familyId);
  }

  recipeFamily<T = unknown>(spec: any): RecipeFamilyHandle<T> {
    const normalized = normalizeRecipeFamilySpec(spec);
    this.inner.recipe_family(normalized);
    return new RecipeFamilyHandle<T>(this, normalized.familyId);
  }

  batch(ops: any[]) {
    const normalized = ops.map(normalizeTransactionOp);
    const packedIndex = normalized.findIndex((op: any) => op.kind === "setPackedGridRgba");
    if (packedIndex === -1) {
      const result = this.inner.batch(normalized);
      this._flushWatchers();
      return result;
    }

    const packed: any = normalized[packedIndex];
    const before = normalized.slice(0, packedIndex);
    const after = normalized.slice(packedIndex + 1);
    const laterPacked = after.find((op: any) => op.kind === "setPackedGridRgba");
    if (laterPacked) {
      throw new Error("Only one packed grid RGBA op is supported per batch.");
    }

    const result = this.inner.transaction_with_packed_grid_rgba(
      before,
      packed.familyId,
      packed.width,
      packed.height,
      packed.rgba,
      after,
    );
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
      }
    }
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
    if (typeof this.inner.read_keyed_grid_packed_fields === "function") {
      try {
        return Float32Array.from(
          this.inner.read_keyed_grid_packed_fields(familyId, columns, rows, fields),
        );
      } catch (error) {
        if (!isMissingWasmExportError(error)) {
          throw error;
        }
      }
    }
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
    if (typeof this.inner.read_keyed_rect_packed_fields === "function") {
      try {
        return Float32Array.from(
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
      } catch (error) {
        if (!isMissingWasmExportError(error)) {
          throw error;
        }
      }
    }
    return this.readKeyedManyPackedFields(
      familyId,
      buildFallbackTileRectKeys(columns, row, startColumn, width, height),
      fields,
    );
  }

  prewarmKeyedGrid(familyId: string, columns: number, rows: number) {
    if (typeof this.inner.prewarm_keyed_grid === "function") {
      this.inner.prewarm_keyed_grid(familyId, columns, rows);
    }
  }

  seedKeyedGridCoords(familyId: string, columns: number, rows: number) {
    if (typeof this.inner.seed_keyed_grid_coords === "function") {
      try {
        this.inner.seed_keyed_grid_coords(familyId, columns, rows);
        return;
      } catch (error) {
        if (!isMissingWasmExportError(error, "seed_keyed_grid_coords")) {
          throw error;
        }
      }
    }
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

  setKeyedMany<T = unknown>(familyId: string, values: Array<{ key: string; value: T }>) {
    return this._setKeyedMany(familyId, values);
  }

  clearKeyedFamilyCache(familyId: string) {
    if (typeof this.inner.clear_keyed_family_cache === "function") {
      this.inner.clear_keyed_family_cache(familyId);
      this._flushWatchers();
    }
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
    return this.batch([{ kind: "set", id, value }]);
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
  return /wasm\..*(read_keyed_(grid|rect)_packed_fields|seed_keyed_grid_coords).*is not a function/.test(error.message);
}
