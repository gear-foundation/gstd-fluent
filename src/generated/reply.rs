#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::common::*;

// ---------------------------------------------------------------------------------------------- //
// bindings for `reply*`
// ReplyBuilder<(Payload, Value, ReservationId, GasLimit)>
// ---------------------------------------------------------------------------------------------- //

pub struct ReplyBuilder<Fields = ((), (), (), ())> {
    fields: Fields,
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, (), (), ())> {
    pub fn bytes(payload: Buffer) -> Self {
        Self {
            fields: (PayloadBytesW(payload), (), (), ()),
        }
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, (), (), ())> {
    pub fn encode(payload: Encodable) -> Self {
        Self {
            fields: (PayloadEncodableW(payload), (), (), ()),
        }
    }
}

impl<Range: RangeBounds<usize>> ReplyBuilder<(PayloadInputW<Range>, (), (), ())> {
    pub fn input(payload: Range) -> Self {
        Self {
            fields: (PayloadInputW(payload), (), (), ()),
        }
    }
}

impl<Payload, ReservationId, GasLimit> ReplyBuilder<(Payload, (), ReservationId, GasLimit)> {
    pub fn with_value(self, value: u128) -> ReplyBuilder<(Payload, ValueW, ReservationId, GasLimit)> {
        let (payload, _, reservation_id, gas_limit) = self.fields;
        ReplyBuilder {
            fields: (payload, ValueW(value), reservation_id, gas_limit),
        }
    }
}

impl<Payload: PayloadWithGasReservationMarker, Value, GasLimit: UnitTypeMarker> ReplyBuilder<(Payload, Value, (), GasLimit)> {
    pub fn with_gas_from_reservation(self, reservation_id: ReservationId) -> ReplyBuilder<(Payload, Value, ReservationIdW, GasLimit)> {
        let (payload, value, _, gas_limit) = self.fields;
        ReplyBuilder {
            fields: (payload, value, ReservationIdW(reservation_id), gas_limit),
        }
    }
}

impl<Payload, Value, ReservationId: UnitTypeMarker> ReplyBuilder<(Payload, Value, ReservationId, ())> {
    pub fn with_gas_limit(self, gas_limit: u64) -> ReplyBuilder<(Payload, Value, ReservationId, GasLimitW)> {
        let (payload, value, reservation_id, _) = self.fields;
        ReplyBuilder {
            fields: (payload, value, reservation_id, GasLimitW(gas_limit)),
        }
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> ReplyBuilder<(PayloadBytesW<Buffer>, Value, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), value, _, _) = self.fields;
        reply_bytes(payload, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> ReplyBuilder<(PayloadBytesW<Buffer>, Value, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), value, _, GasLimitW(gas_limit)) = self.fields;
        reply_bytes_with_gas(payload, gas_limit, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> ReplyBuilder<(PayloadBytesW<Buffer>, Value, ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), value, ReservationIdW(reservation_id), _) = self.fields;
        reply_bytes_from_reservation(reservation_id, payload, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> ReplyBuilder<(PayloadEncodableW<Encodable>, Value, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), value, _, _) = self.fields;
        reply(payload, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> ReplyBuilder<(PayloadEncodableW<Encodable>, Value, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), value, _, GasLimitW(gas_limit)) = self.fields;
        reply_with_gas(payload, gas_limit, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> ReplyBuilder<(PayloadEncodableW<Encodable>, Value, ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), value, ReservationIdW(reservation_id), _) = self.fields;
        reply_from_reservation(reservation_id, payload, value.into().0)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> ReplyBuilder<(PayloadInputW<Range>, Value, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), value, _, _) = self.fields;
        reply_input(value.into().0, payload)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> ReplyBuilder<(PayloadInputW<Range>, Value, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), value, _, GasLimitW(gas_limit)) = self.fields;
        reply_input_with_gas(gas_limit, value.into().0, payload)
    }
}
