export class FormDeclarationError extends TypeError {
  constructor(message, details = null) {
    super(message);
    this.name = "FormDeclarationError";
    this.details = details;
  }
}

