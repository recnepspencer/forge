import { DemoPreface } from "./DemoPreface";
import "./compositionSection.css";

interface CompositionSectionProps {
  onNavigate: (path: string) => void;
}

function CodeLine({ children }: { children?: React.ReactNode }) {
  return <div className="composition-code-line">{children}</div>;
}

function GlueCode() {
  return (
    <div className="composition-code-block">
      <CodeLine>{`function useEditProductAdapter(productId: string) {`}</CodeLine>
      <CodeLine>{`  const queryClient = useQueryClient();`}</CodeLine>
      <CodeLine>{`  const product = useQuery(productQuery(productId));`}</CodeLine>
      <CodeLine>{`  const mutation = useMutation({ mutationFn: saveProduct });`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  const form = useFormik({`}</CodeLine>
      <CodeLine>{`    initialValues: product.data ?? emptyProduct,`}</CodeLine>
      <CodeLine>{`    enableReinitialize: true,`}</CodeLine>
      <CodeLine>{`    onSubmit(values) { mutation.mutate(values); },`}</CodeLine>
      <CodeLine>{`  });`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  // Glue: invent one status vocabulary across query + form + mutation.`}</CodeLine>
      <CodeLine>{`  const status = mutation.isPending ? "saving"`}</CodeLine>
      <CodeLine>{`    : product.isLoading ? "loading"`}</CodeLine>
      <CodeLine>{`    : form.dirty ? "dirty" : "ready";`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  // Glue: route-leave policy has to remember form + write state.`}</CodeLine>
      <CodeLine>{`  const canLeave = !form.dirty && !mutation.isPending;`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  async function saveAndClose() {`}</CodeLine>
      <CodeLine>{`    const saved = await mutation.mutateAsync(form.values);`}</CodeLine>
      <CodeLine>{`    // Glue: server truth must be copied into every affected cache.`}</CodeLine>
      <CodeLine>{`    queryClient.setQueryData(productQuery(productId).queryKey, saved);`}</CodeLine>
      <CodeLine>{`    queryClient.invalidateQueries({ queryKey: ["products"] });`}</CodeLine>
      <CodeLine>{`    toast.success("Saved");`}</CodeLine>
      <CodeLine>{`  }`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  return { form, status, canLeave, saveAndClose };`}</CodeLine>
      <CodeLine>{`}`}</CodeLine>
    </div>
  );
}

function ForgeCode() {
  return (
    <div className="composition-code-block">
      <CodeLine>{`function useEditProduct(productId: string) {`}</CodeLine>
      <CodeLine>{`  const catalog = useResourceCatalog(productCatalog, store);`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  const product = useResourceLine(catalog.productDetail, { productId }, store);`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  const form = useSignalsForm(`}</CodeLine>
      <CodeLine>{`    catalog.editProductForm(product.line),`}</CodeLine>
      <CodeLine>{`    store,`}</CodeLine>
      <CodeLine>{`  );`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  const save = useManagedResourceWrite({`}</CodeLine>
      <CodeLine>{`    line: (body) => catalog.saveProduct.line({ productId, body }),`}</CodeLine>
      <CodeLine>{`    feedback: productFeedback,`}</CodeLine>
      <CodeLine>{`    onFeedback: toastBridge.consume,`}</CodeLine>
      <CodeLine>{`  });`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  // No adapter status: product.summary, form.summary, and save.pending`}</CodeLine>
      <CodeLine>{`  // already expose runtime-owned lifecycle/readiness truth.`}</CodeLine>
      <CodeLine>{`  // No cache glue: saveProduct declares response reconciliation targets.`}</CodeLine>
      <CodeLine>{`  // No toast glue: managed write feedback maps lifecycle to UI feedback.`}</CodeLine>
      <CodeLine />
      <CodeLine>{`  return { product, form, save, canLeave: form.exit.allowed && !save.pending };`}</CodeLine>
      <CodeLine>{`}`}</CodeLine>
    </div>
  );
}

export function CompositionSection({ onNavigate }: CompositionSectionProps) {
  return (
    <div className="xai-section-band accent-composition composition-section">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">06 / Composition</span>
        <h2>The libraries are fine. The adapter layer is the tax.</h2>
        <p>
          TanStack and Formik can both be excellent. The costly code is the
          translation layer that keeps route identity, resource truth, form
          status, write lifecycle, cache reconciliation, and toasts aligned.
        </p>
      </div>

      <DemoPreface demoId={6} />

      <div className="composition-code-grid">
        <article className="composition-card">
          <div className="forms-card-topline"><span>React stack adapter</span></div>
          <h3>Glue you own forever</h3>
          <GlueCode />
        </article>

        <article className="composition-card composition-card-forge">
          <div className="forms-card-topline"><span>Forge runtime handoff</span></div>
          <h3>One runtime contract</h3>
          <ForgeCode />
        </article>
      </div>

      <div className="composition-glue-row">
        <span>Eliminated glue</span>
        <p>
          status adapters, route-leave adapters, cache-copy adapters,
          submit-to-mutation adapters, toast lifecycle adapters, and
          diagnostics translation.
        </p>
      </div>

      <div className="xai-section-actions">
        <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/6")} type="button">
          Open demo
        </button>
        <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs/forms/route-coupling")} type="button">
          Read docs
        </button>
      </div>
    </div>
  );
}
