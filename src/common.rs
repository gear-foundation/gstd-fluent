pub use core::ops::RangeBounds;
pub use gstd::{
    errors::Result, msg::*, prog::*, ActorId, CodeId, Encode, MessageId, ReservationId,
};

pub struct ProgramW(pub(crate) ActorId);
pub struct CodeIdW(pub(crate) CodeId);
pub struct PayloadBytesW<Buffer: AsRef<[u8]>>(pub(crate) Buffer);
pub struct PayloadEncodableW<Encodable: Encode>(pub(crate) Encodable);
pub struct PayloadInputW<Range: RangeBounds<usize>>(pub(crate) Range);
pub struct ValueW(pub(crate) u128);
pub struct ReservationIdW(pub(crate) ReservationId);
pub struct GasLimitW(pub(crate) u64);
pub struct DelayW(pub(crate) u32);

pub trait GasFromReservationMarker {}

impl<Buffer: AsRef<[u8]>> GasFromReservationMarker for PayloadBytesW<Buffer> {}
impl<Encodable: Encode> GasFromReservationMarker for PayloadEncodableW<Encodable> {}

// Currently the `GasLimit` and `ReservationId` generics do not overlap,
// but in future versions of gstd they may be activated at the same time.
// https://github.com/gear-tech/gear/pull/2705

pub trait GasLimitUnitMarker {} //may be removed soon

impl GasLimitUnitMarker for () {}

pub trait ReservationIdUnitMarker {} //may be removed soon

impl ReservationIdUnitMarker for () {}
