---- MODULE CompactionPublicationMutant ----
EXTENDS CompactionVisibility

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
PublishBeforeCutover ==
    /\ lifecycle = "Durable"
    /\ lifecycle' = "Visible"
    /\ visibleGeneration' = visibleGeneration + 1
    /\ mutantEdge' = "PublishBeforeCutover"
    /\ UNCHANGED <<publicationPrepared, oldRetained, tombstonePreserved, readers>>
MutantNext == NormalNext \/ PublishBeforeCutover
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
