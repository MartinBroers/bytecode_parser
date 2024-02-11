mod flow;
mod flow_parser;
mod hex;
mod instruction;
mod memory;
mod opcode;
mod parser;
mod stack;
mod utils;

use clap::Parser;
use flow_parser::FlowParser;
use hex::Hex;
use log::{error, info, warn};
use parser::Parser as BytecodeParser;
use stack::StackElement;
use std::{
    env,
    fs::File,
    io::{self, stdin, BufRead, BufReader, Error, ErrorKind, Read},
    path::Path,
};

pub static mut CALLVALUE: Option<StackElement> = None;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    callvalue: Option<String>,

    #[arg(long)]
    filename: Option<String>,
}

fn read_bytecode(input: String) -> Option<Vec<u32>> {
    for char in input.chars() {
        if !char.is_ascii_hexdigit() {
            warn!(
                "Illegal input, contains non alphanumeric characters: {}",
                input
            );
            return None;
        }
    }

    let radix: u32 = 16;
    let bytecode: Vec<char> = input.chars().collect();
    let bytecode: Vec<u32> = bytecode
        .chunks(2)
        .map(|x| {
            u32::from_str_radix((x[0].to_string() + &x[1].to_string()).as_str(), radix).unwrap()
        })
        .collect();

    if bytecode.len() == 0 {
        None
    } else {
        Some(bytecode)
    }
}

fn parse_args(args: &Args) -> Result<(), std::io::Error> {
    if let Some(callvalue) = &args.callvalue {
        let value: Hex = match Hex::try_from(callvalue) {
            Ok(v) => v,
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
        };
        unsafe {
            CALLVALUE = Some(StackElement {
                value,
                origin: Hex(0),
                size: (format!("{:x}", value.0).len() + 1) / 2 as usize,
            });
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let args = Args::parse();
    let input: String;

    if let Some(ref cli_input) = args.input {
        let cli_input = cli_input.lines().last().unwrap();
        input = cli_input.to_string();
    } else if let Some(ref filename) = args.filename {
        let file_path = Path::new(&filename);
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            let lines: Vec<Result<String, Error>> = reader.lines().collect();
            if let Some(last_line) = lines.last() {
                if let Ok(last_line) = last_line {
                    input = last_line.clone();
                } else {
                    warn!("Could not parse input: {:?}", last_line);
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            } else {
                warn!("Could not parse input: {:?}", lines);
                return Err(Error::from(ErrorKind::InvalidInput));
            }
        } else {
            return Err(Error::from(ErrorKind::NotFound));
        }
    } else {
        error!("Provide input; either using an input string or a file. ");
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    if let Err(e) = parse_args(&args) {
        return Err(e);
    };

    let bytecode;
    let input = read_bytecode(input);
    if let Some(input) = input {
        bytecode = input;
    } else {
        error!("No bytecode found in input");
        return Err(Error::from(io::ErrorKind::InvalidData));
    }
    let parser = BytecodeParser::new(bytecode);
    let mut flow_parser = FlowParser::new(parser.get_instruction_sets(), parser.get_instructions());
    flow_parser.parse_flows();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{hex::Hex, CALLVALUE};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn read_valid_bytecode() {
        let bytes: &'static [u8] = b"\
608060405234801561001057600080fd5b50600436106100365760003560e\
01c80636057361d1461003b5780638f88708b14610057575b600080fd5b61\
005560048036038101906100509190610115565b610087565b005b6100716\
00480360381019061006c9190610115565b610094565b60405161007e9190\
610151565b60405180910390f35b610090816100b3565b5050565b6000610\
09f826100b3565b6002546100ac919061019b565b9050919050565b600081\
6002546100c391906101cf565b60028190555081600181905550600154905\
0919050565b600080fd5b6000819050919050565b6100f2816100df565b81\
146100fd57600080fd5b50565b60008135905061010f816100e9565b92915\
050565b60006020828403121561012b5761012a6100da565b5b6000610139\
84828501610100565b91505092915050565b61014b816100df565b8252505\
0565b60006020820190506101666000830184610142565b92915050565b7f\
4e487b7100000000000000000000000000000000000000000000000000000\
000600052601160045260246000fd5b60006101a6826100df565b91506101\
b1836100df565b92508282039050818111156101c9576101c861016c565b5\
b92915050565b60006101da826100df565b91506101e5836100df565b9250\
8282019050808211156101fd576101fc61016c565b5b9291505056fea2646\
9706673582212206471fae051fe349afbc0803628e924a4d57b36e4067d38\
265220429afd34242d64736f6c63430008130033\
";
        let cursor = String::from_utf8(bytes.to_vec()).unwrap();
        let bytecode = super::read_bytecode(cursor).expect("Could not parse input");
        assert_eq!(96, *bytecode.first().unwrap());
        assert_eq!(51, *bytecode.last().unwrap());

        let bytes: &'static [u8] = b"11";
        let cursor = String::from_utf8(bytes.to_vec()).unwrap();
        let bytecode = super::read_bytecode(cursor).expect("Could not parse input");
        assert_eq!(17, *bytecode.first().unwrap());
        assert_eq!(17, *bytecode.last().unwrap());
    }

    #[test]
    fn invalid_input_space() {
        init();
        let input = "Hello world!";
        let cursor = input.to_string();
        let result = super::read_bytecode(cursor);
        assert!(result.is_none());
    }
    #[test]
    fn invalid_input_no_hex() {
        init();
        let input = "Helloworld".to_string();
        let result = super::read_bytecode(input);
        assert!(result.is_none());
    }
    #[test]
    fn empty_input() {
        init();
        let input = "".to_string();
        let result = super::read_bytecode(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_input_callarg() {
        let input = "123";
        let args = super::Args {
            input: None,
            callvalue: Some(input.to_string()),
            filename: None,
        };
        super::parse_args(&args);
        let callvalue = unsafe { CALLVALUE.clone() }.unwrap();

        assert_eq!(callvalue.value, Hex(0x7b));
        assert_eq!(callvalue.size, 1);

        let input = "256";
        let args = super::Args {
            input: None,
            callvalue: Some(input.to_string()),
            filename: None,
        };
        super::parse_args(&args);
        let callvalue = unsafe { CALLVALUE.clone() }.unwrap();

        assert_eq!(callvalue.value, Hex(0x0100));
        assert_eq!(callvalue.size, 2);

        let input = "";
        let args = super::Args {
            input: None,
            callvalue: Some(input.to_string()),
            filename: None,
        };
        super::parse_args(&args);
        let callvalue = unsafe { CALLVALUE.clone() }.unwrap();

        assert_eq!(callvalue.value, Hex(0x0100));
        assert_eq!(callvalue.size, 2);
    }
}
