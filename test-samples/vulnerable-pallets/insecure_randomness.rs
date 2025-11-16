// Insecure Randomness Pallet
// Demonstrates vulnerabilities in random number generation
// Based on Polkadot Top 10 #1: Insecure Randomness
//
// WARNING: This code contains INTENTIONAL vulnerabilities for testing purposes

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, Randomness};
    use sp_runtime::traits::Hash;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type MyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn lottery_pot)]
    pub type LotteryPot<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn participants)]
    pub type Participants<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn winner_history)]
    pub type WinnerHistory<T: Config> = StorageMap<_, Blake2_128Concat, BlockNumberFor<T>, T::AccountId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ParticipantJoined { who: T::AccountId },
        LotteryDrawn { winner: T::AccountId, amount: BalanceOf<T> },
        RandomNumberGenerated { random: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoParticipants,
        NotEnoughFunds,
        RandomnessFailure,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY 1: Using block hash as randomness source
        /// Block hashes can be predicted by validators/miners
        #[pallet::weight(10_000)]
        pub fn draw_lottery_insecure_v1(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let participants = Participants::<T>::get();
            ensure!(!participants.is_empty(), Error::<T>::NoParticipants);

            // VULNERABLE: Using previous block hash
            // Validators can see this and manipulate block production
            let current_block = frame_system::Pallet::<T>::block_number();
            let parent_hash = frame_system::Pallet::<T>::parent_hash();

            // Convert hash to number (predictable!)
            let random_seed = parent_hash.using_encoded(|b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            });

            let winner_index = (random_seed as usize) % participants.len();
            let winner = participants[winner_index].clone();

            let pot = LotteryPot::<T>::get();
            T::Currency::deposit_creating(&winner, pot);

            Self::deposit_event(Event::LotteryDrawn { winner, amount: pot });
            Ok(())
        }

        /// VULNERABILITY 2: Using block number as randomness
        /// Block numbers are 100% predictable
        #[pallet::weight(10_000)]
        pub fn draw_lottery_insecure_v2(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let participants = Participants::<T>::get();
            ensure!(!participants.is_empty(), Error::<T>::NoParticipants);

            // VULNERABLE: Block number is completely predictable
            let current_block = frame_system::Pallet::<T>::block_number();
            let block_number: u32 = current_block.saturated_into();

            // Attacker can calculate winner in advance!
            let winner_index = (block_number as usize) % participants.len();
            let winner = participants[winner_index].clone();

            let pot = LotteryPot::<T>::get();
            T::Currency::deposit_creating(&winner, pot);

            Self::deposit_event(Event::LotteryDrawn { winner, amount: pot });
            Ok(())
        }

        /// VULNERABILITY 3: Randomness Collective Flip (last 81 blocks)
        /// Still vulnerable to validator manipulation
        #[pallet::weight(10_000)]
        pub fn draw_lottery_collective_flip(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let participants = Participants::<T>::get();
            ensure!(!participants.is_empty(), Error::<T>::NoParticipants);

            // VULNERABLE: pallet_randomness_collective_flip uses last 81 block hashes
            // Still manipulable by validators controlling block production
            let current_block = frame_system::Pallet::<T>::block_number();
            let (random_hash, _) = T::MyRandomness::random(&b"lottery"[..]);

            let random_number = random_hash.using_encoded(|b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            });

            let winner_index = (random_number as usize) % participants.len();
            let winner = participants[winner_index].clone();

            let pot = LotteryPot::<T>::get();
            T::Currency::deposit_creating(&winner, pot);

            Self::deposit_event(Event::LotteryDrawn { winner, amount: pot });
            Self::deposit_event(Event::RandomNumberGenerated { random: random_number });

            Ok(())
        }

        /// VULNERABILITY 4: Using timestamp as randomness
        /// Timestamps can be manipulated within consensus rules
        #[pallet::weight(10_000)]
        pub fn draw_lottery_timestamp(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let participants = Participants::<T>::get();
            ensure!(!participants.is_empty(), Error::<T>::NoParticipants);

            // VULNERABLE: Block timestamp can be manipulated by validators
            // Usually has ~6 second drift tolerance
            let now = frame_system::Pallet::<T>::block_number();
            let timestamp_like: u32 = now.saturated_into();

            let winner_index = (timestamp_like as usize) % participants.len();
            let winner = participants[winner_index].clone();

            let pot = LotteryPot::<T>::get();
            T::Currency::deposit_creating(&winner, pot);

            Self::deposit_event(Event::LotteryDrawn { winner, amount: pot });
            Ok(())
        }

        /// VULNERABILITY 5: Using account data as entropy
        /// Predictable and manipulable by users
        #[pallet::weight(10_000)]
        pub fn generate_nft_id_insecure(
            origin: OriginFor<T>,
            name: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: User can craft 'name' to get desired NFT ID
            // This allows gaming rarity systems
            let mut data = name.clone();
            data.extend_from_slice(&who.encode());

            let hash = T::Hashing::hash(&data);
            let nft_id = hash.using_encoded(|b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            });

            // Attacker can brute-force 'name' to get rare NFT ID
            Self::deposit_event(Event::RandomNumberGenerated { random: nft_id });

            Ok(())
        }

        /// VULNERABILITY 6: Reusing randomness across blocks
        /// Same random value used multiple times
        #[pallet::weight(10_000)]
        pub fn batch_distribute_rewards_insecure(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let participants = Participants::<T>::get();
            ensure!(!participants.is_empty(), Error::<T>::NoParticipants);

            // VULNERABLE: Single random source for multiple rewards
            let current_block = frame_system::Pallet::<T>::block_number();
            let (random_hash, _) = T::MyRandomness::random(&b"rewards"[..]);

            let base_random = random_hash.using_encoded(|b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            });

            // Using same random + offset is predictable
            for (i, participant) in participants.iter().enumerate() {
                let reward_multiplier = (base_random + i as u32) % 100;
                // Distribute rewards based on predictable values
                // Attacker can position themselves to get best multiplier
            }

            Ok(())
        }

        /// Helper: Join lottery
        #[pallet::weight(10_000)]
        pub fn join_lottery(origin: OriginFor<T>, entry_fee: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            T::Currency::withdraw(
                &who,
                entry_fee,
                frame_support::traits::WithdrawReasons::TRANSFER,
                frame_support::traits::ExistenceRequirement::KeepAlive,
            )?;

            let mut participants = Participants::<T>::get();
            participants.push(who.clone());
            Participants::<T>::put(participants);

            let mut pot = LotteryPot::<T>::get();
            pot = pot.saturating_add(entry_fee);
            LotteryPot::<T>::put(pot);

            Self::deposit_event(Event::ParticipantJoined { who });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// SAFE EXAMPLE: How randomness SHOULD be used
        ///
        /// Proper randomness in Substrate requires:
        /// 1. BABE VRF (Verifiable Random Function) - cryptographically secure
        /// 2. Future block hash commitment
        /// 3. Multiple sources of entropy
        ///
        /// For production:
        /// - Use pallet_babe::RandomnessFromOneEpochAgo
        /// - Use pallet_babe::RandomnessFromTwoEpochsAgo (more secure, higher latency)
        /// - Consider Chainlink VRF for critical applications
        /// - NEVER use: block hash, block number, timestamp alone
        pub fn _safe_randomness_example() {
            // Use BABE VRF (configured in Config trait)
            // let (random_hash, block_number) = T::MyRandomness::random(&b"context"[..]);
            //
            // For best security:
            // 1. Use subject parameter that includes user input
            // 2. Store commitment before randomness is known
            // 3. Reveal after randomness is available
            // 4. Use VRF output, not just block hash
        }
    }
}

// Educational Notes:
//
// Real-world impact of insecure randomness:
//
// 1. Front-running: Validators see tx pool, can game lotteries/NFT mints
// 2. Rarity manipulation: Users craft inputs to get rare items
// 3. Unfair advantage: Participants can predict outcomes
// 4. MEV (Maximal Extractable Value): Block producers extract value
//
// Polkadot's solutions:
// - BABE VRF: Cryptographically secure, unpredictable
// - Epoch randomness: Committed in advance, revealed later
// - Multiple validators: Harder to manipulate consensus
//
// References:
// - Polkadot Top 10 Security: https://security.parity.io/top
// - BABE paper: https://research.web3.foundation/en/latest/polkadot/BABE/Babe.html
// - Substrate randomness docs: https://docs.substrate.io/build/randomness/
