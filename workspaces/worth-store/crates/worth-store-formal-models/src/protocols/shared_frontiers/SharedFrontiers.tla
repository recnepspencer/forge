---- MODULE SharedFrontiers ----
EXTENDS Naturals

VARIABLES durability, visibility, reachability, quarantine, admission,
          recoveryPrecedence, verificationAdvanced, oldAuthorityReachable,
          crashed, externalPublished, lastAction

vars == <<durability, visibility, reachability, quarantine, admission,
          recoveryPrecedence, verificationAdvanced, oldAuthorityReachable,
          crashed, externalPublished, lastAction>>

DurabilityStates == {"Pending", "Admitted"}
VisibilityStates == {"Stable", "CompactionCutover", "Reopened"}
ReachabilityStates == {"Reachable", "LiveLease", "ReleaseEligible", "Reused"}
QuarantineStates == {"Clear", "Sealed", "VerificationPending", "Released"}
AdmissionStates == {"None", "ImportPending", "ReplicationPending",
                    "ExternalDurable", "Divergence", "Published"}
Actions == {
  "DurabilityAdmitted", "RecoveryPrecedencePreserved", "LiveLeaseAcquired",
  "LeaseReleased", "CompactionCutover", "Crash", "Reopen",
  "QuarantineSealed", "QuarantineVerificationStarted", "QuarantineReadmitted",
  "ReclaimDeferred", "ReclaimReleased", "GenerationReused",
  "CheckpointPublicationRequested", "ImportAdmissionPending",
  "ReplicationAdmissionPending", "ExternalDurabilityAdmitted",
  "ExternalPublicationRequested", "ReplicationDivergenceDetected"
}

Init == /\ durability = "Pending"
        /\ visibility = "Stable"
        /\ reachability = "Reachable"
        /\ quarantine = "Clear"
        /\ admission = "None"
        /\ recoveryPrecedence = FALSE
        /\ verificationAdvanced = FALSE
        /\ oldAuthorityReachable = TRUE
        /\ crashed = FALSE
        /\ externalPublished = FALSE
        /\ lastAction = "ReclaimDeferred"

DurabilityAdmitted ==
  /\ durability' = "Admitted"
  /\ lastAction' = "DurabilityAdmitted"
  /\ UNCHANGED <<visibility, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

RecoveryPrecedencePreserved ==
  /\ recoveryPrecedence' = TRUE
  /\ lastAction' = "RecoveryPrecedencePreserved"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine, admission,
                 verificationAdvanced, oldAuthorityReachable, crashed,
                 externalPublished>>

LiveLeaseAcquired ==
  /\ reachability' = "LiveLease"
  /\ oldAuthorityReachable' = TRUE
  /\ lastAction' = "LiveLeaseAcquired"
  /\ UNCHANGED <<durability, visibility, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced, crashed,
                 externalPublished>>

LeaseReleased ==
  /\ reachability = "LiveLease"
  /\ reachability' = "Reachable"
  /\ lastAction' = "LeaseReleased"
  /\ UNCHANGED <<durability, visibility, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

CompactionCutover ==
  /\ recoveryPrecedence
  /\ visibility' = "CompactionCutover"
  /\ lastAction' = "CompactionCutover"
  /\ UNCHANGED <<durability, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

Crash ==
  /\ crashed' = TRUE
  /\ lastAction' = "Crash"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, externalPublished>>

Reopen ==
  /\ crashed
  /\ ((visibility = "CompactionCutover") => recoveryPrecedence)
  /\ visibility' = IF visibility = "CompactionCutover" THEN "Reopened" ELSE visibility
  /\ crashed' = FALSE
  /\ lastAction' = "Reopen"
  /\ UNCHANGED <<durability, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, externalPublished>>

QuarantineSealed ==
  /\ quarantine' = "Sealed"
  /\ verificationAdvanced' = FALSE
  /\ externalPublished' = FALSE
  /\ lastAction' = "QuarantineSealed"
  /\ UNCHANGED <<durability, visibility, reachability, admission,
                 recoveryPrecedence, oldAuthorityReachable, crashed>>

QuarantineVerificationStarted ==
  /\ quarantine = "Sealed"
  /\ quarantine' = "VerificationPending"
  /\ lastAction' = "QuarantineVerificationStarted"
  /\ UNCHANGED <<durability, visibility, reachability, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

