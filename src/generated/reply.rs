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

impl<Payload: GasFromReservationMarker, Value, GasLimit> ReplyBuilder<(Payload, Value, (), GasLimit)> {
    pub fn with_gas_from_reservation(self, reservation_id: ReservationId) -> ReplyBuilder<(Payload, Value, ReservationIdW, GasLimit)> {
        let (payload, value, _, gas_limit) = self.fields;
        ReplyBuilder {
            fields: (payload, value, ReservationIdW(reservation_id), gas_limit),
        }
    }
}

impl<Payload, Value, ReservationId> ReplyBuilder<(Payload, Value, ReservationId, ())> {
    pub fn with_gas_limit(self, gas_limit: u64) -> ReplyBuilder<(Payload, Value, ReservationId, GasLimitW)> {
        let (payload, value, reservation_id, _) = self.fields;
        ReplyBuilder {
            fields: (payload, value, reservation_id, GasLimitW(gas_limit)),
        }
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), (), (), ()) = self.fields;
        reply_bytes(payload, 0)
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, (), (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), (), (), GasLimitW(gas_limit)) = self.fields;
        reply_bytes_with_gas(payload, gas_limit, 0)
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, (), ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), (), ReservationIdW(reservation_id), ()) = self.fields;
        reply_bytes_from_reservation(reservation_id, payload, 0)
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, ValueW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), ValueW(value), (), ()) = self.fields;
        reply_bytes(payload, value)
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, ValueW, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), ValueW(value), (), GasLimitW(gas_limit)) = self.fields;
        reply_bytes_with_gas(payload, gas_limit, value)
    }
}

impl<Buffer: AsRef<[u8]>> ReplyBuilder<(PayloadBytesW<Buffer>, ValueW, ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadBytesW(payload), ValueW(value), ReservationIdW(reservation_id), ()) = self.fields;
        reply_bytes_from_reservation(reservation_id, payload, value)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), (), (), ()) = self.fields;
        reply(payload, 0)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, (), (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), (), (), GasLimitW(gas_limit)) = self.fields;
        reply_with_gas(payload, gas_limit, 0)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, (), ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), (), ReservationIdW(reservation_id), ()) = self.fields;
        reply_from_reservation(reservation_id, payload, 0)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, ValueW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), ValueW(value), (), ()) = self.fields;
        reply(payload, value)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, ValueW, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), ValueW(value), (), GasLimitW(gas_limit)) = self.fields;
        reply_with_gas(payload, gas_limit, value)
    }
}

impl<Encodable: Encode> ReplyBuilder<(PayloadEncodableW<Encodable>, ValueW, ReservationIdW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadEncodableW(payload), ValueW(value), ReservationIdW(reservation_id), ()) = self.fields;
        reply_from_reservation(reservation_id, payload, value)
    }
}

impl<Range: RangeBounds<usize>> ReplyBuilder<(PayloadInputW<Range>, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), (), (), ()) = self.fields;
        reply_input(0, payload)
    }
}

impl<Range: RangeBounds<usize>> ReplyBuilder<(PayloadInputW<Range>, (), (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), (), (), GasLimitW(gas_limit)) = self.fields;
        reply_input_with_gas(gas_limit, 0, payload)
    }
}

impl<Range: RangeBounds<usize>> ReplyBuilder<(PayloadInputW<Range>, ValueW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), ValueW(value), (), ()) = self.fields;
        reply_input(value, payload)
    }
}

impl<Range: RangeBounds<usize>> ReplyBuilder<(PayloadInputW<Range>, ValueW, (), GasLimitW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (PayloadInputW(payload), ValueW(value), (), GasLimitW(gas_limit)) = self.fields;
        reply_input_with_gas(gas_limit, value, payload)
    }
}
