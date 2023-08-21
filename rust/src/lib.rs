use std::str::FromStr;

use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    fn new() -> Self {
        Self { memory: Vec::with_capacity(4 * 1024) }
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) {
        if offset + value.len() > self.memory.len() {
            self.memory.resize(offset + value.len(), 0);
        }
        self.memory[offset..(value.len() + offset)].copy_from_slice(value);
    }

    pub fn store8(&mut self, offset: usize, value: u8) {
        if offset >= self.memory.len() {
            self.memory.resize(offset + 1, 0);
        }
        self.memory[offset] = value;
    }

    pub fn load(&self, offset: usize, size: usize) -> [u8; 32] {
        let mut result = [0u8; 32];
        let end = std::cmp::min(self.memory.len(), offset + size);
        result[..end-offset].copy_from_slice(&self.memory[offset..end]);
        result
    }

    pub fn size(&self) -> usize {
        self.memory.len()
    }
}


#[derive(Debug, Deserialize)]
pub struct Tx {
    pub from: Option<String>,
    pub to: Option<String>,
}


#[derive(Debug, Clone)]
pub struct RustEVM {
    memory: Memory,
}

impl RustEVM {
    pub fn new() -> Self {
        RustEVM { memory: Memory::new() }
    }

    pub fn evaluate(mut self, code: &[u8], tx: &Option<Tx>) -> Vec<U256> {

        let mut stack: Vec<U256> = Vec::new();

        let mut opcode: u8 = 0;
        
        let mut pc: usize = 0;

        while pc < code.len() {
                opcode = code[pc];

                println!("starting operation");
                match opcode {
                    STOP => {
                        pc += 1;
                        break;
                    },
                    ADD => {
                        let a = stack.pop();
                        let b = stack.pop();
                        if let (Some(a), Some(b)) = (a, b) {
                            let result = a + b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    POP => {
                        stack.pop();
                    },
                    MUL => {
                        let a = stack.pop();
                        let b = stack.pop();
                        if let (Some(a), Some(b)) = (a, b) {
                            let result = a * b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    SUB => {
                        let a = stack.pop();
                        let b = stack.pop();
                        if let (Some(a), Some(b)) = (a, b) {
                            let result = a - b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    DIV | SDIV => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            if b == U256::from(0) {
                                stack.push(U256::from(0));
                                pc += 1;
                                continue
                            }

                            let result = a / b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    MOD | SMOD => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            if b == U256::from(0) {
                                stack.push(U256::from(0));
                                pc += 1;
                                continue
                            }

                            let result = a % b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    LT => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let boolean = if a < b { U256::from(1) } else { U256::from(0) };
                            stack.push(boolean);
                            pc += 1;
                        }
                    },
                    SLT => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let boolean = if a > b { U256::from(1) } else { U256::from(0) };
                            stack.push(boolean);
                            pc += 1;
                        }
                    },
                    GT | SGT => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            let boolean = if a > b { U256::from(1) } else { U256::from(0) };
                            stack.push(boolean);
                            pc += 1;
                        }
                    },
                    EQ => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let boolean = if a == b { U256::from(1) } else { U256::from(0) };
                            stack.push(boolean);
                            pc += 1;
                        }
                    },
                    ISZERO => {
                        let a = stack.pop();
                        
                        if let Some(a) = a {
                            let boolean = if a == U256::from(0) { U256::from(1) } else { U256::from(0) };
                            stack.push(boolean);
                            pc += 1;
                        }
                    },
                    AND => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let result = a & b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    OR => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let result = a | b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    XOR => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let result = a ^  b;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    NOT => {
                        let a = stack.pop();
                        
                        if let Some(a) = a {
                            println!("a {}", a);
                            let result = !a;
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    BYTE => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            println!("a {}", a);
                            println!("b {}", b);
                            let result = (b >> (U256::from(248) - a * 8)) & U256::from(255);
                            // y = (x >> (248 - i * 8)) & 0xFF
                            println!("result {}", result);
                            stack.push(result);
                            pc += 1;
                        }
                    },
                    PUSH32 => {
                        let data = &code[pc + 1 .. pc + 33];
                        let value = U256::from(data);
                        stack.push(value);
                        pc += 32;
                    },
                    MLOAD => {
                        let offset = stack.pop();
                        if let Some(offset) = offset {
                            println!("loading at offset: {}", offset);
                            let value = self.memory.load(offset.as_usize(), 32 - offset.as_usize());
                            let big_value = U256::from_big_endian(&value);
                            println!("loaded value: {}", big_value);
                            stack.push(big_value);
                        }
                    },
                    MSTORE => {
                        let offset = stack.pop();
                        let value = stack.pop();
                        if let (Some(offset), Some(value)) = (offset, value) {
                            println!("saving at offset: {}", offset);
                            let mut bytes = [0u8; 32];
                            value.to_big_endian(&mut bytes);
                            self.memory.store(offset.as_usize(), &bytes);
                        }
                    },
                    MSTORE8 => {
                        let offset = stack.pop();
                        let value = stack.pop();
                        if let (Some(offset), Some(value)) = (offset, value) {
                            println!("saving at value at offset: {} {}", value, offset);
                            let mut bytes = [0u8; 32];
                            value.to_big_endian(&mut bytes);
                            let last_byte = bytes[31]; // Extract the last byte of the U256 value
                            self.memory.store8(offset.as_usize(), last_byte);
                        }
                    }
                    MSIZE => {
                        let msize = self.memory.size();
                        stack.push(U256::from(msize))
                    }
                    JUMP => {
                        let destination = stack.pop();
                        if let Some(destination) = destination {
                            pc = destination.as_usize();
                        }
                    },
                    JUMPI => {
                        let destination = stack.pop();
                        let condition = stack.pop();
                        if let (Some(destination), Some(condition)) = (destination, condition)  {
                            if condition == U256::from(1) {
                                pc = destination.as_usize();
                            }
                        }
                    },
                    PC => {
                        let pc = pc.clone();
                        stack.push(U256::from(pc));
                    },
                    // "5B" => {
                    //     // let destination = stack.pop();
                    //     // if let Some(destination) = destination {
                    //     //     println!("jump {}", destination);
                    //     //     pc = destination.as_usize();
                    //     // }
                    // },
                    PUSH1 => {
                        let next_value = code.get(pc + 1);
                        if let Some(value) = next_value {
                            let value = U256::from_big_endian(&[*value]);
                            let bigint = value;
                            stack.push(bigint);
                            pc += 1;
                        }
                    },
                    PUSH2 => {
                        let data = &code[pc + 1 .. pc + 2];
                        let value = U256::from(data);
                        stack.push(value);
                        pc += 2;
                    },
                    DUP1 => {
                        let latest = stack.pop();
                        if let Some(value) = latest {
                            let dup = value.clone();
                            stack.push(value + dup);
                            pc += 1;
                        }
                    },
                    DUP2 => {
                        let second_last = stack.get(stack.len() - 2);
                        if let Some(value) = second_last {
                            let dup = value.clone();
                            stack.push(dup);
                            pc += 1;
                        }
                    },
                    DUP3 => {
                        let second_last = stack.get(stack.len() - 3);
                        if let Some(value) = second_last {
                            let dup = value.clone();
                            stack.push(dup);
                            pc += 1;
                        }
                    },
                    SWAP1 => {
                        let a = stack.pop();
                        let b = stack.pop();
                        
                        if let (Some(a), Some(b)) = (a, b) {
                            stack.push(a);
                            stack.push(b);
                            pc += 1;
                        }
                    },
                    SWAP3 => {
                        let b = stack.remove(3);
                        let a = stack.remove(0);
                        // let b = stack.get(2);
                        
                        stack.insert(0, b);
                        stack.insert(3, a);
                        pc += 1;
                    },
                    ADDRESS => {
                        if let Some(address) = tx.as_ref().and_then(|t| t.to.clone()) {
                            println!("address: {}", address);
                            stack.push(U256::from_str(&address).unwrap())
                        }
                        pc += 1;
                    },
                    CALLER => {
                        if let Some(from) = tx.as_ref().and_then(|t| t.from.clone()) {
                            println!("from: {}", from);
                            stack.push(U256::from_str(&from).unwrap())
                        }
                        pc += 1;
                    },
                    _ => {
                        break
                    }
                }

