---- MODULE DurabilityAcknowledgmentMutant ----
EXTENDS DurabilityRecovery

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
AcknowledgeBeforeFence ==
    /\ ~crashed
    /\ wal = "Memory"
    /\ wal' = "Acknowledged"
    /\ walFenceCompleted' = FALSE
    /\ mutantEdge' = "AcknowledgeBeforeFence"
    /\ UNCHANGED <<page, checkpoint, directorySync, recovery, root, crashed>>
MutantNext == NormalNext \/ AcknowledgeBeforeFence
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
