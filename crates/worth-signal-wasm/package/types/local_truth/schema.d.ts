declare const declaredLocalTruthSchemaBrand: unique symbol;

export type LocalTruthValueType = "any" | "boolean" | "number" | "string";
export type LocalTruthCostClass = "constant" | "linearInValue";

export type LocalTruthEquivalence =
  | { readonly kind: "exact" }
  | { readonly kind: "numberEpsilon"; readonly epsilon: number };

export interface LocalTruthAspectDeclaration<
  T extends object,
  K extends Extract<keyof T, string> = Extract<keyof T, string>,
> {
  readonly id: string;
  readonly field: K;
  readonly valueType: LocalTruthValueType;
  readonly equivalence: LocalTruthEquivalence;
  readonly costClass: LocalTruthCostClass;
}

export interface LocalTruthSchemaDeclaration<T extends object> {
  readonly id: string;
  readonly version?: number;
  readonly aspects: ReadonlyArray<LocalTruthAspectDeclaration<T>>;
}

export interface DeclaredLocalTruthSchema<T extends object> {
  readonly artifactFamily: "DeclaredLocalTruthSchema";
  readonly authorityKind: "typescriptInMemoryLocalTruth";
  readonly id: string;
  readonly version: number;
  readonly identity: string;
  readonly aspects: ReadonlyArray<LocalTruthAspectDeclaration<T>>;
  readonly [declaredLocalTruthSchemaBrand]: true;
}
