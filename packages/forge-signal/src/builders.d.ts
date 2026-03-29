import type {
  BranchMergePlan,
  BranchMergeResult,
  Expr,
  ExprInput,
  KeyedReadSpec,
  KeyedRecipeFamilySpec,
  KeyedSourceFamilySpec,
  RuntimePolicy,
  RuntimePolicyPreset,
  SignalPrimitive,
  SignalValue,
  SourceSpec,
  RecipeSpec,
  TransactionOp
} from "./types";
import type {
  KeyedRecipeHandle,
  KeyedSourceHandle,
  RecipeFamilyHandle,
  RecipeHandle,
  SignalHandle,
  SourceFamilyHandle,
  SourceHandle
} from "./surface";

export function initForgeSignal(
  input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module
): Promise<unknown>;
export function createSignalApp(): Promise<import("./surface").SignalApp>;
export function createSignalRuntime(): Promise<import("./surface").SignalRuntime>;

export class SourceBuilder<T = SignalValue> {
  constructor(id: string);
  initial(value: T): this;
  build(): SourceSpec<T>;
}

export class RecipeBuilder<T = SignalValue> {
  constructor(id: string);
  reads(...reads: Array<string | SignalHandle<any> | SourceHandle<any> | RecipeHandle<any>>): this;
  expr(expr: Expr<T>): this;
  when(expr: ExprInput<boolean>): this;
  identityExact(): this;
  identity(expr: ExprInput<SignalValue>): this;
  build(): RecipeSpec<T>;
}

export class SourceFamilyBuilder<T = SignalValue> {
  constructor(familyId: string);
  initial(value: T): this;
  build(): KeyedSourceFamilySpec<T>;
}

export class RecipeFamilyBuilder<T = SignalValue> {
  constructor(familyId: string);
  reads(
    ...reads: Array<
      string | KeyedReadSpec | SourceFamilyHandle<any> | RecipeFamilyHandle<any>
    >
  ): this;
  expr(expr: Expr<T>): this;
  when(expr: ExprInput<boolean>): this;
  identityExact(): this;
  identity(expr: ExprInput<SignalValue>): this;
  build(): KeyedRecipeFamilySpec<T>;
}

export const expr: {
  value<T extends SignalValue>(value: T): Expr<T>;
  read<T = SignalValue>(id: string): Expr<T>;
  get<T = SignalValue>(target: Expr<Record<string, T>>, field: string): Expr<T>;
  at<T = SignalValue>(target: Expr<T[]>, index: ExprInput<number>): Expr<T>;
  first<T = SignalValue>(target: Expr<T[]>): Expr<T>;
  last<T = SignalValue>(target: Expr<T[]>): Expr<T>;
  slice<T = SignalValue>(target: Expr<T[]>, start: ExprInput<number>, end?: ExprInput<number>): Expr<T[]>;
  join(target: Expr<SignalValue[]>, separator: ExprInput<string>): Expr<string>;
  flatten<T = SignalValue>(target: Expr<T[][]>): Expr<T[]>;
  object<T extends Record<string, SignalValue>>(
    fields: { [K in keyof T]: ExprInput<T[K]> } | Array<[string, ExprInput<SignalValue>]>
  ): Expr<T>;
  array<T = SignalValue>(items: Array<ExprInput<T>>): Expr<T[]>;
  sum(...args: Array<ExprInput<number>>): Expr<number>;
  multiply(...args: Array<ExprInput<number>>): Expr<number>;
  concat(...args: Array<ExprInput<SignalPrimitive>>): Expr<string>;
  coalesce<T = SignalValue>(...args: Array<ExprInput<T | null>>): Expr<T | null>;
  length(target: ExprInput<SignalValue>): Expr<number>;
  contains(target: ExprInput<SignalValue>, value: ExprInput<SignalValue>): Expr<boolean>;
  mergeObjects<T extends Record<string, SignalValue>>(...args: Array<ExprInput<T>>): Expr<T>;
  keys(target: Expr<Record<string, SignalValue>>): Expr<string[]>;
  values(target: Expr<Record<string, SignalValue>>): Expr<SignalValue[]>;
  hasField(target: Expr<Record<string, SignalValue>>, field: string): Expr<boolean>;
  pick<T extends Record<string, SignalValue>, K extends keyof T & string>(
    target: Expr<T>,
    ...fields: K[]
  ): Expr<Pick<T, K>>;
  omit<T extends Record<string, SignalValue>, K extends keyof T & string>(
    target: Expr<T>,
    ...fields: K[]
  ): Expr<Omit<T, K>>;
  append<T = SignalValue>(target: Expr<T[]>, value: ExprInput<T>): Expr<T[]>;
  subtract(left: ExprInput<number>, right: ExprInput<number>): Expr<number>;
  divide(left: ExprInput<number>, right: ExprInput<number>): Expr<number>;
  eq(left: ExprInput<SignalValue>, right: ExprInput<SignalValue>): Expr<boolean>;
  neq(left: ExprInput<SignalValue>, right: ExprInput<SignalValue>): Expr<boolean>;
  gt(left: ExprInput<number>, right: ExprInput<number>): Expr<boolean>;
  gte(left: ExprInput<number>, right: ExprInput<number>): Expr<boolean>;
  lt(left: ExprInput<number>, right: ExprInput<number>): Expr<boolean>;
  lte(left: ExprInput<number>, right: ExprInput<number>): Expr<boolean>;
  and(...args: Array<ExprInput<boolean>>): Expr<boolean>;
  or(...args: Array<ExprInput<boolean>>): Expr<boolean>;
  not(arg: ExprInput<boolean>): Expr<boolean>;
  ifElse<T = SignalValue>(condition: ExprInput<boolean>, thenExpr: ExprInput<T>, elseExpr: ExprInput<T>): Expr<T>;
};

export const define: {
  source<T = SignalValue>(id: string): SourceBuilder<T>;
  recipe<T = SignalValue>(id: string): RecipeBuilder<T>;
  sourceFamily<T = SignalValue>(familyId: string): SourceFamilyBuilder<T>;
  recipeFamily<T = SignalValue>(familyId: string): RecipeFamilyBuilder<T>;
};

export const keyed: {
  read(
    family:
      | string
      | KeyedReadSpec
      | SourceFamilyHandle<any>
      | RecipeFamilyHandle<any>
  ): KeyedReadSpec;
};

export const tx: {
  set<T = SignalValue>(id: string, value: T): TransactionOp<T>;
  setMany<T = SignalValue>(values: Array<{ id: string; value: T }>): TransactionOp<T>;
};

export const policy: {
  preset(preset: RuntimePolicyPreset): RuntimePolicy;
};
