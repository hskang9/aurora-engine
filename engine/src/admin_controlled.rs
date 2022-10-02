use aurora_engine_types::account_id::AccountId;

pub trait AdminControlled {
    fn get_eth_connector_contract_account(&self) -> AccountId;
    fn set_eth_connector_contract_account(&mut self, account: AccountId);
}

pub struct PausedError;

impl AsRef<[u8]> for PausedError {
    fn as_ref(&self) -> &[u8] {
        ERR_PAUSED.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdminControlled {
        mask: PausedMask,
    }

    impl MockAdminControlled {
        pub fn new() -> Self {
            Self { mask: 0 }
        }
    }

    impl AdminControlled for MockAdminControlled {
        fn get_paused(&self) -> PausedMask {
            self.mask
        }

        fn set_paused(&mut self, paused: PausedMask) {
            self.mask = paused
        }
    }

    #[test]
    fn test_setting_paused_mask_with_1_bit_marks_it_as_paused() {
        let is_owner = false;
        let mask = 1u8;
        let mut admin_controlled = MockAdminControlled::new();

        assert!(!admin_controlled.is_paused(mask, is_owner));
        admin_controlled.set_paused(mask);
        assert!(admin_controlled.is_paused(mask, is_owner));
    }

    #[test]
    fn test_setting_paused_mask_with_0_bit_marks_it_as_not_paused() {
        let is_owner = false;
        let mask = 1u8;
        let mut admin_controlled = MockAdminControlled::new();
        admin_controlled.set_paused(mask);

        assert!(admin_controlled.is_paused(mask, is_owner));
        admin_controlled.set_paused(0u8);
        assert!(!admin_controlled.is_paused(mask, is_owner));
    }

    #[test]
    fn test_setting_paused_mask_with_1_bit_fails_to_assert_not_paused() {
        let is_owner = false;
        let mask = 1u8;
        let admin_controlled = MockAdminControlled::new();

        let result = admin_controlled.assert_not_paused(mask, is_owner);
        assert!(result.is_ok(), "asserting as paused failed");
    }

    #[test]
    fn test_setting_paused_mask_with_0_bit_asserts_not_paused() {
        let is_owner = false;
        let mask = 1u8;
        let mut admin_controlled = MockAdminControlled::new();

        admin_controlled.set_paused(mask);
        let error = admin_controlled
            .assert_not_paused(mask, is_owner)
            .expect_err("asserting as not paused failed");

        let expected_error_message = b"ERR_PAUSED";
        let actual_error_message = error.as_ref();
        assert_eq!(expected_error_message, actual_error_message);
    }

    #[test]
    fn test_paused_mask_has_no_effect_on_owner() {
        let is_owner = true;
        let mask = 1u8;
        let mut admin_controlled = MockAdminControlled::new();

        admin_controlled.set_paused(mask);
        assert!(!admin_controlled.is_paused(mask, is_owner));
    }

    #[test]
    fn test_asserting_paused_mask_has_no_effect_on_owner() {
        let is_owner = true;
        let mask = 1u8;
        let mut admin_controlled = MockAdminControlled::new();

        admin_controlled.set_paused(mask);
        let result = admin_controlled.assert_not_paused(mask, is_owner);
        assert!(result.is_ok(), "asserting as not paused failed");
    }
}
