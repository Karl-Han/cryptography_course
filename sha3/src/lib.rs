use std::convert::TryInto;
use std::fmt::Display;
use std::io::Read;

const RHO: [u32; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];

const RC: [u64; 24] = [
    1u64,
    0x8082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x808bu64,
    0x80000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x8au64,
    0x88u64,
    0x80008009u64,
    0x8000000au64,
    0x8000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x80000001u64,
    0x8000000080008008u64,
];

#[derive(Default, Debug)]
struct Buffer {
    buf: [u64; 25],
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { buf: [0u64; 25] }
    }

    // used to absorb a new buffer
    // padding and other things done by upper
    pub fn absorb(&mut self, new_buf: [u64; 24]) {}

    // actually the f works
    pub fn keccak(&mut self, round: u8) {
        // assume buf is already the state
        //
        // totally do round
        for i in 0..round {
            // every sheet(same y,z share the same sheet)
            let mut array: [u64; 5] = [0; 5];

            //println!("buf before THETA : {}", self);
            // THETA operation
            for x in 0..5 {
                for y in 0..5 {
                    // get the xor of all lanes on the same sheet
                    array[x] ^= self.buf[5 * y + x];
                }
            }
            for x in 0..5 {
                for y in 0..5 {
                    // do the theta operation by xor (x-1) and (x + 1)
                    // lane with proper shift
                    let y_temp = 5 * y;
                    self.buf[y_temp + x] ^= array[(x + 4) % 5] ^ array[(x + 1) % 5].rotate_left(1);
                }
            }
            //println!("buf after THETA : {}", self);

            // RHO operation
            for i in 1..25 {
                self.buf[i] = self.buf[i].rotate_left(RHO[i]);
            }
            //println!("buf after RHO: {}", self);

            // PI operation
            let mut temp_state = [0u64; 25];
            for x in 0..5 {
                for y in 0..5 {
                    temp_state[5 * y + x] = self.buf[5 * x + ((x + 3 * y) % 5)]
                }
            }
            self.buf = temp_state;
            //println!("buf after PI: {}", self);

            // CHI operation
            let mut temp_state = [0u64; 25];
            for y in 0..5 {
                let y_temp = y * 5;
                for x in 0..5 {
                    let mut lane = !self.buf[y_temp + ((x + 1) % 5)];
                    lane &= self.buf[y_temp + ((x + 2) % 5)];
                    temp_state[y_temp + x] = self.buf[y_temp + x] ^ lane;
                }
            }
            self.buf = temp_state;
            //println!("buf after CHI: {}", self);

            // IOTA operation
            self.buf[0] ^= RC[i as usize];
            //println!("buf after IOTA: {}", self);
        }
    }

    // squeeze out the same as buf
    pub fn squeeze(&mut self) -> [u64; 24] {
        [0u64; 24]
    }

    pub fn xor(&mut self, buf: [u64; 24]) {
        for i in 0..24 {
            self.buf[i] ^= buf[i];
        }
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..12 {
            write!(f, "\n{:016x} {:016x}", self.buf[2 * i], self.buf[2 * i + 1]);
        }
        write!(f, "\n{:016x}", self.buf[24])
    }
}

struct KeccakState {
    buf: Buffer,
    rate: usize,
    capacity: usize,
    offset: usize,
    delim: u8,
}

impl KeccakState {
    pub fn new(rate: usize, capacity: usize, delim: u8) -> KeccakState {
        KeccakState {
            buf: Default::default(),
            rate,
            capacity,
            offset: 0,
            delim,
        }
    }

    // function that transform hex to state string
    pub fn h2s(buf: [u8; 200]) -> [u64; 24] {
        let mut array = [0u64; 24];

        for i in 0..24 {
            array[i] = u64::from_le_bytes(
                buf[i * 8..(i + 1) * 8]
                    .try_into()
                    .expect("h2s: [u8] to u64"),
            );
        }

        array
    }

    // update and do keccak with reader
    pub fn update(&mut self, reader: &mut dyn Read) {
        let mut flag = true;
        let mut buf = [0u8; 200];
        while let Ok(bytes_read) = reader.read(&mut buf) {
            dbg!(bytes_read);
            if bytes_read != 0 || flag{
                if bytes_read < 200 {
                    self.padding(&mut buf, bytes_read);
                    let array = Self::h2s(buf);
                    self.buf.xor(array);
                }

                // all is now 200 bytes now
                self.buf.keccak(24);
                println!("update state buf to {}", self.buf);
            }
            else{
                break;
            }
            flag = false;
        }
    }

    // get the digest with `length`
    pub fn result(&mut self, length: usize) -> Vec<u8> {
        Vec::new()
    }

    // do padding with the existing state
    // pad offset to rate
    pub fn padding(&self, new_buf: &mut [u8; 200], offset: usize) {
        new_buf[offset] ^= self.delim;
        new_buf[self.rate - 1] ^= 0x80;
    }
}

pub trait Hasher {
    fn hash_file(&mut self, filename: String, buf: &mut Vec<u8>);
    fn hash_str(&mut self, s: &str, buf: &mut Vec<u8>);
}

pub struct Keccakf {
    state: KeccakState,
}

impl Hasher for Keccakf {
    // implement hasher trait
    fn hash_file(&mut self, filename: String, buf: &mut Vec<u8>) {
        assert!(buf.len() < self.state.rate);
    }

    fn hash_str(&mut self, s: &str, buf: &mut Vec<u8>) {
        assert!(buf.len() < self.state.rate);

        self.state.update(&mut s.as_bytes());
        println!("hash_str: state buf = {}", self.state.buf);
        println!("rate = {}", self.state.rate);
        let bytes_to_read = self.state.capacity / 2 / 8;

        for i in 0..bytes_to_read {
            //buf[i * 8..(i +1) * 8] = self.state.buf[i].to_le_bytes().try_into().expect("hash_str state.buf to buf");
            let bytes = self.state.buf.buf[i].to_le_bytes();
            for j in 0..8 {
                buf.push(bytes[j]);
            }
        }
    }
}

impl Keccakf {
    pub fn bit2rate(bits: usize) -> usize {
        200 - bits / 4
    }

    pub fn new_v256() -> Keccakf {
        Keccakf {
            state: KeccakState::new(Self::bit2rate(256), 256 / 4, 0x06),
        }
    }

    pub fn new_v128() -> Keccakf {
        Keccakf {
            state: KeccakState::new(Self::bit2rate(128), 128 / 4, 0x06),
        }
    }
}
