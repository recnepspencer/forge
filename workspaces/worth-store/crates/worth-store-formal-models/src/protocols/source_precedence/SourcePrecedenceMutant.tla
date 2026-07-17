---- MODULE SourcePrecedenceMutant ----
EXTENDS SourcePrecedence

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
SelectQuarantinedSource ==
    /\ \E c \in quarantined:
         /\ selected' = {c}
         /\ mutantEdge' = "SelectQuarantinedSource"
         /\ UNCHANGED <<discovered, admitted, advisory, rejected, quarantined, contradiction>>
MutantNext == NormalNext \/ SelectQuarantinedSource
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
