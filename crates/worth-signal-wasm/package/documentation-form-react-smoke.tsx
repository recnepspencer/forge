import type { FormEvent } from "react";
import { createSignals } from "./index.js";
import {
  createReactSignalsStore,
  useSignalsForm,
} from "../react/index.js";
import type {
  SignalsFormActionBinding,
  SignalsFormFieldBinding,
} from "../react/index.js";

function invalidField(field: string, code: string, message: string) {
  return {
    kind: "invalid",
    field,
    message: {
      code,
      message,
      severity: "error",
      target: field,
      audience: "user",
      visibility: "visible",
    },
  } as const;
}

const profileRules = {
  email(value: string) {
    return value.includes("@")
      ? true
      : invalidField("email", "email.invalid", "Enter a complete email.");
  },
  seats(value: number) {
    return value >= 1
      ? true
      : invalidField("seats", "seats.minimum", "Choose at least one seat.");
  },
};

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const store = createReactSignalsStore(signals);
const profiles = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/profiles/:profileId").detail<{
  email: string;
  seats: number;
}>({
  load: async () => ({ email: "ada@example.com", seats: 1 }),
});
const profileLine = profiles.line({ profileId: "ada" });

const profileForm = signals.form.define({
  id: "profile-editor",
  source: signals.form.source.resourceLine(profileLine, { id: "profile" }),
  fields: ({ field }) => ({
    email: field<string>("email", { label: "Email" }),
    seats: field<number, string>("seats", {
      label: "Seats",
      parse: (raw) => Number.parseInt(raw, 10),
    }),
  }),
  validation: ({ field }) => ({
    email: field<string>("email", profileRules.email),
    seats: field<number>("seats", profileRules.seats),
  }),
  actions: ({ submit }) => ({
    submit: submit(),
  }),
});

interface TextInputProps {
  readonly label: string;
  readonly field: SignalsFormFieldBinding<string, string>;
}

function TextInput({ field, label }: TextInputProps) {
  return <input aria-label={label} {...textInputProps(field)} />;
}

interface NumberInputProps {
  readonly label: string;
  readonly field: SignalsFormFieldBinding<number, string>;
  readonly min?: number;
}

function NumberInput({
  field,
  label,
  min,
}: NumberInputProps) {
  return <input aria-label={label} type="number" min={min} {...textInputProps(field)} />;
}

function textInputProps<TValue extends string | number>(
  field: SignalsFormFieldBinding<TValue, string>,
) {
  const message = field.messages.find((entry) => entry.audience === "user");
  return {
    name: field.name,
    value: field.value,
    disabled: field.disabled,
    readOnly: field.readOnly,
    onChange: field.onChange,
    onFocus: field.onFocus,
    onBlur: field.onBlur,
    "aria-invalid": message?.severity === "error",
  };
}

function SubmitButton({
  action,
  children,
}: {
  readonly action: SignalsFormActionBinding;
  readonly children: string;
}) {
  return (
    <button type="submit" disabled={action.disabled}>
      {action.pending ? "Saving..." : children}
    </button>
  );
}

function useProfileEditor() {
  const form = useSignalsForm(profileForm);
  const submit = form.action("submit");
  // @ts-expect-error profileForm declares only the submit action.
  form.action("archive");

  async function save(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await submit.execute();
  }

  return {
    email: form.field<string, string>("email"),
    seats: form.field<number, string>("seats"),
    save,
    submit,
  };
}

export function ProfileEditor() {
  const editor = useProfileEditor();

  return (
    <form onSubmit={editor.save}>
      <TextInput
        label="Email"
        field={editor.email}
      />
      <NumberInput
        label="Seats"
        min={1}
        field={editor.seats}
      />
      <SubmitButton action={editor.submit}>Save profile</SubmitButton>
    </form>
  );
}

void store;
