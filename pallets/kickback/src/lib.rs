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
//! - [`DepositorsOf`]: a [`BoundedVec`] of accounts that have deposited for a specific meeting that
//!   is limited to [`MaxAttendeeCount`] in size.
//! - `Attest`: to attest to a specific attendee being present at an event.
//! - `Settle`: to
//!
//! ## Considerations
//!
//! **Note:** No meeting details are stored on-chain, only the token deposits and incentives are
//! reasoned about in this pallet.
//!
//! All [`MeetingDetails`] include arbitrary meeting details that must be available on an [IPFS](https://ipfs.io) network and have a valid [`Cid`] to access them.
//! The [`MeetingDetails`]  cannot be mutated once established for a meeting to ensure that all
//! attendees have a common view of the meeting's information (deposit, time, location, additional
//! requirements to a deposit to be admitted, etc.). Most users will want this information to point
//! to a static dApp that is integrated with this pallet and the network it's hosted on to RSVP for
//! a specific meeting. This could also simply be linking to a JSON file that could be consumed in a
//! dApp to interact with this pallet.
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

	/// The list of all unsettled meeting details.
	#[pallet::storage]
	#[pallet::getter(fn meetings)]
	pub type Meetings<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::MeetingId,
		MeetingDetails<BalanceOf<T>, T::AccountId>,
		OptionQuery,
	>;

	/// The list of all those RSVPed to a meeting.
	#[pallet::storage]
	#[pallet::getter(fn rsvped)]
	pub type Rsvped<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::MeetingId,
		BoundedVec<T::AccountId, T::MaxAttendeeCount>,
		ValueQuery,
	>;

	/// The list of all host confirmed attendees to a meeting.
	#[pallet::storage]
	#[pallet::getter(fn attended)]
	pub type Attended<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::MeetingId,
		BoundedVec<T::AccountId, T::MaxAttendeeCount>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// A deposit for a meeting has been reserved for and account. [who, meeting]
		DepositReceived { meeting: T::MeetingId, attendee: T::AccountId, deposit: BalanceOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The signing account has no permission to do the operation.
		NoPermission,
		/// The meeting doesn't exist.
		MeetingNotFound,
		/// The account balance is insufficient for the required deposit.
		InsufficientFunds,
		/// The signing account has already RSVPEd for this meeting.
		AlreadyRsvped,
		/// The meeting has reached capacity.
		CapacityReached,
		/// The given MeetingId is unknown.
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
		pub fn create(
			origin: OriginFor<T>,
			details: MeetingDetails<BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
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
			id: T::MeetingId,
			who: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Update storage.

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

		/// Return the deposit (if any) of an asset account.
		///
		/// The origin must be Signed.
		///
		/// - `id`: The identifier of the asset for the account to be created.
		/// - `allow_burn`: If `true` then assets may be destroyed in order to complete the refund.
		///
		/// Emits `Refunded` event when successful.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn rsvp(origin: OriginFor<T>, id: T::MeetingId) -> DispatchResult {
			let attendee = ensure_signed(origin)?;
			let meeting = <Meetings<T>>::get(id).ok_or(Error::<T>::MeetingNotFound)?;
				let deposit = meeting.deposit;
				Self::do_rsvp(&id, &attendee, deposit)?; //
				Self::deposit_event(Event::DepositReceived { meeting: id, attendee: attendee, deposit: deposit });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Reserve a specific meeting deposit for an attendee and add them to the RSVP list.
	fn do_rsvp(id: &T::MeetingId, attendee: &T::AccountId, deposit: BalanceOf<T>) -> DispatchResult {
		// TODO: consider new storage item for list of deposits, something like:
		// <DepositOf<T>>::insert(&who, deposit);
		<Rsvped<T>>::try_mutate(id, |rsvps| -> DispatchResult {
			let pos = rsvps.binary_search(attendee).err().ok_or(Error::<T>::AlreadyRsvped)?;

			rsvps
				.try_insert(pos, attendee.clone())
				.map_err(|_| Error::<T>::CapacityReached)?; // TODO capacity needs a check that it's less than BoundedVec max MaxAttendeeCount
				
			T::Currency::reserve(&attendee, deposit)
				.map_err(|_| Error::<T>::InsufficientFunds)?;

			Ok(())
		})?;

		Ok(())
	}

	// /// Remove a user from the alliance member set.
	// fn remove_member(who: &T::AccountId, role: MemberRole) -> DispatchResult {
	// 	<Members<T, I>>::try_mutate(role, |members| -> DispatchResult {
	// 		let pos = members.binary_search(who).ok().ok_or(Error::<T, I>::NotMember)?;
	// 		members.remove(pos);
	// 		Ok(())
	// 	})?;

	// 	if matches!(role, MemberRole::Founder | MemberRole::Fellow) {
	// 		let members = Self::votable_members_sorted();
	// 		T::MembershipChanged::change_members_sorted(&[], &[who.clone()], &members[..]);
	// 	}
	// 	Ok(())
	// }
}
