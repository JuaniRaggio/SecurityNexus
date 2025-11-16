// Incorrect Weight Benchmarking Pallet
// Demonstrates vulnerabilities from missing or incorrect weight calculations
// Based on Polkadot Top 10 #3: Incorrect Benchmarking
//
// WARNING: This code contains INTENTIONAL vulnerabilities for testing purposes

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn messages)]
    pub type Messages<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>>;

    #[pallet::storage]
    #[pallet::getter(fn user_data)]
    pub type UserData<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn registry)]
    pub type Registry<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageStored { who: T::AccountId },
        DataProcessed { who: T::AccountId, size: u32 },
        BatchCompleted { count: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        TooManyMessages,
        DataTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY 1: Constant weight for variable computation
        /// DoS attack: User can pass huge data, but weight is constant
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)] // WRONG: Should scale with data size!
        pub fn store_unbounded_data(
            origin: OriginFor<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Processing unbounded data with fixed weight
            // Attacker can pass 10MB data but pay for minimal weight
            let mut processed = Vec::new();
            for byte in data.iter() {
                // Expensive operation: O(n) complexity
                processed.push(byte.wrapping_mul(2));
            }

            // Unbounded storage write
            UserData::<T>::insert(&who, processed);

            Self::deposit_event(Event::DataProcessed {
                who,
                size: data.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 2: Missing weight for storage reads/writes
        /// Weight doesn't account for actual storage operations
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)] // WRONG: Doesn't account for storage I/O!
        pub fn batch_store_messages(
            origin: OriginFor<T>,
            messages: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Multiple storage writes with constant weight
            // Each write has a cost, but weight doesn't reflect it
            for msg in messages.iter() {
                let mut user_messages = Messages::<T>::get(&who).unwrap_or_default();
                user_messages.push(msg.clone());

                // Storage write (expensive!) but weight is fixed
                Messages::<T>::insert(&who, user_messages);
            }

            Self::deposit_event(Event::BatchCompleted {
                count: messages.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 3: Underestimated loop complexity
        /// Nested loops with O(n²) complexity but linear weight
        #[pallet::call_index(2)]
        #[pallet::weight(count.saturating_mul(1_000))] // WRONG: Should be count²!
        pub fn process_matrix(
            origin: OriginFor<T>,
            count: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: O(n²) complexity but weight assumes O(n)
            // Attacker can cause block production delays
            let mut result = 0u64;
            for i in 0..count {
                for j in 0..count {
                    // Expensive computation in nested loop
                    result = result.wrapping_add((i as u64).wrapping_mul(j as u64));
                }
            }

            Self::deposit_event(Event::DataProcessed {
                who,
                size: result as u32
            });

            Ok(())
        }

        /// VULNERABILITY 4: No weight for expensive cryptographic operations
        /// Hashing/signature verification not accounted for
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)] // WRONG: Missing crypto operation costs!
        pub fn verify_many_signatures(
            origin: OriginFor<T>,
            data: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Signature verification is expensive
            // But weight doesn't account for it
            for item in data.iter() {
                // Expensive: hashing large data
                let _hash = sp_io::hashing::blake2_256(item);

                // In real scenario: signature verification
                // ed25519_verify, sr25519_verify, etc.
            }

            Self::deposit_event(Event::BatchCompleted {
                count: data.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 5: Missing weight for unbounded iteration
        /// Iterating over unbounded storage with fixed weight
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)] // WRONG: Should account for registry size!
        pub fn process_all_users(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            // VULNERABLE: Registry can grow unbounded
            // Processing all users has fixed weight regardless of size
            let all_users = Registry::<T>::get();

            for user in all_users.iter() {
                // Expensive: storage read for each user
                let data = UserData::<T>::get(user);

                // Expensive: processing user data
                if let Some(d) = data {
                    for _byte in d.iter() {
                        // Computation here
                    }
                }
            }

            Self::deposit_event(Event::BatchCompleted {
                count: all_users.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 6: Weight doesn't account for conditional branches
        /// Different code paths have same weight
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)] // WRONG: Both paths have different costs!
        pub fn conditional_heavy_work(
            origin: OriginFor<T>,
            do_heavy: bool,
            iterations: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if do_heavy {
                // HEAVY PATH: Expensive loop
                let mut result = 0u64;
                for i in 0..iterations {
                    for j in 0..iterations {
                        result = result.wrapping_add(i as u64 * j as u64);
                    }
                }
            } else {
                // LIGHT PATH: Almost free
                // But pays same weight as heavy path!
            }

            Self::deposit_event(Event::DataProcessed {
                who,
                size: iterations
            });

            Ok(())
        }

        /// VULNERABILITY 7: Zero or minimal weight
        /// Completely missing weight annotation
        #[pallet::call_index(6)]
        #[pallet::weight(0)] // CRITICAL: Zero weight!
        pub fn free_dos_attack(
            origin: OriginFor<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // CRITICAL: Zero weight means free computation
            // Attacker can spam this to halt the chain
            let mut result = Vec::new();
            for byte in data.iter() {
                result.push(byte.wrapping_mul(3));
            }

            UserData::<T>::insert(&who, result);

            Self::deposit_event(Event::DataProcessed {
                who,
                size: data.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 8: Weight doesn't account for database lookups
        /// Multiple gets/contains checks not reflected in weight
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)] // WRONG: Missing DB read costs!
        pub fn multiple_lookups(
            origin: OriginFor<T>,
            accounts: Vec<T::AccountId>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // VULNERABLE: Each lookup has a cost
            for account in accounts.iter() {
                // Database read (expensive!)
                let _exists = Messages::<T>::contains_key(account);

                // Another database read
                let _data = UserData::<T>::get(account);

                // Registry lookup
                let registry = Registry::<T>::get();
                let _is_registered = registry.contains(account);
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// SAFE EXAMPLE: Proper weight calculation
        ///
        /// Correct weight should:
        /// 1. Use benchmarking framework (#[pallet::weight(T::WeightInfo::function_name())])
        /// 2. Account for:
        ///    - Computational complexity (loops, nested loops)
        ///    - Storage reads/writes (each read ~25k weight, write ~100k weight)
        ///    - Cryptographic operations (hashing, signatures)
        ///    - Database lookups
        ///    - Memory allocations
        /// 3. Scale with input size for variable-length inputs
        /// 4. Consider worst-case scenarios
        ///
        /// Example:
        /// ```
        /// #[pallet::weight(
        ///     T::DbWeight::get().reads_writes(1, 1)
        ///     .saturating_add(data.len().saturating_mul(100))
        /// )]
        /// pub fn proper_store_data(data: Vec<u8>) -> DispatchResult {
        ///     // Implementation
        /// }
        /// ```
        ///
        /// Use frame-benchmarking:
        /// ```
        /// cargo test --features runtime-benchmarks
        /// ```
        pub fn _safe_weight_example() {}
    }
}

// Educational Notes:
//
// Real-world impact of incorrect weights:
//
// 1. DoS attacks: Chain halts if blocks take too long to execute
// 2. Unfair fees: Users pay too little for expensive operations
// 3. Block stuffing: Attacker fills blocks with underpriced operations
// 4. Network degradation: Validators struggle with overweight blocks
//
// Polkadot weight system:
// - 1 second of block execution = 1,000,000,000,000 weight units
// - Default block: 0.5 seconds for computation (500ms)
// - Benchmarking required for all extrinsics
// - Runtime enforces weight limits
//
// How to benchmark properly:
// 1. Use #[pallet::weight(T::WeightInfo::function_name())]
// 2. Implement benchmarking module
// 3. Test with worst-case inputs
// 4. Account for all storage operations
// 5. Regular re-benchmarking as code changes
//
// References:
// - Polkadot Top 10: https://security.parity.io/top
// - Substrate weights: https://docs.substrate.io/build/tx-weights-fees/
// - Benchmarking: https://docs.substrate.io/test/benchmark/
