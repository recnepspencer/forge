#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaRecoveryFrontierDenial {
    InvalidOrdering,
    ZeroAuthorityEpoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaRecoveryFrontier {
    observed_lsn: u64,
    durable_lsn: u64,
    client_acknowledged_lsn: u64,
    replication_acknowledged_lsn: u64,
    authority_epoch: u64,
}

impl ReplicaRecoveryFrontier {
    pub const fn admit(
        observed_lsn: u64,
        durable_lsn: u64,
        client_acknowledged_lsn: u64,
        replication_acknowledged_lsn: u64,
        authority_epoch: u64,
    ) -> Result<Self, ReplicaRecoveryFrontierDenial> {
        if authority_epoch == 0 {
            return Err(ReplicaRecoveryFrontierDenial::ZeroAuthorityEpoch);
        }
        if replication_acknowledged_lsn > client_acknowledged_lsn
            || client_acknowledged_lsn > durable_lsn
            || durable_lsn > observed_lsn
        {
            return Err(ReplicaRecoveryFrontierDenial::InvalidOrdering);
        }
        Ok(Self {
            observed_lsn,
            durable_lsn,
            client_acknowledged_lsn,
            replication_acknowledged_lsn,
            authority_epoch,
        })
    }

    pub const fn observed_lsn(self) -> u64 {
        self.observed_lsn
    }

    pub const fn durable_lsn(self) -> u64 {
        self.durable_lsn
    }

    pub const fn client_acknowledged_lsn(self) -> u64 {
        self.client_acknowledged_lsn
    }

    pub const fn replication_acknowledged_lsn(self) -> u64 {
        self.replication_acknowledged_lsn
    }

    pub const fn authority_epoch(self) -> u64 {
        self.authority_epoch
    }

    pub const fn acknowledged_data_loss_from(self, current: Self) -> u64 {
        current
            .client_acknowledged_lsn
            .saturating_sub(self.client_acknowledged_lsn)
    }
}
