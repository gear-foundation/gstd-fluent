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

impl<Program, Payload, Value, Delay, ReservationId: UnitTypeMarker> SendBuilder<(Program, Payload, Value, Delay, (), ReservationId)> {
    pub fn with_gas_limit(self, gas_limit: u64) -> SendBuilder<(Program, Payload, Value, Delay, GasLimitW, ReservationId)> {
        let (program, payload, value, delay, _, reservation_id) = self.fields;
        SendBuilder {
            fields: (program, payload, value, delay, GasLimitW(gas_limit), reservation_id),
        }
    }
}

impl<Program, Payload: PayloadWithGasReservationMarker, Value, Delay, GasLimit: UnitTypeMarker> SendBuilder<(Program, Payload, Value, Delay, GasLimit, ())> {
    pub fn with_gas_from_reservation(self, reservation_id: ReservationId) -> SendBuilder<(Program, Payload, Value, Delay, GasLimit, ReservationIdW)> {
        let (program, payload, value, delay, gas_limit, _) = self.fields;
        SendBuilder {
            fields: (program, payload, value, delay, gas_limit, ReservationIdW(reservation_id)),
        }
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, _, _) = self.fields;
        send_bytes(program, payload, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, _, ReservationIdW(reservation_id)) = self.fields;
        send_bytes_from_reservation(reservation_id, program, payload, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, GasLimitW(gas_limit), _) = self.fields;
        send_bytes_with_gas(program, payload, gas_limit, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, DelayW(delay), _, _) = self.fields;
        send_bytes_delayed(program, payload, value.into().0, delay)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, DelayW(delay), _, ReservationIdW(reservation_id)) = self.fields;
        send_bytes_delayed_from_reservation(reservation_id, program, payload, value.into().0, delay)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, Value, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadBytesW(payload), value, DelayW(delay), GasLimitW(gas_limit), _) = self.fields;
        send_bytes_with_gas_delayed(program, payload, gas_limit, value.into().0, delay)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, _, _) = self.fields;
        send(program, payload, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, (), (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, _, ReservationIdW(reservation_id)) = self.fields;
        send_from_reservation(reservation_id, program, payload, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, GasLimitW(gas_limit), _) = self.fields;
        send_with_gas(program, payload, gas_limit, value.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, DelayW(delay), _, _) = self.fields;
        send_delayed(program, payload, value.into().0, delay)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, DelayW, (), ReservationIdW)> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, DelayW(delay), _, ReservationIdW(reservation_id)) = self.fields;
        send_delayed_from_reservation(reservation_id, program, payload, value.into().0, delay)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, Value, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadEncodableW(payload), value, DelayW(delay), GasLimitW(gas_limit), _) = self.fields;
        send_with_gas_delayed(program, payload, gas_limit, value.into().0, delay)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadInputW<Range>, Value, (), (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), value, _, _, _) = self.fields;
        send_input(program, value.into().0, payload)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadInputW<Range>, Value, (), GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), value, _, GasLimitW(gas_limit), _) = self.fields;
        send_input_with_gas(program, gas_limit, value.into().0, payload)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadInputW<Range>, Value, DelayW, (), ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), value, DelayW(delay), _, _) = self.fields;
        send_input_delayed(program, value.into().0, payload, delay)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>> SendBuilder<(ProgramW, PayloadInputW<Range>, Value, DelayW, GasLimitW, ())> {
    pub fn execute(self) -> Result<MessageId> {
        let (ProgramW(program), PayloadInputW(payload), value, DelayW(delay), GasLimitW(gas_limit), _) = self.fields;
        send_input_with_gas_delayed(program, gas_limit, value.into().0, payload, delay)
    }
}
