---- MODULE QuarantineReadmission ----
EXTENDS Naturals

VARIABLES state, scopePreserved, verification, authority, observationOnly, operatorIntent
vars == <<state, scopePreserved, verification, authority, observationOnly, operatorIntent>>

Init == /\ state = "Proposed" /\ scopePreserved = TRUE /\ verification = FALSE /\ authority = FALSE /\ observationOnly = FALSE /\ operatorIntent = FALSE
Seal == /\ state = "Proposed" /\ state' = "Sealed" /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>
ObserveOffline == /\ state = "Sealed" /\ observationOnly' = TRUE /\ UNCHANGED <<state, scopePreserved, verification, authority, operatorIntent>>
RequestOperatorRepair == /\ state = "Sealed" /\ operatorIntent' = TRUE /\ UNCHANGED <<state, scopePreserved, verification, authority, observationOnly>>
BeginVerification == /\ state = "Sealed" /\ state' = "VerificationPending" /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>
CompleteVerification == /\ state = "VerificationPending" /\ verification' = TRUE /\ UNCHANGED <<state, scopePreserved, authority, observationOnly, operatorIntent>>
AdmitAuthority == /\ state = "VerificationPending" /\ authority' = TRUE /\ UNCHANGED <<state, scopePreserved, verification, observationOnly, operatorIntent>>
Readmit == /\ state = "VerificationPending" /\ scopePreserved /\ verification /\ authority /\ state' = "Readmitted" /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>
RetainAudit == /\ state = "Sealed" /\ state' = "RetainedAudit" /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>
ObserveProposed == /\ state = "Proposed" /\ UNCHANGED vars
DenyReadmission == /\ state \in {"Sealed", "VerificationPending"} /\ state' = "Denied" /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>

Next == Seal \/ ObserveOffline \/ RequestOperatorRepair \/ BeginVerification \/ CompleteVerification \/ AdmitAuthority \/ Readmit \/ RetainAudit \/ ObserveProposed \/ DenyReadmission
Spec == Init /\ [][Next]_vars
TypeOK == /\ state \in {"Proposed", "Sealed", "VerificationPending", "Readmitted", "RetainedAudit", "Denied"} /\ scopePreserved \in BOOLEAN /\ verification \in BOOLEAN /\ authority \in BOOLEAN /\ observationOnly \in BOOLEAN /\ operatorIntent \in BOOLEAN
ReadmissionNeedsVerification == state = "Readmitted" => verification
ReadmissionNeedsAuthority == state = "Readmitted" => authority
ReadmissionPreservesScope == state = "Readmitted" => scopePreserved
ObservationCannotRepair == observationOnly /\ ~verification /\ ~authority => state # "Readmitted"
OperatorCannotRepair == operatorIntent /\ ~verification /\ ~authority => state # "Readmitted"
====
