const STATE_SIZE: usize = 288;

fn trivium(key: &[u8], iv: &[u8], output_len: usize) -> Vec<u8> {
    let mut state = [0u8; STATE_SIZE];
    let mut output = Vec::with_capacity(output_len);

    // Initialization
    for i in 0..80 {
        if i < key.len() {
            state[i] = key[i];
        } else {
            break;
        }
    }
    for i in 0..80 {
        if i < iv.len() {
            state[i + 93] = iv[i];
        } else {
            break;
        }
    }
    state[285] |= 1 << 7; // Setting the 8th bit of state[285]

    // Trivium Algorithm
    for _ in 0..4 * STATE_SIZE {
        let t1 = state[65] ^ state[92];
        let t2 = state[161] ^ state[176];
        let t3 = state[242] ^ state[287];

        let s = t1 ^ t2 ^ t3;

        for i in (1..STATE_SIZE).rev() {
            state[i] = state[i - 1];
        }

        state[0] = s ^ t3 ^ (state[176] & state[161]) ^ state[263] ^ (state[92] & state[65]);

        output.push((state[0] & 1) ^ s); // Pushing LSB of state[0] XOR s

        state[93] ^= t1;
        state[177] ^= t2;
        state[286] ^= t3;
    }

    output
}

fn main() {
    let key = b"0123456789abcdef0123456";
    let iv = b"456789abcdef01234567891";
    let output_len = 64;

    let keystream = trivium(key, iv, output_len);
    
    println!("Keystream: {:?}", keystream);
}
