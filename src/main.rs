use log::{error, warn};
use std::{
    env,
    fs::File,
    io::{stdin, BufRead, BufReader, Error, Read},
    path::Path,
};

fn read_bytecode<R: Read>(reader: R) -> Option<Vec<u32>> {
    let reader = BufReader::new(reader);
    let lines: Vec<Result<String, Error>> = reader.lines().collect();
    let bytecode: String;

    if let Some(last_line) = lines.last() {
        if let Ok(last_line) = last_line {
            bytecode = last_line.clone();
        } else {
            warn!("Could not parse input: {:?}", last_line);
            return None;
        }
    } else {
        warn!("Could not parse input: {:?}", lines);
        return None;
    }

    for char in bytecode.chars() {
        if !char.is_ascii_hexdigit() {
            warn!(
                "Illegal input, contains non alphanumeric characters: {}",
                bytecode
            );
            return None;
        }
    }

    let radix: u32 = 16;
    let bytecode: Vec<char> = bytecode.chars().collect();
    let bytecode = bytecode
        .chunks(2)
        .map(|x| x[0].to_digit(radix).unwrap() * 10 + x[1].to_digit(radix).unwrap())
        .collect::<Vec<_>>();

    Some(bytecode)
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let stdin = stdin();
        let handle = stdin.lock();
        read_bytecode(handle);
    } else if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        if let Ok(file) = File::open(file_path) {
            read_bytecode(file);
        } else {
            error!("Error: Unable to open file '{}'", &args[1]);
        }
    } else {
        error!("Usage: {} [file]", &args[0]);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn read_valid_bytecode() {
        let bytes: &'static [u8] = b"\
            ======= storecontract/Store.sol:Store =======\n\
Binary of the runtime part:\n\
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
\n";
        let cursor = Cursor::new(bytes);
        let bytecode = super::read_bytecode(cursor).expect("Could not parse input");
        assert_eq!(60, *bytecode.first().unwrap());
        assert_eq!(33, *bytecode.last().unwrap());

        let bytes: &'static [u8] = b"11";
        let cursor = Cursor::new(bytes);
        let bytecode = super::read_bytecode(cursor).expect("Could not parse input");
        assert_eq!(11, *bytecode.first().unwrap());
        assert_eq!(11, *bytecode.last().unwrap());
    }

    #[test]
    fn invalid_input_space() {
        init();
        let input = "Hello world!";
        let cursur = Cursor::new(input);
        let result = super::read_bytecode(cursur);
        assert!(result.is_none());
    }
    #[test]
    fn invalid_input_no_hex() {
        init();
        let input = "Helloworld";
        let cursur = Cursor::new(input);
        let result = super::read_bytecode(cursur);
        assert!(result.is_none());
    }
    #[test]
    fn empty_input() {
        init();
        let input = "";
        let cursur = Cursor::new(input);
        let result = super::read_bytecode(cursur);
        assert!(result.is_none());
    }
}
