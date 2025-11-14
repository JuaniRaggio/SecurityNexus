//! Test pallet with intentional vulnerabilities for SAFT testing

#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn balances)]
    pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64>;

    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T> = StorageValue<_, u64>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transfer { from: T::AccountId, to: T::AccountId, amount: u64 },
        Minted { account: T::AccountId, amount: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        Overflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY: Unchecked arithmetic operation
        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u64,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            let from_balance = Balances::<T>::get(&from).unwrap_or(0);
            let to_balance = Balances::<T>::get(&to).unwrap_or(0);

            // VULNERABILITY: Unchecked subtraction could underflow
            let new_from_balance = from_balance - amount;

            // VULNERABILITY: Unchecked addition could overflow
            let new_to_balance = to_balance + amount;

            Balances::<T>::insert(&from, new_from_balance);
            Balances::<T>::insert(&to, new_to_balance);

            Self::deposit_event(Event::Transfer { from, to, amount });

            Ok(())
        }

        /// VULNERABILITY: Unchecked arithmetic in mint
        #[pallet::weight(10_000)]
        pub fn mint(
            origin: OriginFor<T>,
            account: T::AccountId,
            amount: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let current_balance = Balances::<T>::get(&account).unwrap_or(0);
            let current_supply = TotalSupply::<T>::get().unwrap_or(0);

            // VULNERABILITY: Unchecked addition
            let new_balance = current_balance + amount;
            let new_supply = current_supply + amount;

            Balances::<T>::insert(&account, new_balance);
            TotalSupply::<T>::put(new_supply);

            Self::deposit_event(Event::Minted { account, amount });

            Ok(())
        }

        /// VULNERABILITY: Missing origin check (commented out)
        #[pallet::weight(10_000)]
        pub fn burn(
            _origin: OriginFor<T>,
            account: T::AccountId,
            amount: u64,
        ) -> DispatchResult {
            // VULNERABILITY: No origin verification!
            // let _ = ensure_signed(origin)?;

            let current_balance = Balances::<T>::get(&account).unwrap_or(0);

            // VULNERABILITY: Unchecked subtraction
            let new_balance = current_balance - amount;

            Balances::<T>::insert(&account, new_balance);

            Ok(())
        }
    }
}
