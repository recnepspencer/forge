import React from "react";

function CodeLine({ children }: { children?: React.ReactNode }): React.ReactElement {
  return <div className="forms-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: React.ReactNode;
}): React.ReactElement {
  return <span className={`forms-tok forms-tok-${kind}`}>{children}</span>;
}

export function FormsSectionCodeSample(): React.ReactElement {
  return (
    <div className="forms-code-block" role="presentation">
      <CodeLine>
        <Tok kind="kw">function</Tok> <Tok kind="fn">validateShippingRegions</Tok>(
        <Tok kind="var">regions</Tok>, <Tok kind="var">approvalLine</Tok>) <Tok kind="plain">{"{"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">requiredRegions</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="var">approvalLine</Tok>.<Tok kind="prop">value</Tok>.<Tok kind="fn">filter</Tok>((<Tok kind="var">region</Tok>) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="var">region</Tok>.<Tok kind="prop">approved</Tok>);
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">missingApproved</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="var">requiredRegions</Tok>.<Tok kind="fn">filter</Tok>((<Tok kind="var">region</Tok>) <Tok kind="op">=&gt;</Tok>{" "}
        !<Tok kind="var">regions</Tok>.<Tok kind="fn">includes</Tok>(<Tok kind="var">region</Tok>.<Tok kind="prop">code</Tok>));
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">if</Tok> (<Tok kind="var">missingApproved</Tok>.<Tok kind="prop">length</Tok> <Tok kind="op">&gt;</Tok> <Tok kind="plain">0</Tok>) <Tok kind="plain">{"{"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="kw">return</Tok> <Tok kind="str">"Every approved shipping region must stay selected"</Tok>;
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"}"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">return</Tok> <Tok kind="var">regions</Tok>.<Tok kind="fn">every</Tok>((<Tok kind="var">code</Tok>) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="var">approvalLine</Tok>.<Tok kind="prop">value</Tok>[<Tok kind="var">code</Tok>]?.<Tok kind="prop">approved</Tok>)
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="op">?</Tok> <Tok kind="plain">null</Tok> <Tok kind="op">:</Tok> <Tok kind="str">"One selected country is still awaiting approval"</Tok>;
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"}"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">function</Tok> <Tok kind="fn">validateCarrierEmails</Tok>(
        <Tok kind="var">regions</Tok>, <Tok kind="var">emails</Tok>) <Tok kind="plain">{"{"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">return</Tok> <Tok kind="var">regions</Tok>.<Tok kind="fn">every</Tok>((<Tok kind="var">code</Tok>) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">isValidEmail</Tok>(<Tok kind="var">emails</Tok>[<Tok kind="var">code</Tok>]))
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="op">?</Tok> <Tok kind="plain">null</Tok> <Tok kind="op">:</Tok> <Tok kind="str">"Every selected country needs a carrier email"</Tok>;
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"}"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">export function</Tok> <Tok kind="fn">RolloutSettingsForm</Tok>() <Tok kind="plain">{"{"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">rolloutSource</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceLine</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">rollout</Tok>());
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">countryApproval</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceLine</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">countryApproval</Tok>());
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">saveRollout</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceOperation</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">saveRollout</Tok>());
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="plain">{`// The form gets both saved source data and approval policy from resource lines.`}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">form</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useSignalsForm</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">source</Tok>: <Tok kind="var">rolloutSource</Tok>.<Tok kind="prop">value</Tok> <Tok kind="op">??</Tok>{" "}
        <Tok kind="var">EMPTY_ROLLOUT</Tok>,
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"} <Tok kind="prop">countryApproval</Tok>: <Tok kind="var">countryApproval</Tok> {"}"},
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">fields</Tok>: ({ "{" } <Tok kind="var">field</Tok> { "}" }) <Tok kind="op">=&gt;</Tok> ({"{"}
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">price</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"price"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: ({ "{" } <Tok kind="var">value</Tok>, <Tok kind="var">draft</Tok> { "}" }) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">validatePrice</Tok>(<Tok kind="var">value</Tok>, <Tok kind="var">draft</Tok>.<Tok kind="prop">baseCost</Tok>, <Tok kind="var">draft</Tok>.<Tok kind="prop">targetMargin</Tok>),
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">targetMargin</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"targetMargin"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">readOnly</Tok>: <Tok kind="plain">true</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">shippingRegions</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"shippingRegions"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: ({ "{" } <Tok kind="var">value</Tok>, <Tok kind="var">resources</Tok> { "}" }) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">validateShippingRegions</Tok>(<Tok kind="var">value</Tok>, <Tok kind="var">resources</Tok>.<Tok kind="prop">countryApproval</Tok>),
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">carrierEmails</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"carrierEmails"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: ({ "{" } <Tok kind="var">value</Tok>, <Tok kind="var">draft</Tok> { "}" }) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">validateCarrierEmails</Tok>(<Tok kind="var">draft</Tok>.<Tok kind="prop">shippingRegions</Tok>, <Tok kind="var">value</Tok>),
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">actions</Tok>: ({ "{" } <Tok kind="var">resourceAction</Tok> { "}" }) <Tok kind="op">=&gt;</Tok> ({"{"}
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">save</Tok>: <Tok kind="var">resourceAction</Tok>(<Tok kind="str">"save"</Tok>, <Tok kind="var">saveRollout</Tok>),
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="kw">return</Tok> (
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">form</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Retail price"</Tok> <Tok kind="plain">{`{...form.field("price")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Target margin"</Tok> <Tok kind="plain">{`{...form.field("targetMargin")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">MultiSelectDropdown</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Shipping regions"</Tok> <Tok kind="plain">{`{...form.field("shippingRegions")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{form.draft.shippingRegions.map((code) => (`}</Tok>
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="prop">key</Tok>=<Tok kind="plain">{`{code}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="prop">label</Tok>=<Tok kind="plain">{`{\`${"${code}"} carrier email\`}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="plain">{`{...form.field("carrierEmails").item(code)}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`))}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{/* reset disables when the form is already back at source */}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">button</Tok> <Tok kind="prop">type</Tok>=<Tok kind="str">"button"</Tok> <Tok kind="prop">disabled</Tok>=<Tok kind="plain">{`{!form.dirty().isDirty}`}</Tok> <Tok kind="prop">onClick</Tok>=<Tok kind="plain">{`{form.reset}`}</Tok><Tok kind="plain">&gt;</Tok>Reset<Tok kind="plain">&lt;/</Tok><Tok kind="fn">button</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{/* save auto-disables until price, approvals, and carrier emails all pass */}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">button</Tok> <Tok kind="prop">disabled</Tok>=<Tok kind="plain">{`{form.action("save").disabled}`}</Tok><Tok kind="plain">&gt;</Tok>Save changes<Tok kind="plain">&lt;/</Tok><Tok kind="fn">button</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">&lt;/</Tok><Tok kind="fn">form</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{" );"}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"}"}</Tok>
      </CodeLine>
    </div>
  );
}
