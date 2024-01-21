use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Default, PartialEq)]
pub struct OpCode {
    pub code: u32,
    pub input_arguments: u32,

    // SWAP1 is 1, SWAP5 is 5 etc
    pub operator_index: usize,

    pub stack_inputs: u32,
    pub stack_outputs: u32,
    pub short_name: String,
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(0x{:02x})", self.short_name, self.code)
    }
}

pub fn opcodes() -> HashMap<u32, OpCode> {
    let mut map = HashMap::new();
    map.insert(
        STOP,
        OpCode {
            code: STOP,
            short_name: "STOP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        ADD,
        OpCode {
            code: ADD,
            short_name: "ADD".to_string(),
            stack_inputs: 2,
            stack_outputs: 1,
            ..Default::default()
        },
    );
    map.insert(
        MUL,
        OpCode {
            code: MUL,
            short_name: "MUL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SUB,
        OpCode {
            code: SUB,
            short_name: "SUB".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DIV,
        OpCode {
            code: DIV,
            short_name: "DIV".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SDIV,
        OpCode {
            code: SDIV,
            short_name: "SDIV".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MOD,
        OpCode {
            code: MOD,
            short_name: "MOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SMOD,
        OpCode {
            code: SMOD,
            short_name: "SMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        ADDMOD,
        OpCode {
            code: ADDMOD,
            short_name: "ADDMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MULMOD,
        OpCode {
            code: MULMOD,
            short_name: "MULMOD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EXP,
        OpCode {
            code: EXP,
            short_name: "EXP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SIGNEXTEND,
        OpCode {
            code: SIGNEXTEND,
            short_name: "SIGNEXTEND".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        LT,
        OpCode {
            code: LT,
            short_name: "LT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        GT,
        OpCode {
            code: GT,
            short_name: "GT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SLT,
        OpCode {
            code: SLT,
            short_name: "SLT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SGT,
        OpCode {
            code: SGT,
            short_name: "SGT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EQ,
        OpCode {
            code: EQ,
            short_name: "EQ".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        ISZERO,
        OpCode {
            code: ISZERO,
            short_name: "ISZERO".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        AND,
        OpCode {
            code: AND,
            short_name: "AND".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        OR,
        OpCode {
            code: OR,
            short_name: "OR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        XOR,
        OpCode {
            code: XOR,
            short_name: "XOR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        NOT,
        OpCode {
            code: NOT,
            short_name: "NOT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        BYTE,
        OpCode {
            code: BYTE,
            short_name: "BYTE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SHL,
        OpCode {
            code: SHL,
            short_name: "SHL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SHR,
        OpCode {
            code: SHR,
            short_name: "SHR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SAR,
        OpCode {
            code: SAR,
            short_name: "SAR".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SHA3,
        OpCode {
            code: 0x22,
            short_name: "SHA3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        ADDRESS,
        OpCode {
            code: ADDRESS,
            short_name: "ADDRESS".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        BALANCE,
        OpCode {
            code: BALANCE,
            short_name: "BALANCE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        ORIGIN,
        OpCode {
            code: ORIGIN,
            short_name: "ORIGIN".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLER,
        OpCode {
            code: CALLER,
            short_name: "CALLER".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLVALUE,
        OpCode {
            code: CALLVALUE,
            short_name: "CALLVALUE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLDATALOAD,
        OpCode {
            code: CALLDATALOAD,
            short_name: "CALLDATALOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLDATASIZE,
        OpCode {
            code: CALLDATASIZE,
            short_name: "CALLDATASIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLDATACOPY,
        OpCode {
            code: CALLDATACOPY,
            short_name: "CALLDATACOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CODESIZE,
        OpCode {
            code: CODESIZE,
            short_name: "CODESIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CODECOPY,
        OpCode {
            code: CODECOPY,
            short_name: "CODECOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        GASPRICE,
        OpCode {
            code: GASPRICE,
            short_name: "GASPRICE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EXTCODESIZE,
        OpCode {
            code: EXTCODESIZE,
            short_name: "EXTCODESIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EXTCODECOPY,
        OpCode {
            code: EXTCODECOPY,
            short_name: "EXTCODECOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        RETURNDATASIZE,
        OpCode {
            code: RETURNDATASIZE,
            short_name: "RETURNDATASIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        RETURNDATACOPY,
        OpCode {
            code: RETURNDATACOPY,
            short_name: "RETURNDATACOPY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EXTCODEHASH,
        OpCode {
            code: EXTCODEHASH,
            short_name: "EXTCODEHASH".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        BLOCKHASH,
        OpCode {
            code: BLOCKHASH,
            short_name: "BLOCKHASH".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        COINBASE,
        OpCode {
            code: COINBASE,
            short_name: "COINBASE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        TIMESTAMP,
        OpCode {
            code: TIMESTAMP,
            short_name: "TIMESTAMP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        NUMBER,
        OpCode {
            code: NUMBER,
            short_name: "NUMBER".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DIFFICULTY,
        OpCode {
            code: DIFFICULTY,
            short_name: "DIFFICULTY".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        GASLIMIT,
        OpCode {
            code: GASLIMIT,
            short_name: "GASLIMIT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CHAINID,
        OpCode {
            code: CHAINID,
            short_name: "CHAINID".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SELFBALANCE,
        OpCode {
            code: SELFBALANCE,
            short_name: "SELFBALANCE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        BASEFEE,
        OpCode {
            code: BASEFEE,
            short_name: "BASEFEE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        POP,
        OpCode {
            code: POP,
            short_name: "POP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MLOAD,
        OpCode {
            code: MLOAD,
            short_name: "MLOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MSTORE,
        OpCode {
            code: MSTORE,
            short_name: "MSTORE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MSTORE8,
        OpCode {
            code: MSTORE8,
            short_name: "MSTORE8".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SLOAD,
        OpCode {
            code: SLOAD,
            short_name: "SLOAD".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SSTORE,
        OpCode {
            code: SSTORE,
            short_name: "SSTORE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        JUMP,
        OpCode {
            code: JUMP,
            short_name: "JUMP".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        JUMPI,
        OpCode {
            code: JUMPI,
            short_name: "JUMPI".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        PC,
        OpCode {
            code: PC,
            short_name: "PC".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        MSIZE,
        OpCode {
            code: MSIZE,
            short_name: "MSIZE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        GAS,
        OpCode {
            code: GAS,
            short_name: "GAS".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        JUMPDEST,
        OpCode {
            code: JUMPDEST,
            short_name: "JUMPDEST".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        PUSH1,
        OpCode {
            code: PUSH1,
            short_name: "PUSH1".to_string(),
            input_arguments: 1,
            ..Default::default()
        },
    );
    map.insert(
        PUSH2,
        OpCode {
            code: PUSH2,
            short_name: "PUSH2".to_string(),
            input_arguments: 2,
            ..Default::default()
        },
    );
    map.insert(
        PUSH3,
        OpCode {
            code: PUSH3,
            short_name: "PUSH3".to_string(),
            input_arguments: 3,
            ..Default::default()
        },
    );
    map.insert(
        PUSH4,
        OpCode {
            code: PUSH4,
            short_name: "PUSH4".to_string(),
            input_arguments: 4,
            ..Default::default()
        },
    );
    map.insert(
        PUSH5,
        OpCode {
            code: PUSH5,
            short_name: "PUSH5".to_string(),
            input_arguments: 5,
            ..Default::default()
        },
    );
    map.insert(
        PUSH6,
        OpCode {
            code: PUSH6,
            short_name: "PUSH6".to_string(),
            input_arguments: 6,
            ..Default::default()
        },
    );
    map.insert(
        PUSH7,
        OpCode {
            code: PUSH7,
            short_name: "PUSH7".to_string(),
            input_arguments: 7,
            ..Default::default()
        },
    );
    map.insert(
        PUSH8,
        OpCode {
            code: PUSH8,
            short_name: "PUSH8".to_string(),
            input_arguments: 8,
            ..Default::default()
        },
    );
    map.insert(
        PUSH9,
        OpCode {
            code: PUSH9,
            short_name: "PUSH9".to_string(),
            input_arguments: 9,
            ..Default::default()
        },
    );
    map.insert(
        PUSH10,
        OpCode {
            code: PUSH10,
            short_name: "PUSH10".to_string(),
            input_arguments: 10,
            ..Default::default()
        },
    );
    map.insert(
        PUSH11,
        OpCode {
            code: PUSH11,
            short_name: "PUSH11".to_string(),
            input_arguments: 11,
            ..Default::default()
        },
    );
    map.insert(
        PUSH12,
        OpCode {
            code: PUSH12,
            short_name: "PUSH12".to_string(),
            input_arguments: 12,
            ..Default::default()
        },
    );
    map.insert(
        PUSH13,
        OpCode {
            code: PUSH13,
            short_name: "PUSH13".to_string(),
            input_arguments: 13,
            ..Default::default()
        },
    );
    map.insert(
        PUSH14,
        OpCode {
            code: PUSH14,
            short_name: "PUSH14".to_string(),
            input_arguments: 14,
            ..Default::default()
        },
    );
    map.insert(
        PUSH15,
        OpCode {
            code: PUSH15,
            short_name: "PUSH15".to_string(),
            input_arguments: 15,
            ..Default::default()
        },
    );
    map.insert(
        PUSH16,
        OpCode {
            code: PUSH16,
            short_name: "PUSH16".to_string(),
            input_arguments: 16,
            ..Default::default()
        },
    );
    map.insert(
        PUSH17,
        OpCode {
            code: PUSH17,
            short_name: "PUSH17".to_string(),
            input_arguments: 17,
            ..Default::default()
        },
    );
    map.insert(
        PUSH18,
        OpCode {
            code: PUSH18,
            short_name: "PUSH18".to_string(),
            input_arguments: 18,
            ..Default::default()
        },
    );
    map.insert(
        PUSH19,
        OpCode {
            code: PUSH19,
            short_name: "PUSH19".to_string(),
            input_arguments: 19,
            ..Default::default()
        },
    );
    map.insert(
        PUSH20,
        OpCode {
            code: PUSH20,
            short_name: "PUSH20".to_string(),
            input_arguments: 20,
            ..Default::default()
        },
    );
    map.insert(
        PUSH21,
        OpCode {
            code: PUSH21,
            short_name: "PUSH21".to_string(),
            input_arguments: 21,
            ..Default::default()
        },
    );
    map.insert(
        PUSH22,
        OpCode {
            code: PUSH22,
            short_name: "PUSH22".to_string(),
            input_arguments: 22,
            ..Default::default()
        },
    );
    map.insert(
        PUSH23,
        OpCode {
            code: PUSH23,
            short_name: "PUSH23".to_string(),
            input_arguments: 23,
            ..Default::default()
        },
    );
    map.insert(
        PUSH24,
        OpCode {
            code: PUSH24,
            short_name: "PUSH24".to_string(),
            input_arguments: 24,
            ..Default::default()
        },
    );
    map.insert(
        PUSH25,
        OpCode {
            code: PUSH25,
            short_name: "PUSH25".to_string(),
            input_arguments: 25,
            ..Default::default()
        },
    );
    map.insert(
        PUSH26,
        OpCode {
            code: PUSH26,
            short_name: "PUSH26".to_string(),
            input_arguments: 26,
            ..Default::default()
        },
    );
    map.insert(
        PUSH27,
        OpCode {
            code: PUSH27,
            short_name: "PUSH27".to_string(),
            input_arguments: 27,
            ..Default::default()
        },
    );
    map.insert(
        PUSH28,
        OpCode {
            code: PUSH28,
            short_name: "PUSH28".to_string(),
            input_arguments: 28,
            ..Default::default()
        },
    );
    map.insert(
        PUSH29,
        OpCode {
            code: PUSH29,
            short_name: "PUSH29".to_string(),
            input_arguments: 29,
            ..Default::default()
        },
    );
    map.insert(
        PUSH30,
        OpCode {
            code: PUSH30,
            short_name: "PUSH30".to_string(),
            input_arguments: 30,
            ..Default::default()
        },
    );
    map.insert(
        PUSH31,
        OpCode {
            code: PUSH31,
            short_name: "PUSH31".to_string(),
            input_arguments: 31,
            ..Default::default()
        },
    );
    map.insert(
        PUSH32,
        OpCode {
            code: PUSH32,
            short_name: "PUSH32".to_string(),
            input_arguments: 32,
            ..Default::default()
        },
    );
    map.insert(
        DUP1,
        OpCode {
            code: DUP1,
            short_name: "DUP1".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP2,
        OpCode {
            code: DUP2,
            short_name: "DUP2".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP3,
        OpCode {
            code: DUP3,
            short_name: "DUP3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP4,
        OpCode {
            code: DUP4,
            short_name: "DUP4".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP5,
        OpCode {
            code: DUP5,
            short_name: "DUP5".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP6,
        OpCode {
            code: DUP6,
            short_name: "DUP6".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP7,
        OpCode {
            code: DUP7,
            short_name: "DUP7".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP8,
        OpCode {
            code: DUP8,
            short_name: "DUP8".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP9,
        OpCode {
            code: DUP9,
            short_name: "DUP9".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP10,
        OpCode {
            code: DUP10,
            short_name: "DUP10".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP11,
        OpCode {
            code: DUP11,
            short_name: "DUP11".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP12,
        OpCode {
            code: DUP12,
            short_name: "DUP12".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP13,
        OpCode {
            code: DUP13,
            short_name: "DUP13".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP14,
        OpCode {
            code: DUP14,
            short_name: "DUP14".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP15,
        OpCode {
            code: DUP15,
            short_name: "DUP15".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DUP16,
        OpCode {
            code: DUP16,
            short_name: "DUP16".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        SWAP1,
        OpCode {
            code: SWAP1,
            short_name: "SWAP1".to_string(),
            operator_index: 1,
            ..Default::default()
        },
    );
    map.insert(
        SWAP2,
        OpCode {
            code: SWAP2,
            short_name: "SWAP2".to_string(),
            operator_index: 2,
            ..Default::default()
        },
    );
    map.insert(
        SWAP3,
        OpCode {
            code: SWAP3,
            short_name: "SWAP3".to_string(),
            operator_index: 3,
            ..Default::default()
        },
    );
    map.insert(
        SWAP4,
        OpCode {
            code: SWAP4,
            short_name: "SWAP4".to_string(),
            operator_index: 4,
            ..Default::default()
        },
    );
    map.insert(
        SWAP5,
        OpCode {
            code: SWAP5,
            short_name: "SWAP5".to_string(),
            operator_index: 5,
            ..Default::default()
        },
    );
    map.insert(
        SWAP6,
        OpCode {
            code: SWAP6,
            short_name: "SWAP6".to_string(),
            operator_index: 6,
            ..Default::default()
        },
    );
    map.insert(
        SWAP7,
        OpCode {
            code: SWAP7,
            short_name: "SWAP7".to_string(),
            operator_index: 7,
            ..Default::default()
        },
    );
    map.insert(
        SWAP8,
        OpCode {
            code: SWAP8,
            short_name: "SWAP8".to_string(),
            operator_index: 8,
            ..Default::default()
        },
    );
    map.insert(
        SWAP9,
        OpCode {
            code: SWAP9,
            short_name: "SWAP9".to_string(),
            operator_index: 9,
            ..Default::default()
        },
    );
    map.insert(
        SWAP10,
        OpCode {
            code: SWAP10,
            short_name: "SWAP10".to_string(),
            operator_index: 10,
            ..Default::default()
        },
    );
    map.insert(
        SWAP11,
        OpCode {
            code: SWAP11,
            short_name: "SWAP11".to_string(),
            operator_index: 11,
            ..Default::default()
        },
    );
    map.insert(
        SWAP12,
        OpCode {
            code: SWAP12,
            short_name: "SWAP12".to_string(),
            operator_index: 12,
            ..Default::default()
        },
    );
    map.insert(
        SWAP13,
        OpCode {
            code: SWAP13,
            short_name: "SWAP13".to_string(),
            operator_index: 13,
            ..Default::default()
        },
    );
    map.insert(
        SWAP14,
        OpCode {
            code: SWAP14,
            short_name: "SWAP14".to_string(),
            operator_index: 14,
            ..Default::default()
        },
    );
    map.insert(
        SWAP15,
        OpCode {
            code: SWAP15,
            short_name: "SWAP15".to_string(),
            operator_index: 15,
            ..Default::default()
        },
    );
    map.insert(
        SWAP16,
        OpCode {
            code: SWAP16,
            short_name: "SWAP16".to_string(),
            operator_index: 16,
            ..Default::default()
        },
    );
    map.insert(
        LOG0,
        OpCode {
            code: LOG0,
            short_name: "LOG0".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        LOG1,
        OpCode {
            code: LOG1,
            short_name: "LOG1".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        LOG2,
        OpCode {
            code: LOG2,
            short_name: "LOG2".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        LOG3,
        OpCode {
            code: LOG3,
            short_name: "LOG3".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        LOG4,
        OpCode {
            code: LOG4,
            short_name: "LOG4".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CREATE,
        OpCode {
            code: CREATE,
            short_name: "CREATE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALL,
        OpCode {
            code: CALL,
            short_name: "CALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CALLCODE,
        OpCode {
            code: CALLCODE,
            short_name: "CALLCODE".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        RETURN,
        OpCode {
            code: RETURN,
            short_name: "RETURN".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        DELEGATECALL,
        OpCode {
            code: DELEGATECALL,
            short_name: "DELEGATECALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        CREATE2,
        OpCode {
            code: CREATE2,
            short_name: "CALLBLACKBOX".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        STATICCALL,
        OpCode {
            code: STATICCALL,
            short_name: "STATICCALL".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        REVERT,
        OpCode {
            code: REVERT,
            short_name: "REVERT".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        INVALID,
        OpCode {
            code: INVALID,
            short_name: "INVALID".to_string(),
            ..Default::default()
        },
    );
    map.insert(
        EOFMAGIC,
        OpCode {
            code: EOFMAGIC,
            short_name: "EOFMAGIC".to_string(),
            ..Default::default()
        },
    );

    map.insert(
        SELFDESTRUCT,
        OpCode {
            code: SELFDESTRUCT,
            short_name: "SELFDESTRUCT".to_string(),
            ..Default::default()
        },
    );
    map
}

pub(crate) const STOP: u32 = 0x00;
pub(crate) const ADD: u32 = 0x01;
pub(crate) const MUL: u32 = 0x02;
pub(crate) const SUB: u32 = 0x03;
pub(crate) const DIV: u32 = 0x04;
pub(crate) const SDIV: u32 = 0x05;
pub(crate) const MOD: u32 = 0x06;
pub(crate) const SMOD: u32 = 0x07;
pub(crate) const ADDMOD: u32 = 0x08;
pub(crate) const MULMOD: u32 = 0x09;
pub(crate) const EXP: u32 = 0x0a;
pub(crate) const SIGNEXTEND: u32 = 0x0b;
pub(crate) const LT: u32 = 0x10;
pub(crate) const GT: u32 = 0x11;
pub(crate) const SLT: u32 = 0x12;
pub(crate) const SGT: u32 = 0x13;
pub(crate) const EQ: u32 = 0x14;
pub(crate) const ISZERO: u32 = 0x15;
pub(crate) const AND: u32 = 0x16;
pub(crate) const OR: u32 = 0x17;
pub(crate) const XOR: u32 = 0x18;
pub(crate) const NOT: u32 = 0x19;
pub(crate) const BYTE: u32 = 0x1a;
pub(crate) const CALLDATALOAD: u32 = 0x35;
pub(crate) const CALLDATASIZE: u32 = 0x36;
pub(crate) const CALLDATACOPY: u32 = 0x37;
pub(crate) const CODESIZE: u32 = 0x38;
pub(crate) const CODECOPY: u32 = 0x39;
pub(crate) const SHL: u32 = 0x1b;
pub(crate) const SHR: u32 = 0x1c;
pub(crate) const SAR: u32 = 0x1d;
pub(crate) const POP: u32 = 0x50;
pub(crate) const MLOAD: u32 = 0x51;
pub(crate) const MSTORE: u32 = 0x52;
pub(crate) const MSTORE8: u32 = 0x53;
pub(crate) const JUMP: u32 = 0x56;
pub(crate) const JUMPI: u32 = 0x57;
pub(crate) const PC: u32 = 0x58;
pub(crate) const MSIZE: u32 = 0x59;
pub(crate) const JUMPDEST: u32 = 0x5b;
pub(crate) const PUSH0: u32 = 0x5f;
pub(crate) const PUSH1: u32 = 0x60;
pub(crate) const PUSH2: u32 = 0x61;
pub(crate) const PUSH3: u32 = 0x62;
pub(crate) const PUSH4: u32 = 0x63;
pub(crate) const PUSH5: u32 = 0x64;
pub(crate) const PUSH6: u32 = 0x65;
pub(crate) const PUSH7: u32 = 0x66;
pub(crate) const PUSH8: u32 = 0x67;
pub(crate) const PUSH9: u32 = 0x68;
pub(crate) const PUSH10: u32 = 0x69;
pub(crate) const PUSH11: u32 = 0x6a;
pub(crate) const PUSH12: u32 = 0x6b;
pub(crate) const PUSH13: u32 = 0x6c;
pub(crate) const PUSH14: u32 = 0x6d;
pub(crate) const PUSH15: u32 = 0x6e;
pub(crate) const PUSH16: u32 = 0x6f;
pub(crate) const PUSH17: u32 = 0x70;
pub(crate) const PUSH18: u32 = 0x71;
pub(crate) const PUSH19: u32 = 0x72;
pub(crate) const PUSH20: u32 = 0x73;
pub(crate) const PUSH21: u32 = 0x74;
pub(crate) const PUSH22: u32 = 0x75;
pub(crate) const PUSH23: u32 = 0x76;
pub(crate) const PUSH24: u32 = 0x77;
pub(crate) const PUSH25: u32 = 0x78;
pub(crate) const PUSH26: u32 = 0x79;
pub(crate) const PUSH27: u32 = 0x7a;
pub(crate) const PUSH28: u32 = 0x7b;
pub(crate) const PUSH29: u32 = 0x7c;
pub(crate) const PUSH30: u32 = 0x7d;
pub(crate) const PUSH31: u32 = 0x7e;
pub(crate) const PUSH32: u32 = 0x7f;
pub(crate) const DUP1: u32 = 0x80;
pub(crate) const DUP2: u32 = 0x81;
pub(crate) const DUP3: u32 = 0x82;
pub(crate) const DUP4: u32 = 0x83;
pub(crate) const DUP5: u32 = 0x84;
pub(crate) const DUP6: u32 = 0x85;
pub(crate) const DUP7: u32 = 0x86;
pub(crate) const DUP8: u32 = 0x87;
pub(crate) const DUP9: u32 = 0x88;
pub(crate) const DUP10: u32 = 0x89;
pub(crate) const DUP11: u32 = 0x8a;
pub(crate) const DUP12: u32 = 0x8b;
pub(crate) const DUP13: u32 = 0x8c;
pub(crate) const DUP14: u32 = 0x8d;
pub(crate) const DUP15: u32 = 0x8e;
pub(crate) const DUP16: u32 = 0x8f;
pub(crate) const SWAP1: u32 = 0x90;
pub(crate) const SWAP2: u32 = 0x91;
pub(crate) const SWAP3: u32 = 0x92;
pub(crate) const SWAP4: u32 = 0x93;
pub(crate) const SWAP5: u32 = 0x94;
pub(crate) const SWAP6: u32 = 0x95;
pub(crate) const SWAP7: u32 = 0x96;
pub(crate) const SWAP8: u32 = 0x97;
pub(crate) const SWAP9: u32 = 0x98;
pub(crate) const SWAP10: u32 = 0x99;
pub(crate) const SWAP11: u32 = 0x9a;
pub(crate) const SWAP12: u32 = 0x9b;
pub(crate) const SWAP13: u32 = 0x9c;
pub(crate) const SWAP14: u32 = 0x9d;
pub(crate) const SWAP15: u32 = 0x9e;
pub(crate) const SWAP16: u32 = 0x9f;
pub(crate) const RETURN: u32 = 0xf3;
pub(crate) const REVERT: u32 = 0xfd;
pub(crate) const INVALID: u32 = 0xfe;
pub(crate) const EOFMAGIC: u32 = 0xef;
pub(crate) const SHA3: u32 = 0x20;
pub(crate) const ADDRESS: u32 = 0x30;
pub(crate) const BALANCE: u32 = 0x31;
pub(crate) const SELFBALANCE: u32 = 0x47;
pub(crate) const BASEFEE: u32 = 0x48;
pub(crate) const ORIGIN: u32 = 0x32;
pub(crate) const CALLER: u32 = 0x33;
pub(crate) const CALLVALUE: u32 = 0x34;
pub(crate) const GASPRICE: u32 = 0x3a;
pub(crate) const EXTCODESIZE: u32 = 0x3b;
pub(crate) const EXTCODECOPY: u32 = 0x3c;
pub(crate) const EXTCODEHASH: u32 = 0x3f;
pub(crate) const RETURNDATASIZE: u32 = 0x3d;
pub(crate) const RETURNDATACOPY: u32 = 0x3e;
pub(crate) const BLOCKHASH: u32 = 0x40;
pub(crate) const COINBASE: u32 = 0x41;
pub(crate) const TIMESTAMP: u32 = 0x42;
pub(crate) const NUMBER: u32 = 0x43;
pub(crate) const DIFFICULTY: u32 = 0x44;
pub(crate) const GASLIMIT: u32 = 0x45;
pub(crate) const SLOAD: u32 = 0x54;
pub(crate) const SSTORE: u32 = 0x55;
pub(crate) const GAS: u32 = 0x5a;
pub(crate) const LOG0: u32 = 0xa0;
pub(crate) const LOG1: u32 = 0xa1;
pub(crate) const LOG2: u32 = 0xa2;
pub(crate) const LOG3: u32 = 0xa3;
pub(crate) const LOG4: u32 = 0xa4;
pub(crate) const CREATE: u32 = 0xf0;
pub(crate) const CALL: u32 = 0xf1;
pub(crate) const CALLCODE: u32 = 0xf2;
pub(crate) const DELEGATECALL: u32 = 0xf4;
pub(crate) const CREATE2: u32 = 0xf5;
pub(crate) const STATICCALL: u32 = 0xfa;
pub(crate) const SELFDESTRUCT: u32 = 0xff;
pub(crate) const CHAINID: u32 = 0x46;
