use std::{
    env,
    fs::File,
    io::{stdin, BufReader, Read},
    path::Path,
};

fn read_bytecode<R: Read>(reader: R) -> Vec<u8> {
    let mut reader = BufReader::new(reader);
    let mut buffer = Vec::new();
    let mut bytecode: Vec<u8> = Vec::new();

    while let Ok(bytes_read) = reader.read_to_end(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        // Check if the first byte is an integer
        for byte in &buffer {
            if byte.is_ascii_digit() {
                bytecode.push(*byte);
            }
        }

        buffer.clear();
    }
    println!("Bytecode: {:?}", bytecode);
    bytecode
}

fn main() {
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
            eprintln!("Error: Unable to open file '{}'", &args[1]);
        }
    } else {
        eprintln!("Usage: {} [file]", &args[0]);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn read_bytecode() {
        let bytes: &'static [u8] = b"\
            ======= storecontract/Store.sol:Store =======\n
Binary of the runtime part:\n
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
265220429afd34242d64736f6c63430008130033
\n";
        let cursor = Cursor::new(bytes);
        let bytecode = super::read_bytecode(cursor);
        // '6' is 54 in ascii
        assert_eq!(54, *bytecode.first().unwrap());
        assert_eq!(51, *bytecode.last().unwrap());

        let bytes: &'static [u8] = b"1";
        let cursor = Cursor::new(bytes);
        let bytecode = super::read_bytecode(cursor);
        assert_eq!(49, *bytecode.first().unwrap());
        assert_eq!(49, *bytecode.last().unwrap());
    }
}
