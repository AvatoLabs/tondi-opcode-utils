use tondi_txscript::opcodes::codes::*;

pub const OP_PUSH_DATA1: u8 = 0x4c;
pub const OP_PUSH_DATA2: u8 = 0x4d;
pub const OP_PUSH_DATA4: u8 = 0x4e;

// HACK: we do not have access to the internal code value from the Opcode struct. 
// Workaround: compare its class to determine if it is one of the OP_PUSH_BYTES opcodes.
pub fn is_push_bytes(code: u8) -> bool {
    (0x01..=0x4b).contains(&code)
}

// HACK: we do not have access to the internal code value from the Opcode struct. 
// Workaround: compare the opcode with the existing three OP_PUSHDATA opcodes.
pub fn is_push_data(code: u8) -> bool {
    matches!(code, OP_PUSH_DATA1 | OP_PUSH_DATA2 | OP_PUSH_DATA4)
}
macro_rules! opcode_from_str {
    ($($op:ident => $val:expr, $doc:expr);*) => {
        type Err = ();
        pub fn from_str(s: &str) -> Result<u8, Err> {
            match s {
                // 别名
                "Op0" => Ok(OpFalse),
                "OpTrue" | "TRUE" => Ok(OpTrue),
                "OpFalse" | "FALSE" => Ok(OpFalse),
                "Op1" => Ok(OpTrue),
                "OpNEG1" | "NEG1" => Ok(Op1Negate),
                "OpNOP2" | "NOP2" => Ok(OpCheckLockTimeVerify),
                "OpNOP3" | "NOP3" => Ok(OpCheckSequenceVerify),
                "Op2" => Ok(Op2),
                "Op3" => Ok(Op3),
                "Op4" => Ok(Op4),
                "Op5" => Ok(Op5),
                "Op6" => Ok(Op6),
                "Op7" => Ok(Op7),
                "Op8" => Ok(Op8),
                "Op9" => Ok(Op9),
                "Op10" => Ok(Op10),
                "Op11" => Ok(Op11),
                "Op12" => Ok(Op12),
                "Op13" => Ok(Op13),
                "Op14" => Ok(Op14),
                "Op15" => Ok(Op15),
                "Op16" => Ok(Op16),
                $(
                    // 同时支持带/不带 Op 前缀两种写法
                    s if s == stringify!($op) || s == &stringify!($op)[2..] => Ok($op),
                )+
                _ => Err(()),
            }
        }
    }
}


