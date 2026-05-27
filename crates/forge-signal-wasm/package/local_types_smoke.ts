import { createSignals } from "./index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const dialog = signals.local.dialogState({
  identity: "invite-user-dialog",
  initialOpen: true,
});
const list = signals.local.listState<string>({
  identity: "candidate-users",
  initial: ["a", "b"],
});
const formSource = signals.local.formSource({
  identity: "invite-user-form",
  initial: {
    email: "",
  },
});
const scopedDialog = signals.scope("admin").local.dialogState({
  identity: "delete-product-dialog",
});

const dialogSignal = dialog.signal;
const listItems = list.items;
const formSignal = formSource.signal;
const formDeclaration = formSource.source;
const scopedDialogScopeId: string = scopedDialog.scopeId;

dialog.open();
dialog.close();
dialog.toggle();
list.reset();
formSource.reset();

void dialogSignal;
void listItems;
void formSignal;
void formDeclaration;
void scopedDialogScopeId;
