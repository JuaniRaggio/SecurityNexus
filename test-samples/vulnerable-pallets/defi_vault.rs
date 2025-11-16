// DeFi Vault Pallet - Cross-chain asset management
// Based on real vulnerabilities found in 2024 DeFi protocols
// This pallet allows users to deposit assets and earn yield

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency, ExistenceRequirement};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// User deposits in the vault
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    /// Total value locked in the vault
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Reward multiplier for yield calculation (stored as basis points, 10000 = 100%)
    #[pallet::storage]
    #[pallet::getter(fn reward_rate)]
    pub type RewardRate<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Pending withdrawals (implements a callback pattern)
    #[pallet::storage]
    pub type PendingWithdrawals<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Deposited { who: T::AccountId, amount: BalanceOf<T> },
        Withdrawn { who: T::AccountId, amount: BalanceOf<T> },
        RewardsClaimed { who: T::AccountId, amount: BalanceOf<T> },
        RewardRateUpdated { new_rate: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        Overflow,
        InsufficientVaultBalance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit assets into the vault
        #[pallet::weight(10_000)]
        pub fn deposit(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Transfer funds from user to vault
            T::Currency::transfer(&who, &Self::account_id(), amount, ExistenceRequirement::KeepAlive)?;

            // VULNERABILITY 1: Integer overflow not checked
            // In 2024, several DeFi protocols were exploited via integer overflow
            let current_deposit = Deposits::<T>::get(&who);
            let new_deposit = current_deposit + amount; // Can overflow!

            Deposits::<T>::insert(&who, new_deposit);

            // VULNERABILITY 2: Race condition - TVL updated after deposit storage
            // An attacker could exploit the gap between these two operations
            let current_tvl = TotalLocked::<T>::get();
            TotalLocked::<T>::put(current_tvl + amount);

            Self::deposit_event(Event::Deposited { who, amount });
            Ok(())
        }

        /// Withdraw assets from the vault with rewards
        #[pallet::weight(10_000)]
        pub fn withdraw(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let user_balance = Deposits::<T>::get(&who);
            ensure!(user_balance >= amount, Error::<T>::InsufficientBalance);

            // Calculate rewards (this is where reentrancy can happen)
            let rewards = Self::calculate_rewards(&who, amount);

            // VULNERABILITY 3: Reentrancy vulnerability (similar to Vyper 2024 bug)
            // Balance is updated AFTER the callback, allowing reentrancy
            PendingWithdrawals::<T>::insert(&who, amount);

            // This callback could call back into withdraw() before state is updated
            Self::process_withdrawal_callback(&who, amount, rewards)?;

            // State updated too late - attacker can reenter before this
            Deposits::<T>::insert(&who, user_balance - amount);

            let current_tvl = TotalLocked::<T>::get();
            TotalLocked::<T>::put(current_tvl - amount);

            Self::deposit_event(Event::Withdrawn { who, amount });
            Ok(())
        }

        /// Claim accumulated rewards
        #[pallet::weight(10_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let user_balance = Deposits::<T>::get(&who);
            let rewards = Self::calculate_rewards(&who, user_balance);

            // VULNERABILITY 4: No check if vault has sufficient balance
            // Could drain the vault if rewards calculation is manipulated
            T::Currency::transfer(
                &Self::account_id(),
                &who,
                rewards,
                ExistenceRequirement::AllowDeath
            )?;

            Self::deposit_event(Event::RewardsClaimed { who, amount: rewards });
            Ok(())
        }

        /// Update the reward rate
        /// VULNERABILITY 5: Missing access control!
        /// Anyone can call this function and manipulate rewards
        #[pallet::weight(10_000)]
        pub fn set_reward_rate(origin: OriginFor<T>, new_rate: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Should have: ensure_root(origin)?;
            // But it's missing! Anyone can set reward rate to 1000000%

            RewardRate::<T>::put(new_rate);

            Self::deposit_event(Event::RewardRateUpdated { new_rate });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the account ID of the vault
        pub fn account_id() -> T::AccountId {
            // In production, this should be a proper account derivation
            T::AccountId::decode(&mut &b"defi_vault"[..]).unwrap()
        }

        /// Calculate rewards for a user
        /// VULNERABILITY 6: Integer overflow in multiplication
        fn calculate_rewards(who: &T::AccountId, amount: BalanceOf<T>) -> BalanceOf<T> {
            let rate = RewardRate::<T>::get();
            let balance = Deposits::<T>::get(who);

            // VULNERABILITY: No overflow check, rate * balance can overflow
            // If attacker sets rate to max u32 via set_reward_rate()...
            let reward_basis = balance.saturating_mul(rate.into());

            // Division by 10000 to convert basis points to percentage
            reward_basis / 10000u32.into()
        }

        /// Process withdrawal with callback pattern
        /// VULNERABILITY 7: External call before state update (reentrancy)
        fn process_withdrawal_callback(
            who: &T::AccountId,
            amount: BalanceOf<T>,
            rewards: BalanceOf<T>,
        ) -> DispatchResult {
            let total = amount + rewards;

            // CRITICAL: External transfer happens before storage update
            // This is the exact pattern that caused the Vyper bug in 2024
            T::Currency::transfer(
                &Self::account_id(),
                who,
                total,
                ExistenceRequirement::AllowDeath
            )?;

            // An attacker can call withdraw() again here before storage is cleared
            PendingWithdrawals::<T>::remove(who);

            Ok(())
        }
    }
}
