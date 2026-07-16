---- MODULE ReplicationDivergenceMutant ----
EXTENDS ReplicationAdmission

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
AcceptDivergenceAsResume ==
    /\ state = "DivergenceDetected"
    /\ candidatePublished' = TRUE
    /\ mutantEdge' = "AcceptDivergenceAsResume"
    /\ UNCHANGED <<state, currentFrontier, candidateFrontier, epochAligned,
                    lineageAligned, durabilityAdmitted, delivery, lastAction>>
MutantNext == NormalNext \/ AcceptDivergenceAsResume
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
