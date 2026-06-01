function CodeLine({ children }: { children?: React.ReactNode }) {
  return <div className="resources-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain" | "comment";
  children: React.ReactNode;
}) {
  return <span className={`resources-tok resources-tok-${kind}`}>{children}</span>;
}

function TanStackSample() {
  return (
    <div className="resources-code-block" role="presentation">
      <CodeLine><Tok kind="comment">// packingListQuery and addPackingItem are app-level helpers.</Tok></CodeLine>
      <CodeLine />
      <CodeLine><Tok kind="kw">const</Tok> <Tok kind="var">items</Tok> <Tok kind="op">=</Tok> <Tok kind="fn">useQuery</Tok>(<Tok kind="var">packingListQuery</Tok>);</CodeLine>
      <CodeLine><Tok kind="kw">const</Tok> <Tok kind="var">addItem</Tok> <Tok kind="op">=</Tok> <Tok kind="fn">useMutation</Tok>({`{`}</CodeLine>
      <CodeLine>{`  mutationFn: addPackingItem,`}</CodeLine>
      <CodeLine>{`  onMutate: async () => {`}</CodeLine>
      <CodeLine>{`    const previous = queryClient.getQueryData<ListItem[]>(["items"]);`}</CodeLine>
      <CodeLine>{`    queryClient.setQueryData(["items"], (current = []) => [...current, optimisticItem]);`}</CodeLine>
      <CodeLine>{`    toast.info("Adding item...");`}</CodeLine>
      <CodeLine>{`    return { previous };`}</CodeLine>
      <CodeLine>{`  },`}</CodeLine>
      <CodeLine>{`  onError: (_error, _body, context) => {`}</CodeLine>
      <CodeLine>{`    queryClient.setQueryData(["items"], context?.previous);`}</CodeLine>
      <CodeLine>{`    toast.error("Failed to add item");`}</CodeLine>
      <CodeLine>{`  },`}</CodeLine>
      <CodeLine>{`  onSuccess: (saved) => {`}</CodeLine>
      <CodeLine>{`    queryClient.setQueryData(["items"], (current = []) => current.map((item) => item.id === saved.id ? saved : item));`}</CodeLine>
      <CodeLine>{`    toast.success("Item added successfully");`}</CodeLine>
      <CodeLine>{`  },`}</CodeLine>
      <CodeLine>{`});`}</CodeLine>
    </div>
  );
}

function ForgeSample() {
  return (
    <div className="resources-code-block" role="presentation">
      <CodeLine><Tok kind="comment">// packingCatalog is declared once with response lenses and branchNative effects.</Tok></CodeLine>
      <CodeLine />
      <CodeLine><Tok kind="kw">const</Tok> <Tok kind="var">catalog</Tok> <Tok kind="op">=</Tok> <Tok kind="fn">useResourceCatalog</Tok>(<Tok kind="var">packingCatalog</Tok>, <Tok kind="var">store</Tok>);</CodeLine>
      <CodeLine><Tok kind="kw">const</Tok> <Tok kind="var">list</Tok> <Tok kind="op">=</Tok> <Tok kind="fn">useResourceLine</Tok>(<Tok kind="var">catalog</Tok>.<Tok kind="prop">items</Tok>, {`{}`}, <Tok kind="var">store</Tok>);</CodeLine>
      <CodeLine />
      <CodeLine><Tok kind="kw">const</Tok> <Tok kind="var">write</Tok> <Tok kind="op">=</Tok> <Tok kind="fn">useManagedResourceWrite</Tok>({`{`}</CodeLine>
      <CodeLine>{`  line: (body) => catalog.confirmItem.line({ itemId: body.id, body }),`}</CodeLine>
      <CodeLine>{`  feedback: { success: "Item added successfully", error: "Failed to add item" },`}</CodeLine>
      <CodeLine>{`  onFeedback: (feedback) => toastBridge.consume(feedback),`}</CodeLine>
      <CodeLine>{`});`}</CodeLine>
      <CodeLine />
      <CodeLine><Tok kind="var">list</Tok>.<Tok kind="prop">line</Tok>.<Tok kind="fn">patch</Tok>(<Tok kind="var">catalog</Tok>.<Tok kind="prop">items</Tok>.<Tok kind="prop">patch</Tok>.<Tok kind="fn">insert</Tok>({`{ itemId, placement: "append", nextItem }`}));</CodeLine>
      <CodeLine><Tok kind="kw">await</Tok> <Tok kind="var">write</Tok>.<Tok kind="fn">execute</Tok>({`{ id: itemId, outcome: "confirm" }`});</CodeLine>
      <CodeLine><Tok kind="fn">console</Tok>.<Tok kind="fn">log</Tok>(<Tok kind="var">list</Tok>.<Tok kind="prop">diagnosticsSummary</Tok>.<Tok kind="prop">latest</Tok>);</CodeLine>
      <CodeLine><Tok kind="fn">console</Tok>.<Tok kind="fn">log</Tok>(<Tok kind="var">write</Tok>.<Tok kind="prop">lastResult</Tok>?.<Tok kind="prop">recovery</Tok>.<Tok kind="fn">summary</Tok>());</CodeLine>
    </div>
  );
}

export function ResourcesSectionCodeSample() {
  return (
    <div className="resources-code-grid">
      <article className="resources-code-card">
        <div className="forms-card-topline"><span>TanStack authoring</span></div>
        <h3>Optimistic insert, rollback snapshot, and toast wiring live in feature code.</h3>
        <TanStackSample />
      </article>
      <article className="resources-code-card">
        <div className="forms-card-topline"><span>Forge authoring</span></div>
        <h3>The runtime owns the write lifecycle, and React maps that operation view into toasts.</h3>
        <ForgeSample />
      </article>
    </div>
  );
}
