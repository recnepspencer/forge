---- MODULE ReplicationAdmission ----
EXTENDS Integers, FiniteSets

CONSTANT Frontiers

VARIABLES state, currentFrontier, candidateFrontier, epochAligned,
          lineageAligned, durabilityAdmitted, delivery, lastAction,
          candidatePublished

vars == <<state, currentFrontier, candidateFrontier, epochAligned,
          lineageAligned, durabilityAdmitted, delivery, lastAction,
          candidatePublished>>

States == {"Raw", "SourceAdmitted", "SourceDenied", "ProgressObserved",
           "DuplicateObserved", "ResumeDenied", "DivergenceDetected",
           "PublicationPending", "PublicationDurable", "PublicationDenied"}
Deliveries == {"None", "Fresh", "Resumed"}
Actions == {
  "SourceAdmitted", "SourcePeerIdentityDenied", "SourceEpochRequiredDenied",
  "SourceLineageIdentityDenied", "SourceCurrentAuthorityDenied",
  "SourceReplayIdentityDenied", "FreshProgressObserved",
  "ResumeProgressObserved", "DuplicateObserved",
  "ResumeCurrentAuthorityDenied",
  "SourceEpochDivergenceDetected", "LineageDivergenceDetected",
  "ReplayOverlapDivergenceDetected", "ResumeProgressGapDenied",
  "FreshPublicationPending", "ResumePublicationPending",
  "FreshPublicationDurable", "ResumePublicationDurable",
  "PublicationCurrentAuthorityDenied", "PublicationPeerProgressChangedDenied",
  "PublicationPeerCapacityDenied", "PublicationProgressStoreDenied"
}

Init == /\ state = "Raw"
        /\ currentFrontier = 0
        /\ candidateFrontier = 0
        /\ epochAligned = TRUE
        /\ lineageAligned = TRUE
        /\ durabilityAdmitted = FALSE
        /\ delivery = "None"
        /\ lastAction = "SourceReplayIdentityDenied"
        /\ candidatePublished = FALSE

CanStartSource == state \in {"Raw", "SourceDenied", "DuplicateObserved",
                             "ResumeDenied", "DivergenceDetected",
                             "PublicationDurable", "PublicationDenied"}

SourceAdmitted ==
  /\ CanStartSource
  /\ \E next \in Frontiers, epochOk \in BOOLEAN, lineageOk \in BOOLEAN:
       /\ state' = "SourceAdmitted"
       /\ candidateFrontier' = next
       /\ epochAligned' = epochOk
       /\ lineageAligned' = lineageOk
  /\ durabilityAdmitted' = TRUE
  /\ delivery' = "None"
  /\ lastAction' = "SourceAdmitted"
  /\ candidatePublished' = FALSE
  /\ UNCHANGED currentFrontier

SourceDenied(action) ==
  /\ CanStartSource
  /\ action \in {"SourcePeerIdentityDenied", "SourceEpochRequiredDenied",
                  "SourceLineageIdentityDenied", "SourceCurrentAuthorityDenied",
                  "SourceReplayIdentityDenied"}
  /\ state' = "SourceDenied"
  /\ lastAction' = action
  /\ durabilityAdmitted' = FALSE
  /\ delivery' = "None"
  /\ candidatePublished' = FALSE
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned, lineageAligned>>

FreshProgressObserved ==
  /\ state = "SourceAdmitted"
  /\ currentFrontier = 0
  /\ candidateFrontier > 0
  /\ epochAligned /\ lineageAligned
  /\ state' = "ProgressObserved"
  /\ delivery' = "Fresh"
  /\ lastAction' = "FreshProgressObserved"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

ResumeProgressObserved ==
  /\ state = "SourceAdmitted"
  /\ currentFrontier > 0
  /\ candidateFrontier = currentFrontier + 1
  /\ epochAligned /\ lineageAligned
  /\ state' = "ProgressObserved"
  /\ delivery' = "Resumed"
  /\ lastAction' = "ResumeProgressObserved"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

DuplicateObserved ==
  /\ state = "SourceAdmitted"
  /\ candidateFrontier = currentFrontier
  /\ epochAligned /\ lineageAligned
  /\ state' = "DuplicateObserved"
  /\ delivery' = "None"
  /\ lastAction' = "DuplicateObserved"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

ResumeCurrentAuthorityDenied ==
  /\ state = "SourceAdmitted"
  /\ state' = "ResumeDenied"
  /\ delivery' = "None"
  /\ lastAction' = "ResumeCurrentAuthorityDenied"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

SourceEpochDivergenceDetected ==
  /\ state = "SourceAdmitted"
  /\ ~epochAligned
  /\ state' = "DivergenceDetected"
  /\ delivery' = "None"
  /\ lastAction' = "SourceEpochDivergenceDetected"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

