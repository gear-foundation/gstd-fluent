use crate::{common::*, generated::*};

pub fn reply_bytes<Buffer: AsRef<[u8]>>(
    payload: Buffer,
) -> ReplyBuilder<(PayloadBytesW<Buffer>, (), (), ())> {
    ReplyBuilder::bytes(payload)
}

pub fn reply<Encodable: Encode>(
    payload: Encodable,
) -> ReplyBuilder<(PayloadEncodableW<Encodable>, (), (), ())> {
    ReplyBuilder::encode(payload)
}

pub fn reply_input<Range: RangeBounds<usize>>(
    payload: Range,
) -> ReplyBuilder<(PayloadInputW<Range>, (), (), ())> {
    ReplyBuilder::input(payload)
}

pub fn send_bytes<Buffer: AsRef<[u8]>>(
    program: ActorId,
    payload: Buffer,
) -> SendBuilder<(ProgramW, PayloadBytesW<Buffer>, (), (), (), ())> {
    SendBuilder::bytes(program, payload)
}

pub fn send<Encodable: Encode>(
    program: ActorId,
    payload: Encodable,
) -> SendBuilder<(ProgramW, PayloadEncodableW<Encodable>, (), (), (), ())> {
    SendBuilder::encode(program, payload)
}

pub fn send_input<Range: RangeBounds<usize>>(
    program: ActorId,
    payload: Range,
) -> SendBuilder<(ProgramW, PayloadInputW<Range>, (), (), (), ())> {
    SendBuilder::input(program, payload)
}