                for v in &stack {
                    println!("stack value {:#X},", v);
                };

                pc += 1;
            //     continue
            // }

        }
        
        return stack.into_iter().rev().collect();
    }
    
}

// revm
pub struct OpCode(u8);

pub const STOP: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EXP: u8 = 0x0a;
pub const SIGNEXTEND: u8 = 0x0b;

pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const SLT: u8 = 0x12;
pub const SGT: u8 = 0x13;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;
pub const BYTE: u8 = 0x1a;

pub const CALLDATALOAD: u8 = 0x35;
pub const CALLDATASIZE: u8 = 0x36;
pub const CALLDATACOPY: u8 = 0x37;
pub const CODESIZE: u8 = 0x38;
pub const CODECOPY: u8 = 0x39;

pub const SHL: u8 = 0x1b;
pub const SHR: u8 = 0x1c;
pub const SAR: u8 = 0x1d;
pub const KECCAK256: u8 = 0x20;
pub const POP: u8 = 0x50;
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const MSTORE8: u8 = 0x53;
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const PC: u8 = 0x58;
pub const MSIZE: u8 = 0x59;
pub const JUMPDEST: u8 = 0x5b;

pub const TLOAD: u8 = 0x5c;
pub const TSTORE: u8 = 0x5d;

pub const MCOPY: u8 = 0x5e;
pub const PUSH0: u8 = 0x5f;
pub const PUSH1: u8 = 0x60;
pub const PUSH2: u8 = 0x61;
pub const PUSH3: u8 = 0x62;
pub const PUSH4: u8 = 0x63;
pub const PUSH5: u8 = 0x64;
pub const PUSH6: u8 = 0x65;
pub const PUSH7: u8 = 0x66;
pub const PUSH8: u8 = 0x67;
pub const PUSH9: u8 = 0x68;
pub const PUSH10: u8 = 0x69;
pub const PUSH11: u8 = 0x6a;
pub const PUSH12: u8 = 0x6b;
pub const PUSH13: u8 = 0x6c;
pub const PUSH14: u8 = 0x6d;
pub const PUSH15: u8 = 0x6e;
pub const PUSH16: u8 = 0x6f;
pub const PUSH17: u8 = 0x70;
pub const PUSH18: u8 = 0x71;
pub const PUSH19: u8 = 0x72;
pub const PUSH20: u8 = 0x73;
pub const PUSH21: u8 = 0x74;
pub const PUSH22: u8 = 0x75;
pub const PUSH23: u8 = 0x76;
pub const PUSH24: u8 = 0x77;
pub const PUSH25: u8 = 0x78;
pub const PUSH26: u8 = 0x79;
pub const PUSH27: u8 = 0x7a;
pub const PUSH28: u8 = 0x7b;
pub const PUSH29: u8 = 0x7c;
pub const PUSH30: u8 = 0x7d;
pub const PUSH31: u8 = 0x7e;
pub const PUSH32: u8 = 0x7f;
pub const DUP1: u8 = 0x80;
pub const DUP2: u8 = 0x81;
pub const DUP3: u8 = 0x82;
pub const DUP4: u8 = 0x83;
pub const DUP5: u8 = 0x84;
pub const DUP6: u8 = 0x85;
pub const DUP7: u8 = 0x86;
pub const DUP8: u8 = 0x87;
pub const DUP9: u8 = 0x88;
pub const DUP10: u8 = 0x89;
pub const DUP11: u8 = 0x8a;
pub const DUP12: u8 = 0x8b;
pub const DUP13: u8 = 0x8c;
pub const DUP14: u8 = 0x8d;
pub const DUP15: u8 = 0x8e;
pub const DUP16: u8 = 0x8f;
pub const SWAP1: u8 = 0x90;
pub const SWAP2: u8 = 0x91;
pub const SWAP3: u8 = 0x92;
pub const SWAP4: u8 = 0x93;
pub const SWAP5: u8 = 0x94;
pub const SWAP6: u8 = 0x95;
pub const SWAP7: u8 = 0x96;
pub const SWAP8: u8 = 0x97;
pub const SWAP9: u8 = 0x98;
pub const SWAP10: u8 = 0x99;
pub const SWAP11: u8 = 0x9a;
pub const SWAP12: u8 = 0x9b;
pub const SWAP13: u8 = 0x9c;
pub const SWAP14: u8 = 0x9d;
pub const SWAP15: u8 = 0x9e;
pub const SWAP16: u8 = 0x9f;
pub const RETURN: u8 = 0xf3;
pub const REVERT: u8 = 0xfd;
pub const INVALID: u8 = 0xfe;
pub const ADDRESS: u8 = 0x30;
pub const BALANCE: u8 = 0x31;
pub const BASEFEE: u8 = 0x48;
pub const ORIGIN: u8 = 0x32;
pub const CALLER: u8 = 0x33;
pub const CALLVALUE: u8 = 0x34;
pub const GASPRICE: u8 = 0x3a;
pub const EXTCODESIZE: u8 = 0x3b;
pub const EXTCODECOPY: u8 = 0x3c;
pub const EXTCODEHASH: u8 = 0x3f;
pub const RETURNDATASIZE: u8 = 0x3d;
pub const RETURNDATACOPY: u8 = 0x3e;
pub const BLOCKHASH: u8 = 0x40;
pub const COINBASE: u8 = 0x41;
pub const TIMESTAMP: u8 = 0x42;
pub const NUMBER: u8 = 0x43;
pub const DIFFICULTY: u8 = 0x44;
pub const GASLIMIT: u8 = 0x45;
pub const SELFBALANCE: u8 = 0x47;
pub const SLOAD: u8 = 0x54;
pub const SSTORE: u8 = 0x55;
pub const GAS: u8 = 0x5a;

pub const LOG0: u8 = 0xa0;
pub const LOG1: u8 = 0xa1;
pub const LOG2: u8 = 0xa2;
pub const LOG3: u8 = 0xa3;
pub const LOG4: u8 = 0xa4;

pub const CREATE: u8 = 0xf0;
pub const CREATE2: u8 = 0xf5;
pub const CALL: u8 = 0xf1;
pub const CALLCODE: u8 = 0xf2;
pub const DELEGATECALL: u8 = 0xf4;
pub const STATICCALL: u8 = 0xfa;
pub const SELFDESTRUCT: u8 = 0xff;
pub const CHAINID: u8 = 0x46;
