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

  handle<T = unknown>(id: string): SourceHandle<T> | RecipeHandle<T> {
    return new SourceHandle<T>(this, id);
  }

  readKeyed<T = unknown>(familyId: string, key: string): T {
    return this._readKeyed(familyId, key);
  }

  setKeyed<T = unknown>(familyId: string, key: string, value: T) {
    return this._setKeyed(familyId, key, value);
  }

  readKeyedMany<T = unknown>(familyId: string, keys: string[]): T[] {
    return this._readKeyedMany(familyId, keys);
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