opcode_from_str!{
    OpFalse => 0x00, "Push an empty array onto the stack.";
    OpData1 => 0x01, "Push the next byte as an array onto the stack.";
    OpData2 => 0x02, "Push the next 2 bytes as an array onto the stack.";
    OpData3 => 0x03, "Push the next 3 bytes as an array onto the stack.";
    OpData4 => 0x04, "Push the next 4 bytes as an array onto the stack.";
    OpData5 => 0x05, "Push the next 5 bytes as an array onto the stack.";
    OpData6 => 0x06, "Push the next 6 bytes as an array onto the stack.";
    OpData7 => 0x07, "Push the next 7 bytes as an array onto the stack.";
    OpData8 => 0x08, "Push the next 8 bytes as an array onto the stack.";
    OpData9 => 0x09, "Push the next 9 bytes as an array onto the stack.";
    OpData10 => 0x0a, "Push the next 10 bytes as an array onto the stack.";
    OpData11 => 0x0b, "Push the next 11 bytes as an array onto the stack.";
    OpData12 => 0x0c, "Push the next 12 bytes as an array onto the stack.";
    OpData13 => 0x0d, "Push the next 13 bytes as an array onto the stack.";
    OpData14 => 0x0e, "Push the next 14 bytes as an array onto the stack.";
    OpData15 => 0x0f, "Push the next 15 bytes as an array onto the stack.";
    OpData16 => 0x10, "Push the next 16 bytes as an array onto the stack.";
    OpData17 => 0x11, "Push the next 17 bytes as an array onto the stack.";
    OpData18 => 0x12, "Push the next 18 bytes as an array onto the stack.";
    OpData19 => 0x13, "Push the next 19 bytes as an array onto the stack.";
    OpData20 => 0x14, "Push the next 20 bytes as an array onto the stack.";
    OpData21 => 0x15, "Push the next 21 bytes as an array onto the stack.";
    OpData22 => 0x16, "Push the next 22 bytes as an array onto the stack.";
    OpData23 => 0x17, "Push the next 23 bytes as an array onto the stack.";
    OpData24 => 0x18, "Push the next 24 bytes as an array onto the stack.";
    OpData25 => 0x19, "Push the next 25 bytes as an array onto the stack.";
    OpData26 => 0x1a, "Push the next 26 bytes as an array onto the stack.";
    OpData27 => 0x1b, "Push the next 27 bytes as an array onto the stack.";
    OpData28 => 0x1c, "Push the next 28 bytes as an array onto the stack.";
    OpData29 => 0x1d, "Push the next 29 bytes as an array onto the stack.";
    OpData30 => 0x1e, "Push the next 30 bytes as an array onto the stack.";
    OpData31 => 0x1f, "Push the next 31 bytes as an array onto the stack.";
    OpData32 => 0x20, "Push the next 32 bytes as an array onto the stack.";
    OpData33 => 0x21, "Push the next 33 bytes as an array onto the stack.";
    OpData34 => 0x22, "Push the next 34 bytes as an array onto the stack.";
    OpData35 => 0x23, "Push the next 35 bytes as an array onto the stack.";
    OpData36 => 0x24, "Push the next 36 bytes as an array onto the stack.";
    OpData37 => 0x25, "Push the next 37 bytes as an array onto the stack.";
    OpData38 => 0x26, "Push the next 38 bytes as an array onto the stack.";
    OpData39 => 0x27, "Push the next 39 bytes as an array onto the stack.";
    OpData40 => 0x28, "Push the next 40 bytes as an array onto the stack.";
    OpData41 => 0x29, "Push the next 41 bytes as an array onto the stack.";
    OpData42 => 0x2a, "Push the next 42 bytes as an array onto the stack.";
    OpData43 => 0x2b, "Push the next 43 bytes as an array onto the stack.";
    OpData44 => 0x2c, "Push the next 44 bytes as an array onto the stack.";
    OpData45 => 0x2d, "Push the next 45 bytes as an array onto the stack.";
    OpData46 => 0x2e, "Push the next 46 bytes as an array onto the stack.";
    OpData47 => 0x2f, "Push the next 47 bytes as an array onto the stack.";
    OpData48 => 0x30, "Push the next 48 bytes as an array onto the stack.";
    OpData49 => 0x31, "Push the next 49 bytes as an array onto the stack.";
    OpData50 => 0x32, "Push the next 50 bytes as an array onto the stack.";
    OpData51 => 0x33, "Push the next 51 bytes as an array onto the stack.";
    OpData52 => 0x34, "Push the next 52 bytes as an array onto the stack.";
    OpData53 => 0x35, "Push the next 53 bytes as an array onto the stack.";
    OpData54 => 0x36, "Push the next 54 bytes as an array onto the stack.";
    OpData55 => 0x37, "Push the next 55 bytes as an array onto the stack.";
    OpData56 => 0x38, "Push the next 56 bytes as an array onto the stack.";
    OpData57 => 0x39, "Push the next 57 bytes as an array onto the stack.";
    OpData58 => 0x3a, "Push the next 58 bytes as an array onto the stack.";
    OpData59 => 0x3b, "Push the next 59 bytes as an array onto the stack.";
    OpData60 => 0x3c, "Push the next 60 bytes as an array onto the stack.";
    OpData61 => 0x3d, "Push the next 61 bytes as an array onto the stack.";
    OpData62 => 0x3e, "Push the next 62 bytes as an array onto the stack.";
    OpData63 => 0x3f, "Push the next 63 bytes as an array onto the stack.";
    OpData64 => 0x40, "Push the next 64 bytes as an array onto the stack.";
    OpData65 => 0x41, "Push the next 65 bytes as an array onto the stack.";
    OpData66 => 0x42, "Push the next 66 bytes as an array onto the stack.";
    OpData67 => 0x43, "Push the next 67 bytes as an array onto the stack.";
    OpData68 => 0x44, "Push the next 68 bytes as an array onto the stack.";
    OpData69 => 0x45, "Push the next 69 bytes as an array onto the stack.";
    OpData70 => 0x46, "Push the next 70 bytes as an array onto the stack.";
    OpData71 => 0x47, "Push the next 71 bytes as an array onto the stack.";
    OpData72 => 0x48, "Push the next 72 bytes as an array onto the stack.";
    OpData73 => 0x49, "Push the next 73 bytes as an array onto the stack.";
    OpData74 => 0x4a, "Push the next 74 bytes as an array onto the stack.";
    OpData75 => 0x4b, "Push the next 75 bytes as an array onto the stack.";
    OP_PUSH_DATA1 => 0x4c, "Read the next byte as N; push the next N bytes as an array onto the stack.";
    OP_PUSH_DATA2 => 0x4d, "Read the next 2 bytes as N; push the next N bytes as an array onto the stack.";
    OP_PUSH_DATA4 => 0x4e, "Read the next 4 bytes as N; push the next N bytes as an array onto the stack.";
    Op1Negate => 0x4f, "Push the array `0x81` onto the stack.";
    OpReserved => 0x50, "Synonym for OpReturn.";
    OpTrue => 0x51, "Push the array `0x01` onto the stack.";
    Op2 => 0x52, "Push the array `0x02` onto the stack.";
    Op3 => 0x53, "Push the array `0x03` onto the stack.";
    Op4 => 0x54, "Push the array `0x04` onto the stack.";
    Op5 => 0x55, "Push the array `0x05` onto the stack.";
    Op6 => 0x56, "Push the array `0x06` onto the stack.";
    Op7 => 0x57, "Push the array `0x07` onto the stack.";
    Op8 => 0x58, "Push the array `0x08` onto the stack.";
    Op9 => 0x59, "Push the array `0x09` onto the stack.";
    Op10 => 0x5a, "Push the array `0x0a` onto the stack.";
    Op11 => 0x5b, "Push the array `0x0b` onto the stack.";
    Op12 => 0x5c, "Push the array `0x0c` onto the stack.";
    Op13 => 0x5d, "Push the array `0x0d` onto the stack.";
    Op14 => 0x5e, "Push the array `0x0e` onto the stack.";
    Op15 => 0x5f, "Push the array `0x0f` onto the stack.";
    Op16 => 0x60, "Push the array `0x10` onto the stack.";
    OpNop => 0x61, "Does nothing.";
    OpVer => 0x62, "Synonym for OpReturn.";
    OpIf => 0x63, "Pop and execute the next statements if a nonzero element was popped.";
    OpNotIf => 0x64, "Pop and execute the next statements if a zero element was popped.";
    OpVerIf => 0x65, "Fail the script unconditionally, does not even need to be executed.";
    OpVerNotIf => 0x66, "Fail the script unconditionally, does not even need to be executed.";
    OpElse => 0x67, "Execute statements if those after the previous OpIf were not, and vice-versa. \
             If there is no previous OpIf, this acts as a RETURN.";
    OpEndIf => 0x68, "Ends the if/else block.";
    OpVerify => 0x69, "If the top value is zero or the stack is empty, fail; otherwise, pop the stack.";
    OpReturn => 0x6a, "Fail the script immediately. (Must be executed.).";
    OpToAltStack => 0x6b, "Pop one element from the main stack onto the alt stack.";
    OpFromAltStack => 0x6c, "Pop one element from the alt stack onto the main stack.";
    Op2Drop => 0x6d, "Drops the top two stack items.";
    Op2Dup => 0x6e, "Duplicates the top two stack items as AB -> ABAB.";
    Op3Dup => 0x6f, "Duplicates the two three stack items as ABC -> ABCABC.";
    Op2Over => 0x70, "Copies the two stack items of items two spaces back to the front, as xxAB -> ABxxAB.";
    Op2Rot => 0x71, "Moves the two stack items four spaces back to the front, as xxxxAB -> ABxxxx.";
    Op2Swap => 0x72, "Swaps the top two pairs, as ABCD -> CDAB.";
    OpIfDup => 0x73, "Duplicate the top stack element unless it is zero.";
    OpDepth => 0x74, "Push the current number of stack items onto the stack.";
    OpDrop => 0x75, "Drops the top stack item.";
    OpDup => 0x76, "Duplicates the top stack item.";
    OpNip => 0x77, "Drops the second-to-top stack item.";
    OpOver => 0x78, "Copies the second-to-top stack item, as xA -> AxA.";
    OpPick => 0x79, "Pop the top stack element as N. Copy the Nth stack element to the top.";
    OpRoll => 0x7a, "Pop the top stack element as N. Move the Nth stack element to the top.";
    OpRot => 0x7b, "Rotate the top three stack items, as [top next1 next2] -> [next2 top next1].";
    OpSwap => 0x7c, "Swap the top two stack items.";
    OpTuck => 0x7d, "Copy the top stack item to before the second item, as [top next] -> [top next top].";
    OpCat => 0x7e, "Concatenates two arrays.";
    OpSubStr => 0x7f, "Returns a section of an array.";
    OpLeft => 0x80, "Keeps only bytes left of the specified point in an array.";
    OpRight => 0x81, "Keeps only bytes right of the specified point in an array.";
    OpSize => 0x82, "Pushes the length of the top stack item onto the stack.";
    OpInvert => 0x83, "Flips all of the bits in the input.";
    OpAnd => 0x84, "Boolean and between each bit in the inputs.";
    OpOr => 0x85, "Boolean or between each bit in the inputs.";
    OpXor => 0x86, "Boolean exclusive or between each bit in the inputs.";
    OpEqual => 0x87, "Pushes 1 if the inputs are exactly equal, 0 otherwise.";
    OpEqualVerify => 0x88, "Returns success if the inputs are exactly equal, failure otherwise.";
    OpReserved1 => 0x89, "Synonym for OpReturn.";
    OpReserved2 => 0x8a, "Synonym for OpReturn.";
    Op1Add => 0x8b, "Increment the top stack element in place.";
    Op1Sub => 0x8c, "Decrement the top stack element in place.";
    Op2Mul => 0x8d, "The input is multiplied by 2.";
    Op2Div => 0x8e, "The input is divided by 2.";
    OpNegate => 0x8f, "Multiply the top stack item by -1 in place.";
    OpAbs => 0x90, "Absolute value the top stack item in place.";
    OpNot => 0x91, "Map 0 to 1 and everything else to 0, in place.";
    Op0NotEqual => 0x92, "Map 0 to 0 and everything else to 1, in place.";
    OpAdd => 0x93, "Pop two stack items and push their sum.";
    OpSub => 0x94, "Pop two stack items and push the second minus the top.";
    OpMul => 0x95, "Pop two stack items and push their product.";
    OpDiv => 0x96, "Pop two stack items and push the second divided by the top.";
    OpMod => 0x97, "Pop two stack items and push the modulo.";
    OpLShift => 0x98, "Shifts the second item left by first item bits.";
    OpRShift => 0x99, "Shifts the second item right by first item bits.";
    OpBoolAnd => 0x9a, "Pop the top two stack items and push 1 if both are nonzero, else push 0.";
    OpBoolOr => 0x9b, "Pop the top two stack items and push 1 if either is nonzero, else push 0.";
    OpNumEqual => 0x9c, "Pop the top two stack items and push 1 if both are numerically equal, else push 0.";
    OpNumEqualVerify => 0x9d, "Pop the top two stack items and return success if both are numerically equal, else return failure.";
    OpNumNotEqual => 0x9e, "Pop the top two stack items and push 0 if both are numerically equal, else push 1.";
    OpLessThan => 0x9f, "Pop the top two items; push 1 if the second is less than the top, 0 otherwise.";
    OpGreaterThan => 0xa0, "Pop the top two items; push 1 if the second is greater than the top, 0 otherwise.";
    OpLessThanOrEqual => 0xa1, "Pop the top two items; push 1 if the second is <= the top, 0 otherwise.";
    OpGreaterThanOrEqual => 0xa2, "Pop the top two items; push 1 if the second is >= the top, 0 otherwise.";
    OpMin => 0xa3, "Pop the top two items; push the smaller.";
    OpMax => 0xa4, "Pop the top two items; push the larger.";
    OpWithin => 0xa5, "Pop the top three items; if the top is >= the second and < the third, push 1, otherwise push 0.";
    OpUnknown166 => 0xa6, "Reserved Undefined opcodes.";
    OpUnknown167 => 0xa7, "Reserved Undefined opcodes.";
    OpSHA256 => 0xa8, "Pop the top stack item and push its SHA256 hash.";
    OpCheckMultiSigECDSA => 0xa9, "ECDSA multisig verification.";
    OpBlake3 => 0xaa, "Pop the top stack item and push its Blake3 hash.";
    OpCheckSigECDSA => 0xab, "ECDSA signature verification.";
    OpCheckSig => 0xac, "Signature verification pushing 1/0 for success/failure.";
    OpCheckSigVerify => 0xad, "Signature verification returning success/failure.";
    OpCheckMultiSig => 0xae, "Multisig verification pushing 1/0 for success/failure.";
    OpCheckMultiSigVerify => 0xaf, "Multisig verification returning success/failure.";
    OpCheckLockTimeVerify => 0xb0, "Check lock time verification.";
    OpCheckSequenceVerify => 0xb1, "Check sequence verification.";
    // OpUnknown178 => 0xb2, "Reserved opcode.";
    // OpUnknown179 => 0xb3, "Reserved opcode.";
    // OpUnknown180 => 0xb4, "Reserved opcode.";
    // OpUnknown181 => 0xb5, "Reserved opcode.";
    // OpUnknown182 => 0xb6, "Reserved opcode.";
    // OpUnknown183 => 0xb7, "Reserved opcode.";
    // OpUnknown184 => 0xb8, "Reserved opcode.";
    // OpUnknown185 => 0xb9, "Reserved opcode.";
    // OpUnknown186 => 0xba, "Reserved opcode.";
    // OpUnknown187 => 0xbb, "Reserved opcode.";
    // OpUnknown188 => 0xbc, "Reserved opcode.";
    // OpUnknown189 => 0xbd, "Reserved opcode.";
    // OpUnknown190 => 0xbe, "Reserved opcode.";
    // OpUnknown191 => 0xbf, "Reserved opcode.";
    // OpUnknown192 => 0xc0, "Reserved opcode.";
    // OpUnknown193 => 0xc1, "Reserved opcode.";
    // OpUnknown194 => 0xc2, "Reserved opcode.";
    // OpUnknown195 => 0xc3, "Reserved opcode.";
    OpUnknown196 => 0xc4, "Reserved opcode.";
    OpUnknown197 => 0xc5, "Reserved opcode.";
    OpUnknown198 => 0xc6, "Reserved opcode.";
    // OpUnknown199 => 0xc7, "Reserved opcode.";
    // OpUnknown200 => 0xc8, "Reserved opcode.";
    // OpUnknown201 => 0xc9, "Reserved opcode.";
    // OpUnknown202 => 0xca, "Reserved opcode.";
    // OpUnknown203 => 0xcb, "Reserved opcode.";
    // OpUnknown204 => 0xcc, "Reserved opcode.";
    // OpUnknown205 => 0xcd, "Reserved opcode.";
    // OpUnknown206 => 0xce, "Reserved opcode.";
    // OpUnknown207 => 0xcf, "Reserved opcode.";
    // OpUnknown208 => 0xd0, "Reserved opcode.";
    // OpUnknown209 => 0xd1, "Reserved opcode.";
    // OpUnknown210 => 0xd2, "Reserved opcode.";
    // OpUnknown211 => 0xd3, "Reserved opcode.";
    // OpUnknown212 => 0xd4, "Reserved opcode.";
    // OpUnknown213 => 0xd5, "Reserved opcode.";
    // OpUnknown214 => 0xd6, "Reserved opcode.";
    // OpUnknown215 => 0xd7, "Reserved opcode.";
    // OpUnknown216 => 0xd8, "Reserved opcode.";
    // OpUnknown217 => 0xd9, "Reserved opcode.";
    // OpUnknown218 => 0xda, "Reserved opcode.";
    // OpUnknown219 => 0xdb, "Reserved opcode.";
    // OpUnknown220 => 0xdc, "Reserved opcode.";
    // OpUnknown221 => 0xdd, "Reserved opcode.";
    // OpUnknown222 => 0xde, "Reserved opcode.";
    // OpUnknown223 => 0xdf, "Reserved opcode.";
    // OpUnknown224 => 0xe0, "Reserved opcode.";
    // OpUnknown225 => 0xe1, "Reserved opcode.";
    // OpUnknown226 => 0xe2, "Reserved opcode.";
    // OpUnknown227 => 0xe3, "Reserved opcode.";
    // OpUnknown228 => 0xe4, "Reserved opcode.";
    // OpUnknown229 => 0xe5, "Reserved opcode.";
    // OpUnknown230 => 0xe6, "Reserved opcode.";
    // OpUnknown231 => 0xe7, "Reserved opcode.";
    // OpUnknown232 => 0xe8, "Reserved opcode.";
    // OpUnknown233 => 0xe9, "Reserved opcode.";
    // OpUnknown234 => 0xea, "Reserved opcode.";
    // OpUnknown235 => 0xeb, "Reserved opcode.";
    // OpUnknown236 => 0xec, "Reserved opcode.";
    // OpUnknown237 => 0xed, "Reserved opcode.";
    // OpUnknown238 => 0xee, "Reserved opcode.";
    // OpUnknown239 => 0xef, "Reserved opcode.";
    // OpUnknown240 => 0xf0, "Reserved opcode.";
    // OpUnknown241 => 0xf1, "Reserved opcode.";
    // OpUnknown242 => 0xf2, "Reserved opcode.";
    // OpUnknown243 => 0xf3, "Reserved opcode.";
    // OpUnknown244 => 0xf4, "Reserved opcode.";
    // OpUnknown245 => 0xf5, "Reserved opcode.";
    // OpUnknown246 => 0xf6, "Reserved opcode.";
    // OpUnknown247 => 0xf7, "Reserved opcode.";
    // OpUnknown248 => 0xf8, "Reserved opcode.";
    // OpUnknown249 => 0xf9, "Reserved opcode.";
    OpSmallInteger => 0xfa, "Reserved opcode.";
    OpPubKeys => 0xfb, "Reserved opcode.";
    OpUnknown252 => 0xfc, "Reserved opcode.";
    OpPubKeyHash => 0xfd, "Reserved opcode.";
    OpPubKey => 0xfe, "Reserved opcode.";
    OpInvalidOpCode => 0xff, "Synonym for OpReturn."
}
