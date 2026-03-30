import {
  encodeSignalValue,
  normalizeRecipeFamilySpec,
  normalizeRecipeSpec,
  normalizeSourceFamilySpec,
  normalizeSourceSpec,
  normalizeTransactionOp,
  decodeSignalValue
} from "../internal/codec.js";
import {
  RecipeFamilyHandle,
  RecipeHandle,
  SourceFamilyHandle,
  SourceHandle
} from "./handles.js";
import { SignalDiagnostics } from "./diagnostics.js";
import { SignalHistory } from "./history.js";
import { SignalSpecialist } from "./specialist.js";
import { SignalAdapters } from "./adapters.js";

export class SignalRuntime {
  constructor(inner) {
    this.inner = inner;
  }

  setRuntimePolicy(policySpec) {
    this.inner.set_runtime_policy(policySpec);
    return this;
  }

  defineSource(spec) {
    const normalized = normalizeSourceSpec(spec);
    this.inner.define_source(normalized);
    return new SourceHandle(this, normalized.id);
  }

  defineRecipe(spec) {
    const normalized = normalizeRecipeSpec(spec);
    this.inner.define_recipe(normalized);
    return new RecipeHandle(this, normalized.id);
  }

  defineSourceFamily(spec) {
    const normalized = normalizeSourceFamilySpec(spec);
    this.inner.define_source_family(normalized);
    return new SourceFamilyHandle(this, normalized.familyId);
  }

  defineRecipeFamily(spec) {
    const normalized = normalizeRecipeFamilySpec(spec);
    this.inner.define_recipe_family(normalized);
    return new RecipeFamilyHandle(this, normalized.familyId);
  }

  transaction(ops) {
    const normalized = ops.map(normalizeTransactionOp);
    const packedIndex = normalized.findIndex((op) => op.kind === "setPackedGridRgba");
    if (packedIndex === -1) {
      return this.inner.transaction(normalized);
    }

    const packed = normalized[packedIndex];
    const before = normalized.slice(0, packedIndex);
    const after = normalized.slice(packedIndex + 1);
    const laterPacked = after.find((op) => op.kind === "setPackedGridRgba");
    if (laterPacked) {
      throw new Error("Only one packed grid RGBA op is supported per transaction.");
    }

    return this.inner.transaction_with_packed_grid_rgba(
      before,
      packed.familyId,
      packed.width,
      packed.height,
      packed.rgba,
      after
    );
  }

  read(id) {
    return this._read(id);
  }

  readKeyed(familyId, key) {
    return this._readKeyed(familyId, key);
  }

  setKeyed(familyId, key, value) {
    return this._setKeyed(familyId, key, value);
  }

  readKeyedMany(familyId, keys) {
    return this._readKeyedMany(familyId, keys);
  }

  setKeyedMany(familyId, values) {
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

  _read(id) {
    return decodeSignalValue(this.inner.read(id));
  }

  _set(id, value) {
    return this.transaction([{ kind: "set", id, value }]);
  }

  _readKeyed(familyId, key) {
    return decodeSignalValue(this.inner.read_keyed(familyId, key));
  }

  _setKeyed(familyId, key, value) {
    return this.inner.set_keyed(familyId, key, encodeSignalValue(value));
  }

  _readKeyedMany(familyId, keys) {
    return this.inner.read_keyed_many(familyId, keys).map(decodeSignalValue);
  }

  _setKeyedMany(familyId, values) {
    return this.inner.set_keyed_many(
      familyId,
      values.map(({ key, value }) => ({
        key,
        value: encodeSignalValue(value)
      }))
    );
  }
}
