pub use core::{marker::PhantomData, ops::RangeBounds};
pub use gstd::{
    errors::Result, msg::*, prog::*, ActorId, CodeId, Decode, Encode, MessageId, ReservationId,
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
pub struct DecodableW<Decodable: Decode>(pub(crate) PhantomData<Decodable>);
pub struct ReplyDepositW(pub(crate) u64);

impl From<()> for ValueW {
    fn from(_: ()) -> Self {
        Self(0)
    }
}

impl From<()> for ReplyDepositW {
    fn from(_: ()) -> Self {
        Self(0)
    }
}

pub trait PayloadWithGasReservationMarker {}

impl<Buffer: AsRef<[u8]>> PayloadWithGasReservationMarker for PayloadBytesW<Buffer> {}
impl<Encodable: Encode> PayloadWithGasReservationMarker for PayloadEncodableW<Encodable> {}

// Currently the `GasLimit` and `ReservationId` generics do not overlap,
// but in future versions of gstd they may be activated at the same time.
// https://github.com/gear-tech/gear/pull/2705

// To be removed soon
pub trait UnitTypeMarker {}

impl UnitTypeMarker for () {}
