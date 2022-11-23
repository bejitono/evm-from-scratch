use primitive_types::U256;
use std::{fmt::Write, str::FromStr, clone};

struct MemoryError;

struct Memory {
    memory: Vec<U256>,
}

impl Memory {
    fn store(&mut self, offset: usize, value: U256) -> Result<bool, MemoryError> {
        // let max = U256::max_value();

        if offset < 0 { //|| offset > max
            return Err(MemoryError);
        }

        if value < U256::from(0) {
            return Err(MemoryError);
        }

        // self.memory[offset] = value;
        self.memory.push(value);

        Ok(true)
    }

    fn load(&self, offset: usize) -> Result<U256, MemoryError> {
        if offset < 0 {
            return Err(MemoryError);
        }

        if offset >= self.memory.len() {
            return Ok(U256::from(0));
        }
        
        return Ok(self.memory[offset]);
    }
}

pub fn evm(code: impl AsRef<[u8]>) -> Vec<U256> {

    // TODO: Implement me
    let mut stack: Vec<U256> = Vec::new();

    let mut opcode: String = String::new();
    
    let mut pc: usize = 0;

    let mut memory = Memory{ memory: vec![], };

    // for (i, data) in code.as_ref().iter().enumerate() {
    while pc < code.as_ref().len() {
        // if i % 2 == 0 {

            opcode = String::new();
            
            // write!(&mut opcode, "{:X}", data).expect("Unable to write");
            write!(&mut opcode, "{:X}", code.as_ref()[pc]).expect("Unable to write");

            // println!("adding opcode {}", opcode);

            println!("starting operation {}", opcode);
            match opcode.trim() {
                "00" => {
                    pc += 1;
                },
                "1" => {
                    let a = stack.pop();
                    let b = stack.pop();
                    if let (Some(a), Some(b)) = (a, b) {
                        let result = a + b;
                        stack.push(result);
                        pc += 1;
                    }
                },
                "50" => {
                    stack.pop();
                },
                "2" => {
                    let a = stack.pop();
                    let b = stack.pop();
                    if let (Some(a), Some(b)) = (a, b) {
                        let result = a * b;
                        stack.push(result);
                        pc += 1;
                    }
                },
                "3" => {
                    let a = stack.pop();
                    let b = stack.pop();
                    if let (Some(a), Some(b)) = (a, b) {
                        let result = a - b;
                        stack.push(result);
                        pc += 1;
                    }
                },
                "4" | "5" => {
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
                "6" | "7" => {
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
                "10" => {
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
                "12" => {
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
                "11" | "13" => {
                    let a = stack.pop();
                    let b = stack.pop();
                    
                    if let (Some(a), Some(b)) = (a, b) {
                        let boolean = if a > b { U256::from(1) } else { U256::from(0) };
                        stack.push(boolean);
                        pc += 1;
                    }
                },
                "14" => {
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
                "15" => {
                    let a = stack.pop();
                    
                    if let Some(a) = a {
                        let boolean = if a == U256::from(0) { U256::from(1) } else { U256::from(0) };
                        stack.push(boolean);
                        pc += 1;
                    }
                },
                "16" => {
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
                "17" => {
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
                "18" => {
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
                "19" => {
                    let a = stack.pop();
                    
                    if let Some(a) = a {
                        println!("a {}", a);
                        let result = !a;
                        stack.push(result);
                        pc += 1;
                    }
                },
                "1A" => {
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
                "7F" => {
                    let data = &code.as_ref()[pc + 1 .. pc + 32];
                    let value = U256::from(data);
                    stack.push(value);
                    pc += 32;
                },
                "51" => { // mload
                    let offset = stack.pop();
                    if let Some(offset) = offset {
                        let loaded_value = &memory.load(offset.as_usize());

                        let value = match loaded_value {
                            Ok(value) => value,
                            Err(error) => panic!("Problem loading"),
                        };
                        println!("value: {}", value);
                        stack.push(*value);
                    }
                },
                "52" => { // mstore
                    let offset = stack.pop();
                    let value = stack.pop();
                    if let (Some(offset), Some(value)) = (offset, value) {
                        println!("offset: {}", offset);
                        println!("value: {}", value);
                        let result = &memory.store(offset.as_usize(), value);
                        let success = match result {
                            Ok(success) => success,
                            Err(error) => panic!("Failed to mstore"),
                        };
                        println!("{success}");

                        // pc += 1;
                    }
                },
                "56" => {
                    let destination = stack.pop();
                    if let Some(destination) = destination {
                        pc = destination.as_usize();
                    }
                },
                "57" => {
                    let destination = stack.pop();
                    let condition = stack.pop();
                    if let (Some(destination), Some(condition)) = (destination, condition)  {
                        if condition == U256::from(1) {
                            pc = destination.as_usize();
                        }
                    }
                },
                "58" => {
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
                "60" => {
                    let next_value = code.as_ref().get(pc + 1);
                    if let Some(value) = next_value {
                        let value = U256::from_big_endian(&[*value]);
                        let bigint = value;
                        stack.push(bigint);
                        pc += 1;
                    }
                },
                "61" => {
                    let data = &code.as_ref()[pc + 1 .. pc + 2];
                    let value = U256::from(data);
                    stack.push(value);
                    pc += 2;
                },
                "80" => {
                    let latest = stack.pop();
                    if let Some(value) = latest {
                        let dup = value.clone();
                        stack.push(value + dup);
                        pc += 1;
                    }
                },
                "81" => {
                    let second_last = stack.get(stack.len() - 2);
                    if let Some(value) = second_last {
                        let dup = value.clone();
                        stack.push(dup);
                        pc += 1;
                    }
                },
                "82" => {
                    let second_last = stack.get(stack.len() - 3);
                    if let Some(value) = second_last {
                        let dup = value.clone();
                        stack.push(dup);
                        pc += 1;
                    }
                },
                "90" => {
                    let a = stack.pop();
                    let b = stack.pop();
                    
                    if let (Some(a), Some(b)) = (a, b) {
                        stack.push(a);
                        stack.push(b);
                        pc += 1;
                    }
                },
                "92" => {
                    let b = stack.remove(3);
                    let a = stack.remove(0);
                    // let b = stack.get(2);
                    
                    stack.insert(0, b);
                    stack.insert(3, a);
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

fn getBigInt(opcode: &str, value: &u8) -> Option<U256> {
    println!("opcode {}", opcode);
    match opcode.trim() {
        "00" => None,
        "60" => {
            println!("hello");
            let value = U256::from_big_endian(&[*value]);
            let bigint = value; // this panics when it can't unwrap
            // This is safer:
            // let bigint = match string {
            //     Ok(bigint) =>  bigint,
            //     Err(error) => panic!("Problem conversion: {:?}", error),
            // };
            Some(bigint)
        },
        _ => None
    }
}

struct TryFromSliceError(());

fn slice_to_array_32<T>(slice: &[T]) -> Result<&[T; 32], TryFromSliceError> {
    if slice.len() == 32 {
        let ptr = slice.as_ptr() as *const [T; 32];
        unsafe {Ok(&*ptr)}
    } else {
        Err(TryFromSliceError(()))
    }
}

fn something() {
    let string = String::from("hello");
    take_ownership(string);

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let string1 = String::from("hello");
    let returned_string = return_ownership(string1);
    // println!("{}", string1); // error, string1 moves to return_ownership
    println!("{}", returned_string); // ok
}

fn take_ownership(string: String) {
    println!("{}", string);
}

fn make_copy(int: i32) {
    println!("{}", int);
}

fn take_ownership2(ints: Vec<i32>) {
    for int in ints {
        println!("{}", int);
    }
}

fn return_ownership(string: String) -> String {
    string
}


fn hello() {
    let hi = String::from("fsdfsds");
    let length = take_string(&hi);

}

fn take_string(string: &String) -> usize {
    return string.len();
}