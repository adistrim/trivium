/* 

This Rust code implements the Trivium stream cipher, a lightweight cryptographic algorithm.
Trivium is a synchronous stream cipher designed for hardware and software implementations
with reduced resource requirements. It operates on 80-byte keys and initialization vectors (IVs)
and produces a keystream used for encryption and decryption of data. This code initializes a
Trivium instance with a key and an IV, encrypts a plaintext message, and then decrypts the 
resulting ciphertext back to the original plaintext. The output is printed to the console, and
can be redirected to an output file if needed.

To run the code first make sure you have rust installed in the system.

to compile it: rustc trivium.rs -o trivium
run that compiled file: ./trivium
to save the output: ./trivium > output.txt

*/


const STATE_SIZE: usize = 288;
const INITIALIZATION_ROUNDS: usize = 4;
const NUMBER_OF_ROUNDS: usize = 100;

struct Trivium {
    state: [u8; STATE_SIZE],  // Trivium internal state
    output: [u8; 4],          // Output buffer
}

impl Trivium {
    // Initialize a new Trivium instance with a key and an initialization vector (IV)
    fn new(key: &[u8], iv: &[u8]) -> Trivium {
        let mut trivium = Trivium {
            state: [0; STATE_SIZE],
            output: [0; 4],
        };

        // Initialize the state with the key and IV
        trivium.state[..80].copy_from_slice(&key);
        trivium.state[93..93 + 80].copy_from_slice(&iv);

        // Initialize the remaining parts of the state
        trivium.state[111] = 1;

        // Run the initialization rounds
        for _ in 0..INITIALIZATION_ROUNDS {
            trivium.update();
        }

        trivium
    }

    // Update the Trivium state
    fn update(&mut self) {
        let mut t1 = 0;
        let mut t2 = 0;

        for _ in 0..NUMBER_OF_ROUNDS {
            // Trivium update function
            t1 = (self.state[65] ^ self.state[90]) | (self.state[92] & self.state[93]) | (self.state[171] & self.state[174])
                | (self.state[263] & self.state[285]);
            t2 = (self.state[161] ^ self.state[174]) | (self.state[242] & self.state[287]);
            self.state.rotate_right(1);
            self.state[0] = t2;
            self.state[93] = t1;
        }
    }

    // Generate keystream
    fn generate(&mut self) {
        for _ in 0..8 {
            self.update();
            let output_copy = self.output.clone();  // Clone the output buffer
            // Zip the mutable output buffer with the cloned output and update each byte
            self.output.iter_mut().zip(output_copy.iter()).for_each(|(b, &x)| *b = x);
        }
    }

    // Encrypt data using Trivium stream cipher
    fn encrypt(&mut self, data: &mut [u8]) {
        self.generate();  // Generate keystream
        // XOR each byte of data with the keystream
        for byte in data.iter_mut() {
            *byte ^= self.output[0];
            self.output.rotate_left(1);  // Rotate the output buffer
        }
    }

    // Decrypt data using Trivium stream cipher (same as encryption)
    fn decrypt(&mut self, data: &mut [u8]) {
        self.encrypt(data);
    }
}

fn main() {
    let key = [0u8; 80];  // Example key (80 bytes), replace it with your own key
    let iv = [0u8; 80];   // Example IV (80 bytes), replace it with your own IV

    let plaintext = b"Hello, world!";  // Example plaintext

    let mut trivium = Trivium::new(&key, &iv);  // Initialize Trivium with key and IV
    let mut data = plaintext.to_vec();  // Convert plaintext to a mutable vector

    trivium.encrypt(&mut data);  // Encrypt the data
    println!("Ciphertext: {:?}", data);

    // Decrypt
    let mut trivium = Trivium::new(&key, &iv);  // Initialize Trivium again for decryption
    trivium.decrypt(&mut data);  // Decrypt the data
    println!("Decrypted: {:?}", String::from_utf8_lossy(&data));
}
