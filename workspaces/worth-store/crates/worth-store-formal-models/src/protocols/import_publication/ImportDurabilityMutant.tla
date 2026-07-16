---- MODULE ImportDurabilityMutant ----
EXTENDS ImportPublication

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
PublishWithoutDurability ==
    /\ state = Materialized
    /\ state' = Durable
    /\ mutantEdge' = "PublishWithoutDurability"
    /\ UNCHANGED durabilityAdmitted
MutantNext == NormalNext \/ PublishWithoutDurability
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
