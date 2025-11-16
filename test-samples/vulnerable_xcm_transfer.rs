//! Vulnerable XCM Transfer Pallet
//!
//! This pallet demonstrates XCM decimal precision vulnerabilities.
//! It performs cross-chain transfers without properly handling decimal differences
//! between parachains, which can lead to loss of funds or incorrect amounts.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use xcm::latest::prelude::*;
    use xcm_executor::traits::TransferType;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type XcmSender: SendXcm;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// XCM transfer initiated
        TransferInitiated {
            dest: MultiLocation,
            amount: Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// XCM send failed
        XcmSendFailed,
        /// Invalid destination
        InvalidDestination,
    }

    type Balance = u128;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABLE: Transfer DOT to a parachain without decimal conversion
        ///
        /// This function transfers 1 DOT (1_000_000_000_000 Planck, 12 decimals)
        /// to another parachain without checking if that parachain uses the same
        /// decimal configuration. If the destination uses 10 decimals (like Polkadot),
        /// this will transfer 100 DOT instead of 1 DOT!
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn vulnerable_transfer_to_parachain(
            origin: OriginFor<T>,
            para_id: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // VULNERABILITY: Hardcoded amount without considering decimal differences
            // Assuming this is Acala (12 decimals) sending to Polkadot (10 decimals)
            let amount = 1_000_000_000_000u128; // 1 DOT in 12 decimals

            let dest = MultiLocation {
                parents: 1,
                interior: X1(Parachain(para_id)),
            };

            // VULNERABILITY: Direct transfer without decimal conversion
            Self::do_xcm_transfer(dest.clone(), amount)?;

            Self::deposit_event(Event::TransferInitiated {
                dest,
                amount,
            });

            Ok(())
        }

        /// VULNERABLE: Transfer with runtime-calculated amount but no decimal handling
        ///
        /// Even though this uses a parameter, it doesn't convert decimals
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn vulnerable_transfer_custom_amount(
            origin: OriginFor<T>,
            para_id: u32,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let dest = MultiLocation {
                parents: 1,
                interior: X1(Parachain(para_id)),
            };

            // VULNERABILITY: No decimal conversion even with user-provided amount
            Self::do_xcm_transfer(dest.clone(), amount)?;

            Self::deposit_event(Event::TransferInitiated {
                dest,
                amount,
            });

            Ok(())
        }

        /// VULNERABLE: Hardcoded withdrawal amount
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn vulnerable_withdraw_asset(
            origin: OriginFor<T>,
            para_id: u32,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // VULNERABILITY: Hardcoded 10 DOT without decimal consideration
            let amount = 10_000_000_000_000u128; // 10 DOT in some decimal system

            let asset = MultiAsset {
                id: Concrete(MultiLocation::here()),
                fun: Fungible(amount),
            };

            // This could withdraw 1000 DOT if decimals are different!
            let dest = MultiLocation {
                parents: 1,
                interior: X1(Parachain(para_id)),
            };

            Self::withdraw_asset(asset, dest)?;

            Ok(())
        }

        /// VULNERABLE: Multiple transfers with hardcoded amounts
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn vulnerable_batch_transfer(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // VULNERABILITY: Multiple hardcoded amounts
            let amounts = vec![
                1_000_000_000_000u128,  // 1 DOT?
                5_000_000_000_000u128,  // 5 DOT?
                10_000_000_000_000u128, // 10 DOT?
            ];

            for (i, amount) in amounts.iter().enumerate() {
                let dest = MultiLocation {
                    parents: 1,
                    interior: X1(Parachain(2000 + i as u32)),
                };

                // VULNERABILITY: No decimal conversion in batch operations
                Self::do_xcm_transfer(dest, *amount)?;
            }

            Ok(())
        }

        /// SAFE EXAMPLE: Proper decimal conversion
        ///
        /// This is how it should be done
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn safe_transfer_with_conversion(
            origin: OriginFor<T>,
            para_id: u32,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let dest = MultiLocation {
                parents: 1,
                interior: X1(Parachain(para_id)),
            };

            // SAFE: Convert decimals before transfer
            let source_decimals = Self::get_chain_decimals(); // 12 for Acala
            let dest_decimals = Self::get_parachain_decimals(para_id); // 10 for Polkadot
            let converted_amount = Self::convert_balance(amount, source_decimals, dest_decimals);

            Self::do_xcm_transfer(dest.clone(), converted_amount)?;

            Self::deposit_event(Event::TransferInitiated {
                dest,
                amount: converted_amount,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Internal XCM transfer function
        fn do_xcm_transfer(dest: MultiLocation, amount: Balance) -> DispatchResult {
            let asset = MultiAsset {
                id: Concrete(MultiLocation::here()),
                fun: Fungible(amount),
            };

            // Simplified XCM message construction
            let _message = Xcm(vec![
                WithdrawAsset(vec![asset.clone()].into()),
                BuyExecution {
                    fees: asset.clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: All.into(),
                    beneficiary: dest,
                },
            ]);

            // In real implementation, would send message via XcmSender
            // T::XcmSender::send_xcm(dest, message)?;

            Ok(())
        }

        /// Withdraw asset (simplified)
        fn withdraw_asset(_asset: MultiAsset, _dest: MultiLocation) -> DispatchResult {
            // Simplified implementation
            Ok(())
        }

        /// Get decimal configuration for this chain
        fn get_chain_decimals() -> u32 {
            12 // Example: Acala uses 12 decimals
        }

        /// Get decimal configuration for destination parachain
        fn get_parachain_decimals(para_id: u32) -> u32 {
            match para_id {
                0 => 10,    // Polkadot Relay Chain: 10 decimals
                1000 => 12, // Acala: 12 decimals
                2000 => 18, // Moonbeam: 18 decimals (GLMR)
                2004 => 10, // Hydration: 10 decimals
                _ => 12,    // Default to 12
            }
        }

        /// Convert balance between different decimal systems
        ///
        /// SAFE: This is the proper way to handle decimal conversion
        fn convert_balance(amount: Balance, from_decimals: u32, to_decimals: u32) -> Balance {
            if from_decimals == to_decimals {
                return amount;
            }

            if from_decimals > to_decimals {
                // Scale down: divide by 10^(from_decimals - to_decimals)
                let scale = 10u128.pow(from_decimals - to_decimals);
                amount / scale
            } else {
                // Scale up: multiply by 10^(to_decimals - from_decimals)
                let scale = 10u128.pow(to_decimals - from_decimals);
                amount * scale
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_conversion() {
        // 1 DOT with 12 decimals (Acala) = 1_000_000_000_000
        let acala_amount = 1_000_000_000_000u128;

        // Convert to Polkadot (10 decimals)
        let polkadot_amount = pallet::Pallet::<()>::convert_balance(acala_amount, 12, 10);

        // Should be 10_000_000_000 (1 DOT with 10 decimals)
        assert_eq!(polkadot_amount, 10_000_000_000);
    }

    #[test]
    fn test_decimal_conversion_upscale() {
        // 1 DOT with 10 decimals (Polkadot) = 10_000_000_000
        let polkadot_amount = 10_000_000_000u128;

        // Convert to Moonbeam (18 decimals)
        let moonbeam_amount = pallet::Pallet::<()>::convert_balance(polkadot_amount, 10, 18);

        // Should be 10_000_000_000_000_000_000 (1 DOT with 18 decimals)
        assert_eq!(moonbeam_amount, 10_000_000_000_000_000_000);
    }
}
