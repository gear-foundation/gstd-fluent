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

impl<Program, Payload, Value, GasLimit, ReservationId> SendBuilder<(Program, Payload, Value, (), GasLimit, ReservationId)> {
    pub fn for_reply(self) -> SendBuilderForReply<(Program, Payload, Value, GasLimit, ReservationId, (), ())> {
        let (program, payload, value, _, gas_limit, reservation_id) = self.fields;
        SendBuilderForReply {
            fields: (program, payload, value, gas_limit, reservation_id, (), ()),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn for_reply_as<Decodable: Decode>(self) -> SendBuilderForReply<(Program, Payload, Value, GasLimit, ReservationId, (), DecodableW<Decodable>)> {
        let (program, payload, value, _, gas_limit, reservation_id) = self.fields;
        SendBuilderForReply {
            fields: (program, payload, value, gas_limit, reservation_id, (), DecodableW(PhantomData)),
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

// ---------------------------------------------------------------------------------------------- //
// bindings for `send*`: for_reply(), for_reply_as::<Decodable>()
// SendBuilderForReply<(Program, Payload, Value, GasLimit, ReservationId, ReplyDeposit, Decodable)>
// ---------------------------------------------------------------------------------------------- //

pub struct SendBuilderForReply<Fields = ((), (), (), (), (), ())> {
    fields: Fields,
}

impl<Program, Payload, Value, GasLimit, ReservationId, Decodable> SendBuilderForReply<(Program, Payload, Value, GasLimit, ReservationId, (), Decodable)> {
    pub fn with_reply_deposit(self, reply_deposit: u64) -> SendBuilderForReply<(Program, Payload, Value, GasLimit, ReservationId, ReplyDepositW, Decodable)> {
        let (program, payload, value, gas_limit, reservation_id, _, decodable) = self.fields;
        SendBuilderForReply {
            fields: (program, payload, value, gas_limit, reservation_id, ReplyDepositW(reply_deposit), decodable),
        }
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, (), (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_bytes_for_reply(program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, (), ReservationIdW, ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, ReservationIdW(reservation_id), reply_deposit, _) = self.fields;
        send_bytes_from_reservation_for_reply(reservation_id, program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, GasLimitW, (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadBytesW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_bytes_with_gas_for_reply(program, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, (), (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_for_reply(program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, (), ReservationIdW, ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, ReservationIdW(reservation_id), reply_deposit, _) = self.fields;
        send_from_reservation_for_reply(reservation_id, program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, GasLimitW, (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadEncodableW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_with_gas_for_reply(program, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadInputW<Range>, Value, (), (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadInputW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_input_for_reply(program, value.into().0, payload, reply_deposit.into().0)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> SendBuilderForReply<(ProgramW, PayloadInputW<Range>, Value, GasLimitW, (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<MessageFuture> {
        let (ProgramW(program), PayloadInputW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_input_with_gas_for_reply(program, gas_limit, value.into().0, payload, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, (), (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_bytes_for_reply_as(program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, (), ReservationIdW, ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadBytesW(payload), value, _, ReservationIdW(reservation_id), reply_deposit, _) = self.fields;
        send_bytes_from_reservation_for_reply_as(reservation_id, program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadBytesW<Buffer>, Value, GasLimitW, (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadBytesW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_bytes_with_gas_for_reply_as(program, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, (), (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_for_reply_as(program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, (), ReservationIdW, ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadEncodableW(payload), value, _, ReservationIdW(reservation_id), reply_deposit, _) = self.fields;
        send_from_reservation_for_reply_as(reservation_id, program, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Encodable: Encode, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadEncodableW<Encodable>, Value, GasLimitW, (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadEncodableW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_with_gas_for_reply_as(program, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadInputW<Range>, Value, (), (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadInputW(payload), value, _, _, reply_deposit, _) = self.fields;
        send_input_for_reply_as(program, value.into().0, payload, reply_deposit.into().0)
    }
}

impl<Range: RangeBounds<usize>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> SendBuilderForReply<(ProgramW, PayloadInputW<Range>, Value, GasLimitW, (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecMessageFuture<Decodable>> {
        let (ProgramW(program), PayloadInputW(payload), value, GasLimitW(gas_limit), _, reply_deposit, _) = self.fields;
        send_input_with_gas_for_reply_as(program, gas_limit, value.into().0, payload, reply_deposit.into().0)
    }
}
