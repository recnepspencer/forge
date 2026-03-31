import { compositeKeyedId } from "../internal/codec.ts";

export class SignalHandle<T = unknown> {
  owner: any;
  id: string;

  constructor(owner: any, id: string) {
    this.owner = owner;
    this.id = id;
  }

  read(): T {
    return this.owner._read(this.id);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class SourceHandle<T = unknown> extends SignalHandle<T> {
  set(value: T) {
    return this.owner._set(this.id, value);
  }
}

export class RecipeHandle<T = unknown> extends SignalHandle<T> {}

export class KeyedSourceHandle<T = unknown> {
  owner: any;
  familyId: string;
  key: string;
  id: string;

  constructor(owner: any, familyId: string, key: string) {
    this.owner = owner;
    this.familyId = familyId;
    this.key = key;
    this.id = compositeKeyedId(familyId, key);
  }

  read(): T {
    return this.owner._readKeyed(this.familyId, this.key);
  }

  set(value: T) {
    return this.owner._setKeyed(this.familyId, this.key, value);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class KeyedRecipeHandle<T = unknown> {
  owner: any;
  familyId: string;
  key: string;
  id: string;

  constructor(owner: any, familyId: string, key: string) {
    this.owner = owner;
    this.familyId = familyId;
    this.key = key;
    this.id = compositeKeyedId(familyId, key);
  }

  read(): T {
    return this.owner._readKeyed(this.familyId, this.key);
  }

  why() {
    return this.owner.diagnostics().why(this.id);
  }
}

export class SourceFamilyHandle<T = unknown> {
  owner: any;
  familyId: string;

  constructor(owner: any, familyId: string) {
    this.owner = owner;
    this.familyId = familyId;
  }

  toRead() {
    return { familyId: this.familyId };
  }

  key(key: string) {
    return new KeyedSourceHandle<T>(this.owner, this.familyId, key);
  }

  read(key: string): T {
    return this.owner._readKeyed(this.familyId, key);
  }

  set(key: string, value: T) {
    return this.owner._setKeyed(this.familyId, key, value);
  }
}

export class RecipeFamilyHandle<T = unknown> {
  owner: any;
  familyId: string;

  constructor(owner: any, familyId: string) {
    this.owner = owner;
    this.familyId = familyId;
  }

  toRead() {
    return { familyId: this.familyId };
  }

  key(key: string) {
    return new KeyedRecipeHandle<T>(this.owner, this.familyId, key);
  }

  read(key: string): T {
    return this.owner._readKeyed(this.familyId, key);
  }
}
