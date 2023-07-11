use crate::{common::*, generated::*};

pub fn create_program<Buffer: AsRef<[u8]>>(
    code_id: CodeId,
    payload: Buffer,
) -> CreateProgramBuilder<(CodeIdW, PayloadBytesW<Buffer>, (), (), ())> {
    CreateProgramBuilder::bytes(code_id, payload)
}