LineageDivergenceDetected ==
  /\ state = "SourceAdmitted"
  /\ ~lineageAligned
  /\ state' = "DivergenceDetected"
  /\ delivery' = "None"
  /\ lastAction' = "LineageDivergenceDetected"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

ReplayOverlapDivergenceDetected ==
  /\ state = "SourceAdmitted"
  /\ candidateFrontier < currentFrontier
  /\ state' = "DivergenceDetected"
  /\ delivery' = "None"
  /\ lastAction' = "ReplayOverlapDivergenceDetected"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

ResumeProgressGapDenied ==
  /\ state = "SourceAdmitted"
  /\ currentFrontier > 0
  /\ candidateFrontier > currentFrontier + 1
  /\ state' = "ResumeDenied"
  /\ delivery' = "None"
  /\ lastAction' = "ResumeProgressGapDenied"
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, candidatePublished>>

PublicationPending(kind, action) ==
  /\ state = "ProgressObserved"
  /\ delivery = kind
  /\ action \in {"FreshPublicationPending", "ResumePublicationPending"}
  /\ (kind = "Fresh") = (action = "FreshPublicationPending")
  /\ state' = "PublicationPending"
  /\ lastAction' = action
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, delivery, candidatePublished>>

PublicationDurable(kind, action) ==
  /\ state = "PublicationPending"
  /\ delivery = kind
  /\ durabilityAdmitted /\ epochAligned /\ lineageAligned
  /\ candidateFrontier > currentFrontier
  /\ action \in {"FreshPublicationDurable", "ResumePublicationDurable"}
  /\ (kind = "Fresh") = (action = "FreshPublicationDurable")
  /\ state' = "PublicationDurable"
  /\ currentFrontier' = candidateFrontier
  /\ lastAction' = action
  /\ candidatePublished' = TRUE
  /\ UNCHANGED <<candidateFrontier, epochAligned, lineageAligned,
                 durabilityAdmitted, delivery>>

PublicationCurrentAuthorityDenied ==
  /\ state = "PublicationPending"
  /\ state' = "PublicationDenied"
  /\ lastAction' = "PublicationCurrentAuthorityDenied"
  /\ candidatePublished' = FALSE
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, delivery>>

PublicationPeerProgressChangedDenied ==
  /\ state = "PublicationPending"
  /\ state' = "PublicationDenied"
  /\ lastAction' = "PublicationPeerProgressChangedDenied"
  /\ candidatePublished' = FALSE
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, delivery>>

PublicationStorageDenied(action) ==
  /\ action \in {"PublicationPeerCapacityDenied", "PublicationProgressStoreDenied"}
  /\ state = "PublicationPending"
  /\ state' = "PublicationDenied"
  /\ lastAction' = action
  /\ candidatePublished' = FALSE
  /\ UNCHANGED <<currentFrontier, candidateFrontier, epochAligned,
                 lineageAligned, durabilityAdmitted, delivery>>

Next == SourceAdmitted
        \/ (\E action \in Actions: SourceDenied(action))
        \/ FreshProgressObserved \/ ResumeProgressObserved \/ DuplicateObserved
        \/ ResumeCurrentAuthorityDenied
        \/ SourceEpochDivergenceDetected \/ LineageDivergenceDetected
        \/ ReplayOverlapDivergenceDetected \/ ResumeProgressGapDenied
        \/ (\E kind \in {"Fresh", "Resumed"}, action \in Actions:
             PublicationPending(kind, action))
        \/ (\E kind \in {"Fresh", "Resumed"}, action \in Actions:
             PublicationDurable(kind, action))
        \/ PublicationCurrentAuthorityDenied
        \/ PublicationPeerProgressChangedDenied
        \/ (\E action \in Actions: PublicationStorageDenied(action))

Spec == Init /\ [][Next]_vars

TypeOK == /\ state \in States
          /\ currentFrontier \in Frontiers
          /\ candidateFrontier \in Frontiers
          /\ epochAligned \in BOOLEAN
          /\ lineageAligned \in BOOLEAN
          /\ durabilityAdmitted \in BOOLEAN
          /\ delivery \in Deliveries
          /\ lastAction \in Actions
          /\ candidatePublished \in BOOLEAN

SourceAdmissionRequiresDurability == state = "SourceAdmitted" => durabilityAdmitted
DuplicateDoesNotAdvance == state = "DuplicateObserved" => candidateFrontier = currentFrontier
PendingDoesNotAdvance == state = "PublicationPending" => ~candidatePublished
DivergenceCannotPublish == state = "DivergenceDetected" => ~candidatePublished
DurablePublicationIsExact ==
  state = "PublicationDurable" =>
    candidatePublished /\ currentFrontier = candidateFrontier
    /\ epochAligned /\ lineageAligned /\ durabilityAdmitted

====