QuarantineReadmitted ==
  /\ quarantine = "VerificationPending"
  /\ quarantine' = "Released"
  /\ verificationAdvanced' = TRUE
  /\ lastAction' = "QuarantineReadmitted"
  /\ UNCHANGED <<durability, visibility, reachability, admission,
                 recoveryPrecedence, oldAuthorityReachable, crashed,
                 externalPublished>>

ReclaimDeferred ==
  /\ lastAction' = "ReclaimDeferred"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

ReclaimReleased ==
  /\ reachability = "Reachable"
  /\ quarantine \in {"Clear", "Released"}
  /\ reachability' = "ReleaseEligible"
  /\ oldAuthorityReachable' = FALSE
  /\ lastAction' = "ReclaimReleased"
  /\ UNCHANGED <<durability, visibility, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced, crashed,
                 externalPublished>>

GenerationReused ==
  /\ reachability = "ReleaseEligible"
  /\ ~oldAuthorityReachable
  /\ quarantine \in {"Clear", "Released"}
  /\ reachability' = "Reused"
  /\ lastAction' = "GenerationReused"
  /\ UNCHANGED <<durability, visibility, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

CheckpointPublicationRequested ==
  /\ ~crashed
  /\ durability = "Admitted"
  /\ quarantine \in {"Clear", "Released"}
  /\ lastAction' = "CheckpointPublicationRequested"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine, admission,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

ImportAdmissionPending ==
  /\ admission' = "ImportPending"
  /\ externalPublished' = FALSE
  /\ lastAction' = "ImportAdmissionPending"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed>>

ReplicationAdmissionPending ==
  /\ admission' = "ReplicationPending"
  /\ externalPublished' = FALSE
  /\ lastAction' = "ReplicationAdmissionPending"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed>>

ExternalDurabilityAdmitted ==
  /\ admission \in {"ImportPending", "ReplicationPending"}
  /\ durability' = "Admitted"
  /\ admission' = "ExternalDurable"
  /\ lastAction' = "ExternalDurabilityAdmitted"
  /\ UNCHANGED <<visibility, reachability, quarantine,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed, externalPublished>>

ExternalPublicationRequested ==
  /\ ~crashed
  /\ durability = "Admitted"
  /\ admission = "ExternalDurable"
  /\ quarantine \in {"Clear", "Released"}
  /\ admission' = "Published"
  /\ externalPublished' = TRUE
  /\ lastAction' = "ExternalPublicationRequested"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed>>

ReplicationDivergenceDetected ==
  /\ admission \in {"ReplicationPending", "ExternalDurable"}
  /\ admission' = "Divergence"
  /\ externalPublished' = FALSE
  /\ lastAction' = "ReplicationDivergenceDetected"
  /\ UNCHANGED <<durability, visibility, reachability, quarantine,
                 recoveryPrecedence, verificationAdvanced,
                 oldAuthorityReachable, crashed>>

Next == DurabilityAdmitted \/ RecoveryPrecedencePreserved
        \/ LiveLeaseAcquired \/ LeaseReleased \/ CompactionCutover
        \/ Crash \/ Reopen \/ QuarantineSealed
        \/ QuarantineVerificationStarted \/ QuarantineReadmitted
        \/ ReclaimDeferred \/ ReclaimReleased \/ GenerationReused
        \/ CheckpointPublicationRequested \/ ImportAdmissionPending
        \/ ReplicationAdmissionPending \/ ExternalDurabilityAdmitted
        \/ ExternalPublicationRequested \/ ReplicationDivergenceDetected

Spec == Init /\ [][Next]_vars

TypeOK == /\ durability \in DurabilityStates
          /\ visibility \in VisibilityStates
          /\ reachability \in ReachabilityStates
          /\ quarantine \in QuarantineStates
          /\ admission \in AdmissionStates
          /\ recoveryPrecedence \in BOOLEAN
          /\ verificationAdvanced \in BOOLEAN
          /\ oldAuthorityReachable \in BOOLEAN
          /\ crashed \in BOOLEAN
          /\ externalPublished \in BOOLEAN
          /\ lastAction \in Actions

NoReclaimOfReachableAuthority ==
  reachability = "Reused" => ~oldAuthorityReachable

NoQuarantinedCurrentPublication ==
  externalPublished => quarantine \in {"Clear", "Released"}

ExternalPublicationRequiresDurability ==
  externalPublished => durability = "Admitted" /\ admission = "Published"

QuarantineReleaseRequiresVerification ==
  quarantine = "Released" => verificationAdvanced

CompactionCutoverPreservesRecovery ==
  visibility \in {"CompactionCutover", "Reopened"} => recoveryPrecedence

====
