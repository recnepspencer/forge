---- MODULE QuarantineReleaseMutant ----
EXTENDS QuarantineReadmission

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
ReleaseWithoutVerification ==
    /\ state = "Sealed"
    /\ state' = "Readmitted"
    /\ mutantEdge' = "ReleaseWithoutVerification"
    /\ UNCHANGED <<scopePreserved, verification, authority, observationOnly, operatorIntent>>
MutantNext == NormalNext \/ ReleaseWithoutVerification
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
