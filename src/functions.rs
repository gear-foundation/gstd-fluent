use crate::{common::*, generated::*};

/// Creates a builder that allows to send a new message as a reply to the message being processed.
pub fn reply_bytes<Buffer: AsRef<[u8]>>(
    payload: Buffer,
) -> ReplyBuilder<(PayloadBytesW<Buffer>, (), (), ())> {
    ReplyBuilder::bytes(payload)
}

/// Creates a builder that allows to send a new message as a reply to the message being processed.
pub fn reply<Encodable: Encode>(
    payload: Encodable,
) -> ReplyBuilder<(PayloadEncodableW<Encodable>, (), (), ())> {
    ReplyBuilder::encode(payload)
}

/// Creates a builder that allows to send a new message as a reply to the message being processed.
pub fn reply_input<Range: RangeBounds<usize>>(
    payload: Range,
) -> ReplyBuilder<(PayloadInputW<Range>, (), (), ())> {
    ReplyBuilder::input(payload)
}

/// Creates a builder that allows to send a new message to a program or user.
pub fn send_bytes<Buffer: AsRef<[u8]>>(
    program: ActorId,
    payload: Buffer,
) -> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), (), ())> {
    SendBuilder::bytes(program, payload)
}

/// Creates a builder that allows to send a new message to a program or user.
pub fn send<Encodable: Encode>(
    program: ActorId,
    payload: Encodable,
) -> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), (), ())> {
    SendBuilder::encode(program, payload)
}

/// Creates a builder that allows to send a new message to a program or user.
pub fn send_input<Range: RangeBounds<usize>>(
    program: ActorId,
    payload: Range,
) -> SendBuilder<(ProgramW, PayloadInputW<Range>, (), (), (), ())> {
    SendBuilder::input(program, payload)
}

/// Creates a builder that allows to create a new program from the already existing on-chain code.
pub fn create_program_bytes<Buffer: AsRef<[u8]>>(
    code_id: CodeId,
    payload: Buffer,
) -> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), (), ())> {
    CreateProgramBuilder::bytes(code_id, payload)
}

/// Creates a builder that allows to create a new program from the already existing on-chain code.
pub fn create_program<Encodable: Encode>(
    code_id: CodeId,
    payload: Encodable,
) -> CreateProgramBuilder<(CodeIdW, PayloadEncodableW<Encodable>, (), (), ())> {
    CreateProgramBuilder::encode(code_id, payload)
}
