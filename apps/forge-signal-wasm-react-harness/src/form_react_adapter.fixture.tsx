import React from "react";

import {
  useFormAction,
  useFormField,
  useSignalsForm,
} from "@aust-group/forge-signal-wasm/react";

import {
  createFakeForm,
  createFakeStore,
} from "./form_react_adapter.fake_runtime";

export { createFakeForm, createFakeStore } from "./form_react_adapter.fake_runtime";

export function FormFieldProbe({
  form,
  store,
}: {
  form: ReturnType<typeof createFakeForm>;
  store: ReturnType<typeof createFakeStore>;
}): JSX.Element {
  const title = useFormField<string, string>(form, "title", store as never);
  const published = useFormField<boolean, boolean>(form, "published", store as never);
  const submit = useFormAction(form, "submit", store as never);
  const titleProps = title.textInput();
  const publishedProps = published.checkboxInput();

  return (
    <div>
      <input data-testid="title" value={titleProps.value} onChange={(event) => titleProps.onChange(event)} onBlur={() => titleProps.onBlur()} />
      <input data-testid="published" type="checkbox" checked={publishedProps.checked} onChange={(event) => publishedProps.onChange(event)} />
      <button data-testid="submit" disabled={submit.disabled} onClick={() => submit.execute()}>
        {submit.pending ? "pending" : "ready"}
      </button>
      <div data-testid="submit-result-kind">{submit.resultKind ?? "none"}</div>
      <div data-testid="title-messages">{title.messages.length}</div>
      <div data-testid="title-dirty">{String((title.dirty as { isDirty: boolean }).isDirty)}</div>
    </div>
  );
}

export function SignalsFormProbe({
  store,
  remountKey,
}: {
  store: ReturnType<typeof createFakeStore>;
  remountKey?: unknown;
}): JSX.Element {
  const form = useSignalsForm({
    source: {
      title: "",
      published: false,
      role: "editor",
      appIds: [],
    },
    fields: {
      title: {} as never,
      published: {} as never,
      role: {} as never,
      appIds: {} as never,
    },
    actions: {
      submit: {} as never,
    },
  }, store as never, { remountKey });

  const titleProps = form.field<string, string>("title");
  const publishedProps = form.checkbox<boolean>("published");
  const roleProps = form.select<string, string>("role", [
    { label: "Editor", value: "editor" },
    { label: "Admin", value: "admin" },
  ]);
  const appIdsProps = form.multiSelect<string>("appIds", [
    { label: "Northstar", value: "northstar" },
    { label: "Orbit", value: "orbit" },
  ]);

  return (
    <div>
      <input data-testid="signals-title" value={titleProps.value} onChange={(event) => titleProps.onChange(event)} onBlur={() => titleProps.onBlur()} />
      <input data-testid="signals-published" type="checkbox" checked={publishedProps.checked} onChange={(event) => publishedProps.onChange(event)} />
      <select data-testid="signals-role" value={roleProps.value} onChange={(event) => roleProps.onChange(event)}>
        {roleProps.options.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
      </select>
      <select data-testid="signals-appIds" multiple value={[...appIdsProps.value]} onChange={(event) => appIdsProps.onChange(event)}>
        {appIdsProps.options.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
      </select>
      <div data-testid="signals-title-messages">{form.fieldState("title").messages.length}</div>
      <div data-testid="signals-effective-title">{String((form.effective as { title?: string }).title ?? "")}</div>
      <div data-testid="signals-patch-empty">{String((form.patchPlan as { empty: boolean }).empty)}</div>
      <button data-testid="signals-submit" disabled={form.actions.submit.disabled} onClick={() => form.actions.submit.execute()}>
        {form.actions.submit.pending ? "pending" : "ready"}
      </button>
      <div data-testid="signals-submit-result-kind">{form.actions.submit.resultKind ?? "none"}</div>
      <button data-testid="signals-reset" onClick={() => form.reset()}>
        reset
      </button>
      <div data-testid="signals-dirty">{String(form.dirty.isDirty)}</div>
    </div>
  );
}
