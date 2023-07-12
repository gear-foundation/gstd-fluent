#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::common::*;

// ---------------------------------------------------------------------------------------------- //
// bindings for `create_program*`
// CreateProgramBuilder<(CodeId, Payload, Value, Delay, GasLimit)>
// ---------------------------------------------------------------------------------------------- //

pub struct CreateProgramBuilder<Fields = ((), (), (), (), ())> {
    fields: Fields,
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), (), ())> {
    pub fn bytes(code_id: CodeId, payload: Buffer) -> Self {
        Self {
            fields: (CodeIdW(code_id), PayloadBytesW(payload), (), (), ()),
        }
    }
}

impl<CodeId, Payload, Delay, GasLimit> CreateProgramBuilder<(CodeId, Payload, (), Delay, GasLimit)> {
    pub fn with_value(self, value: u128) -> CreateProgramBuilder<(CodeId, Payload, ValueW, Delay, GasLimit)> {
        let (code_id, payload, _, delay, gas_limit) = self.fields;
        CreateProgramBuilder {
            fields: (code_id, payload, ValueW(value), delay, gas_limit),
        }
    }
}

impl<CodeId, Payload, Value, GasLimit> CreateProgramBuilder<(CodeId, Payload, Value, (), GasLimit)> {
    pub fn with_delay(self, delay: u32) -> CreateProgramBuilder<(CodeId, Payload, Value, DelayW, GasLimit)> {
        let (code_id, payload, value, _, gas_limit) = self.fields;
        CreateProgramBuilder {
            fields: (code_id, payload, value, DelayW(delay), gas_limit),
        }
    }
}

impl<CodeId, Payload, Value, Delay> CreateProgramBuilder<(CodeId, Payload, Value, Delay, ())> {
    pub fn with_gas_limit(self, gas_limit: u64) -> CreateProgramBuilder<(CodeId, Payload, Value, Delay, GasLimitW)> {
        let (code_id, payload, value, delay, _) = self.fields;
        CreateProgramBuilder {
            fields: (code_id, payload, value, delay, GasLimitW(gas_limit)),
        }
    }
}

impl<CodeId, Payload, Value, GasLimit> CreateProgramBuilder<(CodeId, Payload, Value, (), GasLimit)> {
    pub fn for_reply(self) -> CreateProgramBuilderForReply<(CodeId, Payload, Value, GasLimit, (), ())> {
        let (code_id, payload, value, _, gas_limit) = self.fields;
        CreateProgramBuilderForReply {
            fields: (code_id, payload, value, gas_limit, (), ()),
        }
    }

    pub fn for_reply_as<Decodable: Decode>(self) -> CreateProgramBuilderForReply<(CodeId, Payload, Value, GasLimit, (), DecodableW<Decodable>)> {
        let (code_id, payload, value, _, gas_limit) = self.fields;
        CreateProgramBuilderForReply {
            fields: (code_id, payload, value, gas_limit, (), DecodableW(PhantomData)),
        }
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, Value, (), ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, (), ()) = self.fields;
        ProgramGenerator::create_program(code_id, payload, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, Value, (), GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, (), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas(code_id, payload, gas_limit, value.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, Value, DelayW, ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, DelayW(delay), ()) = self.fields;
        ProgramGenerator::create_program_delayed(code_id, payload, value.into().0, delay)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, Value, DelayW, GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, DelayW(delay), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas_delayed(code_id, payload, gas_limit, value.into().0, delay)
    }
}

// ---------------------------------------------------------------------------------------------- //
// bindings for `create_program*`: for_reply(), for_reply_as::<Decodable>()
// CreateProgramBuilderForReply<(CodeId, Payload, Value, GasLimit, ReplyDeposit, Decodable)>
// ---------------------------------------------------------------------------------------------- //

pub struct CreateProgramBuilderForReply<Fields = ((), (), (), (), (), ())> {
    fields: Fields,
}

impl<CodeId, Payload, Value, GasLimit, Decodable> CreateProgramBuilderForReply<(CodeId, Payload, Value, GasLimit, (), Decodable)> {
    pub fn with_reply_deposit(self, reply_deposit: u64) -> CreateProgramBuilderForReply<(CodeId, Payload, Value, GasLimit, ReplyDepositW, Decodable)> {
        let (code_id, payload, value, gas_limit, _, decodable) = self.fields;
        CreateProgramBuilderForReply {
            fields: (code_id, payload, value, gas_limit, ReplyDepositW(reply_deposit), decodable),
        }
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> CreateProgramBuilderForReply<(CodeIdW, PayloadBytesW<Buffer>, Value, (), ReplyDeposit, ())> {
    pub fn execute(self) -> Result<CreateProgramFuture> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, (), reply_deposit, ()) = self.fields;
        ProgramGenerator::create_program_for_reply(code_id, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>> CreateProgramBuilderForReply<(CodeIdW, PayloadBytesW<Buffer>, Value, GasLimitW, ReplyDeposit, ())> {
    pub fn execute(self) -> Result<CreateProgramFuture> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, GasLimitW(gas_limit), reply_deposit, ()) = self.fields;
        ProgramGenerator::create_program_with_gas_for_reply(code_id, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> CreateProgramBuilderForReply<(CodeIdW, PayloadBytesW<Buffer>, Value, (), ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecCreateProgramFuture<Decodable>> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, (), reply_deposit, _) = self.fields;
        ProgramGenerator::create_program_for_reply_as(code_id, payload, value.into().0, reply_deposit.into().0)
    }
}

impl<Buffer: AsRef<[u8]>, Value: Into<ValueW>, ReplyDeposit: Into<ReplyDepositW>, Decodable: Decode> CreateProgramBuilderForReply<(CodeIdW, PayloadBytesW<Buffer>, Value, GasLimitW, ReplyDeposit, DecodableW<Decodable>)> {
    pub fn execute(self) -> Result<CodecCreateProgramFuture<Decodable>> {
        let (CodeIdW(code_id), PayloadBytesW(payload), value, GasLimitW(gas_limit), reply_deposit, _) = self.fields;
        ProgramGenerator::create_program_with_gas_for_reply_as(code_id, payload, gas_limit, value.into().0, reply_deposit.into().0)
    }
}
