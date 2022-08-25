//! # Kickback Pallet
//!
//! This pallet implements the [Kickback protocol](https://github.com/wearekickback) for meeting management via skin-in-the-game incentives.
//!
//! ## Overview
//!
//! To be admitted to an meeting, an per-meeting deposit must be reserved from an account.
//!
//! ### Terminology
//! - [`MeetingDetails`]: Metadata of a specific meeting.
//! - [`Meetings`]: a [`BoundedVec`] of meeting details that is limited to [`MaxMeetingCount`] in
//!   size.
//! - [`DepositorsOf`]: a [`BoundedVec`] of accounts that have deposited for a specific meeting that is
//!   limited to [`MaxAttendeeCount`] in size.
//!
//! ## Considerations
//!
//! **Note:** No meeting details are stored on-chain, only the token deposits and incentives are
//! reasoned about in this pallet.
//!
//! All [`MeetingDetails`] include arbitrary meeting details that must be available on an [IPFS](https://ipfs.io) network and have a valid [`Cid`] to access them.
//! This cannot be mutated once established for a meeting to ensure that all attendees have a common
//! view of the meeting's information (time, location, requirements other than a deposit to be
//! admitted, etc.). Most users will want this information to point to a static dApp
//! that is integrated with this pallet and the network it's hosted on to RSVP for a specific
//! meeting. This could also simply be linking to a JSON file that could be consumed in a dApp to
//! interact with this pallet.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod types;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use frame_support::traits::{Currency, ReservableCurrency};

pub use pallet::*;
pub use types::*;

/// The balance type of this pallet.
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Identifier for a specific Meeting.
		type MeetingId: Parameter
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// The currency used for deposits.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The maximum number of meetings.
		#[pallet::constant]
		type MaxMeetingCount: Get<u32>;

		/// The maximum number of attendees per meeting.
		#[pallet::constant]
		type MaxAttendeeCount: Get<u32>;
	}

	// TODO: is this a useful storage item for lookup? Or just use Meetings
	// /// The current set of active meetings.
	// #[pallet::storage]
	// #[pallet::getter(fn active_meetings)]
	// pub type ActiveMeetings<T: Config> =
	// 	StorageValue<_, BoundedVec<T::MeetingId, T::MaxMeetingCount>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn meetings)]
	/// The list of all unsettled meeting details.
	pub type Meetings<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::MeetingId,
		MeetingDetails<BalanceOf<T>, T::AccountId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn depositors_of)]
	/// All accounts that have deposited fund for a specific meeting.
	pub type DepositorsOf<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::MeetingId,
		BoundedVec<AttendeeDetails<T::AccountId>, T::MaxAttendeeCount>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// A deposit for a meeting has been reserved for and account. [who, meeting]
		RsvpDeposit { meeting: T::MeetingId, who: T::AccountId, deposit: BalanceOf<T> },
		}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The signing account has no permission to do the operation.
		NoPermission,
		/// The signing account has already RSVPEd for this meeting.
		AlreadyRsvped,
		/// The meeting has reached capacity.
		TooManyRsvped,
		/// The given MeetingID is unknown.
		Unknown,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Cancel a meeting, refunding all those that RSVPed. Origin of this call must be from the
		/// `host`.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(origin: OriginFor<T>, details: MeetingDetails<T, T::AccountId>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.

			Ok(())
		}

		/// Return the deposit (if any) of an asset account.
		///
		/// The origin must be Signed.
		///
		/// - `id`: The identifier of the asset for the account to be created.
		/// - `allow_burn`: If `true` then assets may be destroyed in order to complete the refund.
		///
		/// Emits `Refunded` event when successful.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn refund(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::MeetingId,
			who: T::AccountId,
		) -> DispatchResult {
			Ok(())
		}
		/// Return the deposit (if any) of an asset account.
		///
		/// The origin must be Signed.
		///
		/// - `id`: The identifier of the asset for the account to be created.
		/// - `allow_burn`: If `true` then assets may be destroyed in order to complete the refund.
		///
		/// Emits `Refunded` event when successful.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn rsvp(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::MeetingId,
			who: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if let Some(meet) = <Meetings<T>>::get(id) {
				let deposit = meet.deposit;
				// deposited is a vec of AttendeeDetails
				<DepositorsOf<T>>::try_mutate(&meet, |depositors| ->{
					DispatchResult {
							let pos = depositors.binary_search_by_key(&who, |&(a, b)| who).err().ok_or(Error::<T>::AlreadyRsvped)?;
							attendees
								.try_insert(pos, who.clone())
								.map_err(|_| Error::<T>::TooManyRsvped)?;
							Ok(())
					})?;
					
					Self::deposit_event(Event::RsvpDeposit { meeting: id.into(), who: who.into(), deposit: deposit.into()});
				}
			}

			Ok(())
		}
		/// Cancel a meeting, refunding all those that RSVPed. Origin of this call must be from the
		/// `host`.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn cancel(origin: OriginFor<T>, id: T::MeetingId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Update storage.

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {

		/// Account deposits funds and is added to the Deposited list.
		fn do_rsvp(who &T::AccountId, depositors: &BoundedVec<AttendeeDetails<T::AccountId>, T::MaxAttendeeCount>, deposit: &BalanceOf<T>){
			T::Currency::reserve(&who, deposit)?;

		}
}