use worth_ui_host_contract::{
    UiGlyphRasterTransactionOutcome, UiHostSessionReleaseOutcome,
    UiHostSurfaceDeregistrationOutcome, UiMountedPresentationAttemptIdentity,
};

use super::{GateDPinClient, UiNativeMountedTextReleaseOutcome};

impl GateDPinClient {
    pub(super) fn finish_committed(
        &mut self,
        receipt: worth_ui_host_contract::UiGlyphRasterTransactionReceipt,
    ) -> bool {
        let mut evidence = self.evidence.borrow_mut();
        evidence.committed_pin_census.push(receipt.committed_pins());
        if self.next_world == 0 {
            evidence.first_committed = true;
        } else if self.next_world == 1 {
            evidence.second_committed = true;
        } else {
            evidence.pressure_transactions = evidence.pressure_transactions.saturating_add(1);
            evidence.evictions = evidence.evictions.saturating_add(receipt.evictions());
        }
        drop(evidence);
        if self.worlds[self.next_world].pressure {
            self.pressure_release_pending = true;
            return false;
        }
        self.next_world += 1;
        if receipt.misses() != 0 {
            self.pressure_advance_pending = true;
            return false;
        }
        self.begin_next()
    }

    pub(super) fn release_pressure_world(&mut self) -> bool {
        let world = &self.worlds[self.next_world];
        let request = worth_ui_host_contract::UiMountedTextPinReleaseRequest::from_runtime(
            world.registration,
            UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        );
        let outcome = self.text.release(
            world.requirement.binding(),
            self.host_session.effect_port(),
            request,
        );
        let receipt = match outcome {
            UiNativeMountedTextReleaseOutcome::Native(
                UiGlyphRasterTransactionOutcome::Committed(receipt),
            ) => receipt,
            UiNativeMountedTextReleaseOutcome::Native(
                UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
                    worth_ui_host_contract::UiGlyphRasterTransactionDenial::ReservationConflict,
                ),
            ) => return false,
            _ => panic!("one pressure owner must release through the native atlas"),
        };
        assert_eq!(
            receipt.committed_pins(),
            self.evidence.borrow().committed_pin_census[1]
        );
        let host = self.host_session.effect_port();
        assert!(matches!(
            host.adapter()
                .deregister_surface(host.authority(), world.registration),
            UiHostSurfaceDeregistrationOutcome::Deregistered(_)
        ));
        self.prepare_next_pressure_world();
        self.evidence.borrow_mut().pressure_releases += 1;
        self.pressure_release_pending = false;
        self.next_world += 1;
        self.begin_next()
    }

    pub(super) fn release_and_close(&mut self) -> bool {
        let first_request = worth_ui_host_contract::UiMountedTextPinReleaseRequest::from_runtime(
            self.worlds[0].registration,
            UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        );
        assert!(matches!(
            self.text.release(
                self.worlds[0].requirement.binding(),
                self.host_session.effect_port(),
                first_request,
            ),
            UiNativeMountedTextReleaseOutcome::Local
        ));
        self.evidence.borrow_mut().first_release_was_local = true;

        let request = worth_ui_host_contract::UiMountedTextPinReleaseRequest::from_runtime(
            self.worlds[1].registration,
            UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        );
        let UiNativeMountedTextReleaseOutcome::Native(UiGlyphRasterTransactionOutcome::Committed(
            release_receipt,
        )) = self.text.release(
            self.worlds[1].requirement.binding(),
            self.host_session.effect_port(),
            request,
        )
        else {
            panic!("the exact last-owner pin release must commit");
        };
        assert_eq!(release_receipt.committed_pins(), 0);
        self.evidence.borrow_mut().final_release_crossed_native = true;

        for world in self.worlds.iter().take(2) {
            let host = self.host_session.effect_port();
            assert!(matches!(
                host.adapter()
                    .deregister_surface(host.authority(), world.registration),
                UiHostSurfaceDeregistrationOutcome::Deregistered(_)
            ));
        }
        assert!(matches!(
            self.host_session.release_adapter_session(),
            UiHostSessionReleaseOutcome::Released(_)
        ));
        self.finished = true;
        true
    }
}
