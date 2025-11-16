// Replay Attack Pallet
// Demonstrates vulnerabilities from unsigned extrinsics and replay attacks
// Based on Polkadot Top 10 #5: Replay Attacks
//
// WARNING: This code contains INTENTIONAL vulnerabilities for testing purposes

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::Currency;
    use frame_support::unsigned::TransactionValidity;
    use sp_runtime::transaction_validity::{
        InvalidTransaction, TransactionSource, ValidTransaction,
    };
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Price feed data (submitted via unsigned extrinsics)
    #[pallet::storage]
    #[pallet::getter(fn price)]
    pub type Price<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Heartbeat from validators
    #[pallet::storage]
    pub type Heartbeats<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BlockNumberFor<T>
    >;

    /// Claim status (for faucet)
    #[pallet::storage]
    pub type HasClaimed<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery
    >;

    /// VULNERABILITY: No nonce tracking for unsigned transactions
    #[pallet::storage]
    pub type ProcessedOperations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,  // Should be (AccountId, nonce) but isn't!
        bool
    >;

    #[pallet::event]
    #[pallet:generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PriceUpdated { price: u128 },
        HeartbeatReceived { who: T::AccountId },
        TokensClaimed { who: T::AccountId, amount: BalanceOf<T> },
        VoteSubmitted { who: T::AccountId, vote: bool },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyClaimed,
        InvalidSignature,
        ReplayDetected,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY 1: Unsigned transaction without replay protection
        /// Same price update can be submitted infinitely
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn submit_price_unsigned(
            origin: OriginFor<T>,
            price: u128,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // VULNERABLE: No nonce, no timestamp check
            // Attacker can replay old price updates
            // If price was 100 yesterday and 50 today,
            // attacker can keep submitting the 100 price

            Price::<T>::put(price);

            Self::deposit_event(Event::PriceUpdated { price });

            Ok(())
        }

        /// VULNERABILITY 2: Heartbeat without nonce
        /// Same heartbeat can be replayed
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn submit_heartbeat(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // VULNERABLE: No nonce or unique identifier
            // Attacker can replay heartbeats to make offline validators appear online
            let current_block = frame_system::Pallet::<T>::block_number();
            Heartbeats::<T>::insert(&validator, current_block);

            Self::deposit_event(Event::HeartbeatReceived { who: validator });

            Ok(())
        }

        /// VULNERABILITY 3: Faucet with weak replay protection
        /// Uses simple boolean flag, can be bypassed
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn claim_faucet_tokens(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Simple boolean check
            // If storage is cleared or in multiple chains, can claim again
            ensure!(!HasClaimed::<T>::get(&who), Error::<T>::AlreadyClaimed);

            let amount: BalanceOf<T> = 100u32.into();
            T::Currency::deposit_creating(&who, amount);

            HasClaimed::<T>::insert(&who, true);

            Self::deposit_event(Event::TokensClaimed { who, amount });

            Ok(())
        }

        /// VULNERABILITY 4: Unsigned vote without unique ID
        /// Same vote can be counted multiple times
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn submit_vote_unsigned(
            origin: OriginFor<T>,
            voter: T::AccountId,
            vote: bool,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // VULNERABLE: No nonce per voter
            // Attacker can replay same vote hundreds of times
            // Critical for governance!

            Self::deposit_event(Event::VoteSubmitted { who: voter, vote });

            Ok(())
        }

        /// VULNERABILITY 5: Operation ID without account binding
        /// Different users can replay same operation ID
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn process_operation(
            origin: OriginFor<T>,
            operation_id: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: operation_id not bound to account
            // If Alice uses ID 123, Bob can't use it
            // But Alice can use it again from different origin!
            ensure!(
                !ProcessedOperations::<T>::contains_key(operation_id),
                Error::<T>::ReplayDetected
            );

            ProcessedOperations::<T>::insert(operation_id, true);

            // Process operation...

            Ok(())
        }

        /// VULNERABILITY 6: No timestamp validation
        /// Old transactions can be replayed
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn submit_data_no_timestamp(
            origin: OriginFor<T>,
            data: Vec<u8>,
            signature: Vec<u8>,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // VULNERABLE: No timestamp in signed data
            // Attacker can capture transaction from yesterday
            // and replay it today with different context

            // Verify signature (assume it's valid)
            // But signed data doesn't include timestamp or block number
            // So old valid signatures can be replayed

            Ok(())
        }

        /// VULNERABILITY 7: Double-spend via replay
        /// Transaction can be replayed on fork
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn transfer_on_fork(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            // VULNERABLE: No chain ID in transaction
            // If chain forks, same transaction valid on both chains
            // Classic replay attack from Ethereum/ETC split

            T::Currency::transfer(
                &from,
                &to,
                amount,
                frame_support::traits::ExistenceRequirement::KeepAlive
            )?;

            Ok(())
        }

        /// VULNERABILITY 8: Missing deadline check
        /// Transactions can be held and submitted later
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn limit_order_no_deadline(
            origin: OriginFor<T>,
            price_limit: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: No deadline parameter
            // Attacker can see user wants to buy at $100
            // Hold transaction in mempool
            // Submit it when price hits $100 (even months later)
            // User gets frontrun on their own old order

            let current_price = Price::<T>::get();

            if current_price <= price_limit {
                // Execute order
            }

            Ok(())
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(
            _source: TransactionSource,
            call: &Self::Call,
        ) -> TransactionValidity {
            // VULNERABLE: No proper validation
            // Should check nonce, timestamp, signature, etc.

            match call {
                Call::submit_price_unsigned { price: _ } => {
                    // VULNERABLE: Always accepts
                    // No nonce check, no replay protection
                    Ok(ValidTransaction::default())
                }
                Call::submit_heartbeat { validator: _ } => {
                    // VULNERABLE: No unique identifier
                    Ok(ValidTransaction::default())
                }
                Call::submit_vote_unsigned { voter: _, vote: _ } => {
                    // CRITICAL: Allows vote replay!
                    Ok(ValidTransaction::default())
                }
                _ => InvalidTransaction::Call.into(),
            }
        }
    }

    impl<T: Config> Pallet<T> {
        /// SAFE EXAMPLE: Proper replay protection
        ///
        /// Best practices:
        /// 1. Nonce per account for unsigned transactions
        /// 2. Timestamp in signed data (with validity window)
        /// 3. Unique transaction ID (hash of account + nonce + data)
        /// 4. Chain ID to prevent cross-chain replay
        /// 5. Deadline parameter for time-sensitive operations
        /// 6. ValidateUnsigned with proper checks:
        ///    - Signature verification
        ///    - Nonce increment
        ///    - Timestamp within acceptable range
        ///    - Provides uniqueness tag
        ///
        /// Example:
        /// ```
        /// #[pallet::storage]
        /// pub type Nonces<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64>;
        ///
        /// pub fn safe_unsigned_call(
        ///     who: T::AccountId,
        ///     nonce: u64,
        ///     timestamp: u64,
        ///     data: Vec<u8>,
        ///     signature: Vec<u8>,
        /// ) -> DispatchResult {
        ///     // 1. Verify signature includes nonce + timestamp + chain_id
        ///     // 2. Check nonce == Nonces::<T>::get(&who) + 1
        ///     // 3. Check timestamp within 60 seconds
        ///     // 4. Increment nonce
        ///     // 5. Process data
        /// }
        ///
        /// fn validate_unsigned() -> TransactionValidity {
        ///     Ok(ValidTransaction {
        ///         provides: vec![unique_id], // Hash of account + nonce
        ///         requires: vec![],
        ///         priority: priority,
        ///         longevity: 64, // Only valid for 64 blocks
        ///         propagate: true,
        ///     })
        /// }
        /// ```
        pub fn _safe_replay_protection_example() {}
    }
}

