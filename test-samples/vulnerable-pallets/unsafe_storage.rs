// Unsafe Storage Pallet
// Demonstrates vulnerabilities from improper storage management
// Based on Polkadot Top 10 #4: Storage Issues
//
// WARNING: This code contains INTENTIONAL vulnerabilities for testing purposes

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// VULNERABILITY: Unbounded vector storage
    /// No limit on size, can grow indefinitely
    #[pallet::storage]
    #[pallet::getter(fn user_posts)]
    pub type UserPosts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<Vec<u8>>, // Unbounded!
        ValueQuery
    >;

    /// VULNERABILITY: No storage deposit required
    /// Users can fill storage for free
    #[pallet::storage]
    #[pallet::getter(fn documents)]
    pub type Documents<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        Vec<u8>
    >;

    /// VULNERABILITY: Unbounded global list
    /// Grows forever, never cleaned up
    #[pallet::storage]
    #[pallet::getter(fn all_users)]
    pub type AllUsers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// VULNERABILITY: No cleanup mechanism
    #[pallet::storage]
    pub type UserMetadata<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        (Vec<u8>, BlockNumberFor<T>)
    >;

    /// Counter for documents (used in vulnerable functions)
    #[pallet::storage]
    pub type DocumentCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PostAdded { who: T::AccountId, count: u32 },
        DocumentStored { id: u32, size: u32 },
        UserRegistered { who: T::AccountId },
        DataCleared { who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        TooManyPosts,
        DocumentTooLarge,
        InsufficientDeposit,
        DocumentNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// VULNERABILITY 1: Unbounded storage without deposit
        /// User can add unlimited posts for free
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_post_no_deposit(
            origin: OriginFor<T>,
            content: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: No deposit required for storage
            // Attacker can spam millions of posts and fill the chain
            let mut posts = UserPosts::<T>::get(&who);
            posts.push(content);

            // No limit check!
            UserPosts::<T>::insert(&who, posts.clone());

            Self::deposit_event(Event::PostAdded {
                who,
                count: posts.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 2: Massive storage without proportional deposit
        /// Fixed fee regardless of data size
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn store_document_fixed_fee(
            origin: OriginFor<T>,
            content: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Fixed deposit of 1 token
            // Whether storing 1 byte or 1MB
            let deposit = 1u32.into();
            T::Currency::reserve(&who, deposit)?;

            let doc_id = DocumentCounter::<T>::get();
            Documents::<T>::insert(doc_id, content.clone());

            DocumentCounter::<T>::put(doc_id + 1);

            Self::deposit_event(Event::DocumentStored {
                id: doc_id,
                size: content.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 3: Unbounded global list
        /// AllUsers grows forever, never pruned
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn register_user_unbounded(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Global list grows unbounded
            // Eventually makes any iteration prohibitively expensive
            let mut all_users = AllUsers::<T>::get();

            // No duplicate check (adds same user multiple times!)
            all_users.push(who.clone());

            AllUsers::<T>::put(all_users);

            Self::deposit_event(Event::UserRegistered { who });

            Ok(())
        }

        /// VULNERABILITY 4: Storage write without clearing old data
        /// Memory leak: old data never removed
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn update_metadata_no_cleanup(
            origin: OriginFor<T>,
            new_data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let current_block = frame_system::Pallet::<T>::block_number();

            // VULNERABLE: Overwrites without checking old size
            // If old data was 1MB and new is 1KB, deposit doesn't reflect this
            UserMetadata::<T>::insert(&who, (new_data, current_block));

            Ok(())
        }

        /// VULNERABILITY 5: Batch operations without aggregate deposit
        /// Can store huge amounts by batching
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn batch_store_posts(
            origin: OriginFor<T>,
            posts: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: No total size check
            // User can pass 1000 posts of 1MB each
            let mut user_posts = UserPosts::<T>::get(&who);

            for post in posts {
                user_posts.push(post);
            }

            // No deposit proportional to total size
            UserPosts::<T>::insert(&who, user_posts.clone());

            Self::deposit_event(Event::PostAdded {
                who,
                count: user_posts.len() as u32
            });

            Ok(())
        }

        /// VULNERABILITY 6: Removal doesn't refund deposit
        /// Deposit locked forever even after data deleted
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn delete_document_no_refund(
            origin: OriginFor<T>,
            doc_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Remove data but don't unreserve deposit
            // User's funds locked forever
            Documents::<T>::remove(doc_id);

            // Should have: T::Currency::unreserve(&who, deposit);
            // But it's missing!

            Ok(())
        }

        /// VULNERABILITY 7: No storage limit check
        /// Can fill entire chain storage
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn store_unlimited_data(
            origin: OriginFor<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: No maximum size check
            // Attacker can pass Vec with millions of bytes
            // Could fill entire chain storage in one call

            let mut posts = UserPosts::<T>::get(&who);
            posts.push(data);
            UserPosts::<T>::insert(&who, posts);

            Ok(())
        }

        /// VULNERABILITY 8: No cleanup on account deletion
        /// Orphaned data remains forever
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn close_account_no_cleanup(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // VULNERABLE: Remove main data but leave orphans
            UserPosts::<T>::remove(&who);

            // But UserMetadata still exists!
            // Global AllUsers list still has this account
            // Documents they created still exist

            Self::deposit_event(Event::DataCleared { who });

            Ok(())
        }

        /// SAFE EXAMPLE: Proper storage with deposit (commented for reference)
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn store_with_proper_deposit(
            origin: OriginFor<T>,
            content: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // SAFE: Calculate deposit based on storage size
            // Typical: X tokens per byte stored
            let bytes_stored = content.len() as u32;
            let deposit_per_byte: BalanceOf<T> = 100u32.into();
            let required_deposit = deposit_per_byte.saturating_mul(bytes_stored.into());

            // Reserve proportional deposit
            T::Currency::reserve(&who, required_deposit)?;

            // Limit maximum size
            const MAX_SIZE: usize = 100_000; // 100KB max
            ensure!(content.len() <= MAX_SIZE, Error::<T>::DocumentTooLarge);

            // Store with bounded collection
            let doc_id = DocumentCounter::<T>::get();
            Documents::<T>::insert(doc_id, content.clone());
            DocumentCounter::<T>::put(doc_id + 1);

            Self::deposit_event(Event::DocumentStored {
                id: doc_id,
                size: content.len() as u32
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// SAFE EXAMPLE: Storage best practices
        ///
        /// 1. Always require deposits proportional to storage size
        /// 2. Set maximum sizes for variable-length data
        /// 3. Use BoundedVec instead of Vec for storage
        /// 4. Implement cleanup mechanisms (on_finalize, garbage collection)
        /// 5. Refund deposits when storage is freed
        /// 6. Use ChildStorage for large/temporary data
        /// 7. Implement pagination for lists
        /// 8. Regular audits of storage growth
        ///
        /// Example with BoundedVec:
        /// ```
        /// use frame_support::BoundedVec;
        ///
        /// #[pallet::storage]
        /// pub type BoundedPosts<T: Config> = StorageMap<
        ///     _,
        ///     Blake2_128Concat,
        ///     T::AccountId,
        ///     BoundedVec<Vec<u8>, ConstU32<100>> // Max 100 posts
        /// >;
        /// ```
        ///
        /// Deposit calculation example:
        /// ```
        /// const DEPOSIT_PER_BYTE: Balance = 1_000;
        /// let deposit = data.len() as Balance * DEPOSIT_PER_BYTE;
        /// T::Currency::reserve(&who, deposit)?;
        /// ```
        pub fn _safe_storage_example() {}
    }
}

// Educational Notes:
//
// Real-world impact of unsafe storage:
//
// 1. State bloat: Chain storage grows unbounded, expensive to run nodes
// 2. DoS: Attackers fill storage cheaply, making chain unusable
// 3. Economic attacks: Free storage => unfair advantage
// 4. Performance degradation: Large storage slows down all operations
// 5. Database corruption: Orphaned data, memory leaks
//
// Polkadot storage economics:
// - Storage has real cost (SSD space, memory, sync time)
// - Deposits ensure users pay for what they use
// - Typical: 1 DOT per KB of storage
// - Refunds when storage freed encourage cleanup
//
// Best practices:
// 1. BoundedVec/BoundedBTreeMap for all collections
// 2. Deposits proportional to storage footprint
// 3. Cleanup hooks (on_finalize, on_initialize)
// 4. Maximum size limits on all inputs
// 5. Refund mechanisms for freed storage
// 6. Storage migration tools
// 7. Monitoring dashboards for storage growth
//
// References:
// - Polkadot Top 10: https://security.parity.io/top
// - Storage best practices: https://docs.substrate.io/build/runtime-storage/
// - Deposits: https://docs.substrate.io/build/tx-weights-fees/
