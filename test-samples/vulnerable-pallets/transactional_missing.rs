// Missing Transactional Attribute Pallet
// Demonstrates vulnerabilities from missing #[transactional] annotations
// Based on Polkadot Top 10: Transactional Issues
//
// WARNING: This code contains INTENTIONAL vulnerabilities for testing purposes

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn balance_of)]
    pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn allowance)]
    pub type Allowances<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // owner
        Blake2_128Concat,
        T::AccountId, // spender
        BalanceOf<T>,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn locked_amount)]
    pub type LockedAmounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type TotalSupply<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transferred { from: T::AccountId, to: T::AccountId, amount: BalanceOf<T> },
        Minted { to: T::AccountId, amount: BalanceOf<T> },
        Burned { from: T::AccountId, amount: BalanceOf<T> },
        Locked { who: T::AccountId, amount: BalanceOf<T> },
        Unlocked { who: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        InsufficientAllowance,
        Overflow,
        TransferFailed,
        MintFailed,
        LockFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY 1: Multi-step operation without #[transactional]
        /// If any step fails, previous changes aren't reverted
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn transfer_with_fee(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: BalanceOf<T>,
            fee: BalanceOf<T>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            // Step 1: Transfer amount
            let from_balance = Balances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

            Balances::<T>::insert(&from, from_balance - amount);
            let to_balance = Balances::<T>::get(&to);
            Balances::<T>::insert(&to, to_balance + amount);

            // Step 2: Transfer fee (this might fail!)
            let fee_collector: T::AccountId = T::AccountId::decode(&mut &b"fee_collector"[..]).unwrap();
            let from_balance_after = Balances::<T>::get(&from);

            // VULNERABLE: If this fails, Step 1 is NOT reverted!
            // User loses 'amount' but fee collection fails
            // Partial state change = corruption
            ensure!(from_balance_after >= fee, Error::<T>::InsufficientBalance);

            Balances::<T>::insert(&from, from_balance_after - fee);
            let fee_balance = Balances::<T>::get(&fee_collector);
            Balances::<T>::insert(&fee_collector, fee_balance + fee);

            Self::deposit_event(Event::Transferred { from, to, amount });
            Ok(())
        }

        /// VULNERABILITY 2: Minting without rollback on failure
        /// Total supply updated even if recipient update fails
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn mint_and_distribute(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Step 1: Update total supply
            let total = TotalSupply::<T>::get();
            let new_total = total.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
            TotalSupply::<T>::put(new_total);

            // Step 2: Credit recipient (might fail!)
            let recipient_balance = Balances::<T>::get(&recipient);
            let new_balance = recipient_balance.checked_add(&amount)
                .ok_or(Error::<T>::Overflow)?;

            // VULNERABLE: If this ensure! fails, total supply already increased!
            // Money printed but not distributed = accounting error
            ensure!(new_balance <= 1000000u32.into(), Error::<T>::MintFailed);

            Balances::<T>::insert(&recipient, new_balance);

            Self::deposit_event(Event::Minted { to: recipient, amount });
            Ok(())
        }

        /// VULNERABILITY 3: Complex swap without atomicity
        /// Partial swaps leave inconsistent state
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn swap_tokens(
            origin: OriginFor<T>,
            other: T::AccountId,
            amount_give: BalanceOf<T>,
            amount_receive: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Step 1: Deduct from who
            let who_balance = Balances::<T>::get(&who);
            ensure!(who_balance >= amount_give, Error::<T>::InsufficientBalance);
            Balances::<T>::insert(&who, who_balance - amount_give);

            // Step 2: Credit other (with amount_give)
            let other_balance = Balances::<T>::get(&other);
            Balances::<T>::insert(&other, other_balance + amount_give);

            // Step 3: Deduct from other (might fail!)
            let other_balance_after = Balances::<T>::get(&other);
            ensure!(other_balance_after >= amount_receive, Error::<T>::InsufficientBalance);

            // VULNERABLE: If this fails, Steps 1 & 2 already executed!
            // Tokens transferred one way but not reciprocated
            Balances::<T>::insert(&other, other_balance_after - amount_receive);

            // Step 4: Credit who
            let who_balance_after = Balances::<T>::get(&who);
            Balances::<T>::insert(&who, who_balance_after + amount_receive);

            Ok(())
        }

        /// VULNERABILITY 4: Lock/Unlock without rollback
        /// Lock succeeds but subsequent operation fails
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn lock_and_transfer(
            origin: OriginFor<T>,
            amount_to_lock: BalanceOf<T>,
            transfer_to: T::AccountId,
            transfer_amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Step 1: Lock tokens
            let locked = LockedAmounts::<T>::get(&who);
            LockedAmounts::<T>::insert(&who, locked + amount_to_lock);

            let balance = Balances::<T>::get(&who);
            let available = balance - locked - amount_to_lock;

            // Step 2: Transfer (might fail!)
            // VULNERABLE: If transfer fails, tokens remain locked!
            // User's funds stuck in locked state
            ensure!(available >= transfer_amount, Error::<T>::InsufficientBalance);

            Balances::<T>::insert(&who, balance - transfer_amount);
            let to_balance = Balances::<T>::get(&transfer_to);
            Balances::<T>::insert(&transfer_to, to_balance + transfer_amount);

            Self::deposit_event(Event::Locked { who, amount: amount_to_lock });
            Ok(())
        }

        /// VULNERABILITY 5: Batch operations without transactional
        /// Some transfers succeed, others fail, inconsistent state
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn batch_transfer(
            origin: OriginFor<T>,
            recipients: Vec<(T::AccountId, BalanceOf<T>)>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            // VULNERABLE: If 3rd transfer fails, first 2 already went through
            // No rollback = inconsistent state
            for (to, amount) in recipients {
                let from_balance = Balances::<T>::get(&from);
                ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

                Balances::<T>::insert(&from, from_balance - amount);
                let to_balance = Balances::<T>::get(&to);
                Balances::<T>::insert(&to, to_balance + amount);

                Self::deposit_event(Event::Transferred {
                    from: from.clone(),
                    to,
                    amount
                });
            }

            Ok(())
        }

        /// VULNERABILITY 6: Burn without total supply adjustment atomicity
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        // MISSING: #[transactional]
        pub fn burn_from_multiple(
            origin: OriginFor<T>,
            accounts: Vec<(T::AccountId, BalanceOf<T>)>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let mut total_burned = BalanceOf::<T>::from(0u32);

            for (account, amount) in accounts {
                let balance = Balances::<T>::get(&account);
                ensure!(balance >= amount, Error::<T>::InsufficientBalance);

                Balances::<T>::insert(&account, balance - amount);
                total_burned = total_burned + amount;

                Self::deposit_event(Event::Burned { from: account, amount });
            }

            // VULNERABLE: Total supply update at the end
            // If this fails, all burns already happened!
            // Tokens destroyed but total supply not adjusted
            let total = TotalSupply::<T>::get();
            let new_total = total.checked_sub(&total_burned)
                .ok_or(Error::<T>::Overflow)?;
            TotalSupply::<T>::put(new_total);

            Ok(())
        }

        /// SAFE EXAMPLE: Using #[transactional] properly
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        #[transactional] // ✅ SAFE: All-or-nothing
        pub fn safe_transfer_with_fee(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: BalanceOf<T>,
            fee: BalanceOf<T>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            // Step 1: Transfer amount
            let from_balance = Balances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);
            Balances::<T>::insert(&from, from_balance - amount);
            let to_balance = Balances::<T>::get(&to);
            Balances::<T>::insert(&to, to_balance + amount);

            // Step 2: Transfer fee
            let fee_collector: T::AccountId = T::AccountId::decode(&mut &b"fee_collector"[..]).unwrap();
            let from_balance_after = Balances::<T>::get(&from);
            ensure!(from_balance_after >= fee, Error::<T>::InsufficientBalance);

            // SAFE: If fee transfer fails, Step 1 is automatically rolled back!
            Balances::<T>::insert(&from, from_balance_after - fee);
            let fee_balance = Balances::<T>::get(&fee_collector);
            Balances::<T>::insert(&fee_collector, fee_balance + fee);

            Self::deposit_event(Event::Transferred { from, to, amount });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// SAFE EXAMPLE: Best practices for transactional operations
        ///
        /// Rules:
        /// 1. Use #[transactional] for ANY multi-step operation
        /// 2. Use #[transactional] for operations that can fail midway
        /// 3. Use #[transactional] for batches
        /// 4. Use #[transactional] for complex state updates
        ///
        /// Modern Substrate (v10+):
        /// - #[transactional] is DEFAULT for dispatchables
        /// - But be explicit for clarity
        ///
        /// #[transactional] ensures:
        /// - All storage changes reverted on error
        /// - Atomicity: all-or-nothing execution
        /// - Consistency: no partial state changes
        /// - Proper event emission (events also rolled back)
        ///
        /// When NOT to use:
        /// - Single storage write with no failure path
        /// - Read-only operations
        /// - Operations that are naturally atomic
        ///
        /// Example patterns:
        /// ```
        /// #[transactional]
        /// pub fn complex_operation() -> DispatchResult {
        ///     // Multiple storage writes
        ///     // Any error rolls back ALL changes
        ///     // Clean state guaranteed
        /// }
        /// ```
        ///
        /// Nested transactional:
        /// ```
        /// #[transactional]
        /// pub fn outer() -> DispatchResult {
        ///     inner()?; // Also transactional
        ///     // If inner fails, outer rolls back too
        /// }
        /// ```
        pub fn _safe_transactional_example() {}
    }
}

// Educational Notes:
//
// Real-world impact of missing #[transactional]:
//
// 1. Accounting errors: Money minted but not distributed
// 2. Locked funds: Locks succeed but unlocks fail
// 3. Partial swaps: One side transfers, other doesn't
// 4. Inconsistent state: Total supply != sum of balances
// 5. Exploit vectors: Attackers can trigger partial failures
//
// Historical incidents:
// - Early Substrate pallets had this issue frequently
// - Modern Substrate makes #[transactional] default
// - Still common in custom pallets and older code
//
// Detection:
// - Look for multiple storage writes in one function
// - Check for operations after fallible calls
// - Audit batch operations
// - Review error handling paths
//
// Best practices:
// 1. Always use #[transactional] for multi-step ops
// 2. Test failure scenarios
// 3. Verify state consistency after errors
// 4. Use storage transactions explicitly if needed:
//    ```
//    with_transaction(|| {
//        // operations
//        TransactionOutcome::Commit(result)
//    })
//    ```
// 5. Document why #[transactional] is NOT used (if applicable)
//
// References:
// - Polkadot Top 10: https://security.parity.io/top
// - Transactional storage: https://docs.substrate.io/build/runtime-storage/
// - FRAME macros: https://docs.substrate.io/reference/frame-macros/
