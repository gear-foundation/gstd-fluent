#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::common::*;

// ---------------------------------------------------------------------------------------------- //
// bindings for `send*`
// SendBuilder<(Program, Payload, Value, Delay, GasLimit, ReservationId)>
// ---------------------------------------------------------------------------------------------- //

pub struct SendBuilder<Fields = ((), (), (), (), (), ())> {
    fields: Fields,
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), (), ())> {
    pub fn bytes(program: ActorId, payload: Buffer) -> Self {
        Self {
            fields: (ProgramW(program), PayloadBytesW(payload), (), (), (), ()),
        }
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), (), ())> {
    pub fn encode(program: ActorId, payload: Encodable) -> Self {
        Self {
            fields: (ProgramW(program), PayloadEncodableW(payload), (), (), (), ()),
        }
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, (), (), (), ())> {
    pub fn input(program: ActorId, payload: Range) -> Self {
        Self {
            fields: (ProgramW(program), PayloadInputW(payload), (), (), (), ()),
        }
    }
}

impl<Program, Payload, Delay, GasLimit, ReservationId> SendBuilder<(Program, Payload, (), Delay, GasLimit, ReservationId)> {
    pub fn with_value(self, value: u128) -> SendBuilder<(Program, Payload, ValueW, Delay, GasLimit, ReservationId)> {
        let (program, payload, _, delay, gas_limit, reservation_id) = self.fields;
        SendBuilder {
            fields: (program, payload, ValueW(value), delay, gas_limit, reservation_id),
        }
    }
}

impl<Program, Payload, Value, GasLimit, ReservationId> SendBuilder<(Program, Payload, Value, (), GasLimit, ReservationId)> {
    pub fn with_delay(self, delay: u32) -> SendBuilder<(Program, Payload, Value, DelayW, GasLimit, ReservationId)> {
        let (program, payload, value, _, gas_limit, reservation_id) = self.fields;
        SendBuilder {
            fields: (program, payload, value, DelayW(delay), gas_limit, reservation_id),
        }
    }
}

impl<Program, Payload, Value, Delay, ReservationId> SendBuilder<(Program, Payload, Value, Delay, (), ReservationId)> {
    pub fn with_gas_limit(self, gas_limit: u64) -> SendBuilder<(Program, Payload, Value, Delay, GasLimitW, ReservationId)> {
        let (program, payload, value, delay, _, reservation_id) = self.fields;
        SendBuilder {
            fields: (program, payload, value, delay, GasLimitW(gas_limit), reservation_id),
        }
    }
}

impl<Program, Payload: GasFromReservationMarker, Value, Delay, GasLimit> SendBuilder<(Program, Payload, Value, Delay, GasLimit, ())> {
    pub fn with_gas_from_reservation(self, reservation_id: ReservationId) -> SendBuilder<(Program, Payload, Value, Delay, GasLimit, ReservationIdW)> {
        let (program, payload, value, delay, gas_limit, _) = self.fields;
        SendBuilder {
            fields: (program, payload, value, delay, gas_limit, ReservationIdW(reservation_id)),
        }
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), (), (), ()) = self.fields;
        send_bytes(program, payload, 0)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), (), (), ReservationIdW(reservation_id)) = self.fields;
        send_bytes_from_reservation(reservation_id, program, payload, 0)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), (), GasLimitW(gas_limit), ()) = self.fields;
        send_bytes_with_gas(program, payload, gas_limit, 0)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), DelayW(delay), (), ()) = self.fields;
        send_bytes_delayed(program, payload, 0, delay)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), DelayW(delay), (), ReservationIdW(reservation_id)) = self.fields;
        send_bytes_delayed_from_reservation(reservation_id, program, payload, 0, delay)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), (), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_bytes_with_gas_delayed(program, payload, gas_limit, 0, delay)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), (), (), ()) = self.fields;
        send_bytes(program, payload, value)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), (), (), ReservationIdW(reservation_id)) = self.fields;
        send_bytes_from_reservation(reservation_id, program, payload, value)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), (), GasLimitW(gas_limit), ()) = self.fields;
        send_bytes_with_gas(program, payload, gas_limit, value)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), DelayW(delay), (), ()) = self.fields;
        send_bytes_delayed(program, payload, value, delay)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), DelayW(delay), (), ReservationIdW(reservation_id)) = self.fields;
        send_bytes_delayed_from_reservation(reservation_id, program, payload, value, delay)
    }
}

impl<Buffer: AsRef<[u8]>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, ValueW, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), ValueW(value), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_bytes_with_gas_delayed(program, payload, gas_limit, value, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), (), (), ()) = self.fields;
        send(program, payload, 0)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), (), (), ReservationIdW(reservation_id)) = self.fields;
        send_from_reservation(reservation_id, program, payload, 0)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), (), GasLimitW(gas_limit), ()) = self.fields;
        send_with_gas(program, payload, gas_limit, 0)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), DelayW(delay), (), ()) = self.fields;
        send_delayed(program, payload, 0, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), DelayW(delay), (), ReservationIdW(reservation_id)) = self.fields;
        send_delayed_from_reservation(reservation_id, program, payload, 0, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), (), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_with_gas_delayed(program, payload, gas_limit, 0, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), (), (), ()) = self.fields;
        send(program, payload, value)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), (), (), ReservationIdW(reservation_id)) = self.fields;
        send_from_reservation(reservation_id, program, payload, value)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), (), GasLimitW(gas_limit), ()) = self.fields;
        send_with_gas(program, payload, gas_limit, value)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), DelayW(delay), (), ()) = self.fields;
        send_delayed(program, payload, value, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), DelayW(delay), (), ReservationIdW(reservation_id)) = self.fields;
        send_delayed_from_reservation(reservation_id, program, payload, value, delay)
    }
}

impl<Encodable: Encode> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, ValueW, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), ValueW(value), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_with_gas_delayed(program, payload, gas_limit, value, delay)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, (), (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), (), (), (), ()) = self.fields;
        send_input(program, 0, payload)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, (), (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), (), (), GasLimitW(gas_limit), ()) = self.fields;
        send_input_with_gas(program, gas_limit, 0, payload)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, (), DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), (), DelayW(delay), (), ()) = self.fields;
        send_input_delayed(program, 0, payload, delay)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, (), DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), (), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_input_with_gas_delayed(program, gas_limit, 0, payload, delay)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, ValueW, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), ValueW(value), (), (), ()) = self.fields;
        send_input(program, value, payload)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, ValueW, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), ValueW(value), (), GasLimitW(gas_limit), ()) = self.fields;
        send_input_with_gas(program, gas_limit, value, payload)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, ValueW, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), ValueW(value), DelayW(delay), (), ()) = self.fields;
        send_input_delayed(program, value, payload, delay)
    }
}

impl<Range: RangeBounds<usize>> SendBuilder<(ProgramW, PayloadInputW<Range>, ValueW, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), ValueW(value), DelayW(delay), GasLimitW(gas_limit), ()) = self.fields;
        send_input_with_gas_delayed(program, gas_limit, value, payload, delay)
    }
}
