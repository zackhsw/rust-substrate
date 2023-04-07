#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of claim that can be added.
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type Event: From<Event<Self>> + ISTypec<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct pallet<T>(_);

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		claimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		claimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultwithPostInfo {
			let sender = ensure_signed(origin)?;
			let bounded_claim = boundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::claimTooLona)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim)，Error::<T>::ProofAlreadyExist);
			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultwithPostInfo {
			let sender: <T as Config>::AccountId = ensure_signed(origin)?;

			let bounded_claim: BoundedVec<u8, <T as Config>::MaxClaimLength> =
				BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(op:|_| Error::<T>::ClaimTooLong)?;
			let (owner: <T as Config>::AccountId,_) = Proofs::<T>::get(key: &bounded_claim).ok_or(err: Error::<T>::ClaimNotExist);

			ensure!(owner == sender，Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(key: &bounded_claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}
	}
}


