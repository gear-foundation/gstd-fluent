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

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), (), ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), (), (), ()) = self.fields;
        ProgramGenerator::create_program(code_id, payload, 0)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), (), GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), (), (), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas(code_id, payload, gas_limit, 0)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), DelayW, ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), (), DelayW(delay), ()) = self.fields;
        ProgramGenerator::create_program_delayed(code_id, payload, 0, delay)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), DelayW, GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), (), DelayW(delay), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas_delayed(code_id, payload, gas_limit, 0, delay)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, ValueW, (), ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), ValueW(value), (), ()) = self.fields;
        ProgramGenerator::create_program(code_id, payload, value)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, ValueW, (), GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), ValueW(value), (), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas(code_id, payload, gas_limit, value)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, ValueW, DelayW, ())> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), ValueW(value), DelayW(delay), ()) = self.fields;
        ProgramGenerator::create_program_delayed(code_id, payload, value, delay)
    }
}

impl<Buffer: AsRef<[u8]>> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, ValueW, DelayW, GasLimitW)> {
    pub fn execute(self) -> Result<(MessageId, ActorId)> {
        let (CodeIdW(code_id), PayloadBytesW(payload), ValueW(value), DelayW(delay), GasLimitW(gas_limit)) = self.fields;
        ProgramGenerator::create_program_with_gas_delayed(code_id, payload, gas_limit, value, delay)
    }
}
