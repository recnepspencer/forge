---- MODULE ImportPublication ----
EXTENDS Naturals

CONSTANTS Raw, Readmitted, ArtifactAdmitted, Materialized, Pending, Durable, Denied

VARIABLES state, durabilityAdmitted

vars == <<state, durabilityAdmitted>>

Init == /\ state = Raw /\ durabilityAdmitted = FALSE
ObserveRaw == /\ state = Raw /\ UNCHANGED vars

Readmit == /\ state = Raw
           /\ state' = Readmitted
           /\ UNCHANGED durabilityAdmitted

AdmitArtifact == /\ state = Readmitted
                 /\ state' = ArtifactAdmitted
                 /\ UNCHANGED durabilityAdmitted

Materialize == /\ state = ArtifactAdmitted
               /\ state' = Materialized
               /\ UNCHANGED durabilityAdmitted

Ready == /\ state = Materialized
         /\ state' = Pending
         /\ durabilityAdmitted' = TRUE

Publish == /\ state = Pending
           /\ durabilityAdmitted
           /\ state' = Durable
           /\ UNCHANGED durabilityAdmitted

RejectPublication == /\ state = Pending
                     /\ state' = Denied
                     /\ UNCHANGED durabilityAdmitted

CrashBeforePublication == /\ state = Pending
                          /\ state' = Materialized
                          /\ durabilityAdmitted' = FALSE

Next == ObserveRaw \/ Readmit \/ AdmitArtifact \/ Materialize \/ Ready \/ Publish
        \/ RejectPublication \/ CrashBeforePublication

Spec == Init /\ [][Next]_vars

TypeOK == /\ state \in {Raw, Readmitted, ArtifactAdmitted, Materialized, Pending, Durable, Denied}
          /\ durabilityAdmitted \in BOOLEAN
RawIsNotCurrent == state = Raw => state # Durable
PendingIsNotDurable == state = Pending => state # Durable
DurablePublicationWasAdmitted == state = Durable => durabilityAdmitted

====