// Educational Notes:
//
// Real-world impact of replay attacks:
//
// 1. Double-spending: Same payment processed twice
// 2. Vote manipulation: Governance attacks via vote replay
// 3. Price oracle manipulation: Old prices resubmitted
// 4. Cross-chain replay: Ethereum/ETC split (2016)
// 5. Faucet drain: Infinite free tokens
//
// Famous replay attacks:
// - Ethereum DAO fork (2016): Transactions replayed on ETC
// - Bitcoin Gold (2017): Replay attacks post-fork
// - Various DeFi protocols: Signature replay vulnerabilities
//
// Protection mechanisms:
// 1. Nonce: Unique incrementing number per account
// 2. Timestamp: Transaction validity window
// 3. Chain ID: Prevents cross-chain replay (EIP-155)
// 4. Unique ID: Hash of all transaction data
// 5. Deadline: Transaction expires after time
// 6. Provides/Requires: Transaction pool deduplication
//
// Substrate specifics:
// - ValidateUnsigned for replay protection
// - provides: Unique transaction identifier
// - requires: Dependencies on other transactions
// - longevity: How many blocks tx is valid
//
// References:
// - Polkadot Top 10: https://security.parity.io/top
// - Unsigned transactions: https://docs.substrate.io/build/unsigned-transactions/
// - Transaction validity: https://docs.substrate.io/build/tx-weights-fees/
// - EIP-155 (Ethereum replay protection): https://eips.ethereum.org/EIPS/eip-155
