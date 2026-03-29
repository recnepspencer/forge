import { compositeKeyedId } from "../internal/codec.js";

export class SignalHandle {
  constructor(owner, id) {
    this.owner = owner;
    this.id = id;
  }

  read() {
    return this.owner._read(this.id);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class SourceHandle extends SignalHandle {
  set(value) {
    return this.owner._set(this.id, value);
  }
}

export class RecipeHandle extends SignalHandle {}

export class KeyedSourceHandle {
  constructor(owner, familyId, key) {
    this.owner = owner;
    this.familyId = familyId;
    this.key = key;
    this.id = compositeKeyedId(familyId, key);
  }

  read() {
    return this.owner._readKeyed(this.familyId, this.key);
  }

  set(value) {
    return this.owner._setKeyed(this.familyId, this.key, value);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class KeyedRecipeHandle {
  constructor(owner, familyId, key) {
    this.owner = owner;
    this.familyId = familyId;
    this.key = key;
    this.id = compositeKeyedId(familyId, key);
  }

  read() {
    return this.owner._readKeyed(this.familyId, this.key);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class SourceFamilyHandle {
  constructor(owner, familyId) {
    this.owner = owner;
    this.familyId = familyId;
  }

  toRead() {
    return { familyId: this.familyId };
  }

  key(key) {
    return new KeyedSourceHandle(this.owner, this.familyId, key);
  }

  read(key) {
    return this.owner._readKeyed(this.familyId, key);
  }

  set(key, value) {
    return this.owner._setKeyed(this.familyId, key, value);
  }
}

export class RecipeFamilyHandle {
  constructor(owner, familyId) {
    this.owner = owner;
    this.familyId = familyId;
  }

  toRead() {
    return { familyId: this.familyId };
  }

  key(key) {
    return new KeyedRecipeHandle(this.owner, this.familyId, key);
  }

  read(key) {
    return this.owner._readKeyed(this.familyId, key);
  }
}
