import {
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

  constructor(inner: any) {
    this.inner = inner;
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
      return this.inner.batch(normalized);
    }

    const packed: any = normalized[packedIndex];
    const before = normalized.slice(0, packedIndex);
    const after = normalized.slice(packedIndex + 1);
    const laterPacked = after.find((op: any) => op.kind === "setPackedGridRgba");
    if (laterPacked) {
      throw new Error("Only one packed grid RGBA op is supported per batch.");
    }

    return this.inner.transaction_with_packed_grid_rgba(
      before,
      packed.familyId,
      packed.width,
      packed.height,
      packed.rgba,
      after,
    );
  }

  read<T = unknown>(id: string): T {
    return this._read(id);
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
    return this.inner.set_keyed(familyId, key, encodeSignalValue(value as any));
  }

  _readKeyedMany<T = unknown>(familyId: string, keys: string[]): T[] {
    return this.inner.read_keyed_many(familyId, keys).map(decodeSignalValue) as T[];
  }

  _setKeyedMany<T = unknown>(familyId: string, values: Array<{ key: string; value: T }>) {
    return this.inner.set_keyed_many(
      familyId,
      values.map(({ key, value }) => ({
        key,
        value: encodeSignalValue(value as any),
      })),
    );
  }
}
