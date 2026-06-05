import React from "react";

import {
  RESOURCE_DEMO_DIAGNOSTICS_JSON_SCHEMA,
  renderResourceDemoDiagnostics,
  type ListItem,
  type ResourceDemoDiagnostics,
} from "./resourcesSectionSupport";

function useVisibleToasts(events: readonly { id: string; tone: string; title: string; detail: string }[]): readonly {
  id: string;
  tone: string;
  title: string;
  detail: string;
}[] {
  const [visible, setVisible] = React.useState<readonly {
    id: string;
    tone: string;
    title: string;
    detail: string;
  }[]>([]);
  const latestId = events[0]?.id ?? null;

  React.useEffect(() => {
    if (!latestId) {
      return;
    }
    const next = events[0];
    setVisible((current) => [next, ...current.filter((entry) => entry.id !== next.id)].slice(0, 2));
    const timeout = window.setTimeout(() => {
      setVisible((current) => current.filter((entry) => entry.id !== next.id));
    }, 2200);
    return () => window.clearTimeout(timeout);
  }, [events, latestId]);

  return visible;
}

export function ResourcePanel({
  items,
  error,
  events,
  loading,
  title,
  variant,
}: {
  items: readonly ListItem[] | null;
  error?: string | null;
  loading: boolean;
  events: readonly { id: string; tone: string; title: string; detail: string }[];
  title: string;
  variant: string;
}): React.ReactElement {
  const toasts = useVisibleToasts(events);

  return (
    <article className="resources-window">
      <div className="resources-window-header">
        <span>{variant}</span>
        <strong>{title}</strong>
      </div>

      <div className="resources-window-body">
        <div className="resources-window-card">
          {error ? (
            <div className="resources-window-empty is-error">{error}</div>
          ) : loading || !items ? (
            <div className="resources-window-empty">Loading items...</div>
          ) : (
            <>
              <span className="resources-window-label">Packing list</span>
              <div className="resources-list">
                {items.map((item) => (
                  <div className="resources-list-item" key={item.id}>
                    <strong>{item.label}</strong>
                    <span className={`resources-list-sync resources-list-sync-${item.sync}`}>{item.sync}</span>
                  </div>
                ))}
              </div>
            </>
          )}
        </div>

        <div className="resources-toast-stack" aria-live="polite">
          {toasts.map((toast) => (
            <div className={`resources-toast resources-toast-${toast.tone}`} key={toast.id}>
              <strong>{toast.title}</strong>
              <span>{toast.detail}</span>
            </div>
          ))}
        </div>
      </div>
    </article>
  );
}

export function DiagnosticsComparison({
  forgeDiagnostics,
  tanstackDiagnostics,
}: {
  forgeDiagnostics: ResourceDemoDiagnostics | null;
  tanstackDiagnostics: ResourceDemoDiagnostics | null;
}): React.ReactElement {
  const tanstackJson = JSON.stringify(tanstackDiagnostics);
  const forgeJson = JSON.stringify(forgeDiagnostics);
  const schemaJson = JSON.stringify(RESOURCE_DEMO_DIAGNOSTICS_JSON_SCHEMA);

  return (
    <div className="resources-diagnostics-grid">
      <code data-resource-diagnostics-json-schema="resource-demo-diagnostics.v1" hidden>{schemaJson}</code>
      <div className="resources-diagnostics-column">
        <span>TanStack diagnostics</span>
        <code data-resource-diagnostics="tanstack" data-resource-diagnostics-schema="resource-demo-diagnostics.v1" hidden>{tanstackJson}</code>
        <pre className="resources-terminal">
          {renderResourceDemoDiagnostics(tanstackDiagnostics).join("\n")}
        </pre>
      </div>

      <div className="resources-diagnostics-column">
        <span>Forge diagnostics</span>
        <code data-resource-diagnostics="forge" data-resource-diagnostics-schema="resource-demo-diagnostics.v1" hidden>{forgeJson}</code>
        <pre className="resources-terminal">
          {renderResourceDemoDiagnostics(forgeDiagnostics).join("\n")}
        </pre>
      </div>
    </div>
  );
}

export function ResourceFeedbackGuide(): React.ReactElement {
  return (
    <div className="resources-feedback-guide">
      <span>Feedback to watch</span>
      <div>
        <strong>Optimistic row</strong>
        <p>The item appears before the server answers.</p>
      </div>
      <div>
        <strong>Toast feed</strong>
        <p>The UI can map lifecycle feedback into success or failure messages.</p>
      </div>
      <div>
        <strong>Line lifecycle</strong>
        <p>Forge records the admitted effect and whether rollback is available.</p>
      </div>
      <div>
        <strong>Rollback result</strong>
        <p>On failure, the runtime can restore the previous visible list.</p>
      </div>
    </div>
  );
}
