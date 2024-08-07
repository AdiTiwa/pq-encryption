use crate::heisenberg::Hn;

use rand::Rng;

/* Lattice Encryption using H_n
 * ============================================================
 *
 * each secret H_n gives three f64 values, a, b, c, where a, b, c <= n, those
 * which forms the shared secret vector, then used to create 
 *
*/

fn dot(a: &Vec<i32>, b: &Vec<i32>, modulus: i32) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] * b[i]);
    }
    sum % modulus
}

fn flatten(secrets: &Vec<Hn>) -> Vec<i32> {
    let mut shared_secret = Vec::new();
    for i in 0..secrets.len() {
        shared_secret.push(secrets[i].H.a as i32);
        shared_secret.push(secrets[i].H.b as i32);
        shared_secret.push(secrets[i].H.c as i32);
    }
    shared_secret
}

pub fn encrypt(message: &[u8], secrets: &Vec<Hn>) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    
    let shared_secret = flatten(&secrets); 
    
    let mut encrypted_message: Vec<Vec<i32>> = Vec::new();

    for byte in message {
        for i in 0..8 {
            let bit = (byte >> i) & 1;

            let mut cooefficients = Vec::new();
            for _ in 0..shared_secret.len() {
                cooefficients.push(rng.gen_range(0..secrets[0].n));
            }

            if bit == 1 {
                let v = dot(&cooefficients, &shared_secret, secrets[0].n);
                cooefficients.push(v % secrets[0].n);
                encrypted_message.push(cooefficients);
            } else {
                let v = dot(&cooefficients, &shared_secret, secrets[0].n) + secrets[0].n / 2;
                cooefficients.push(v % secrets[0].n);
                encrypted_message.push(cooefficients);
            }
        }
    }

    encrypted_message
}

pub fn decrypt(encrypted_message: &Vec<Vec<i32>>, secrets: &Vec<Hn>) -> String {
    let mut message = Vec::new(); 
    let shared_secret = flatten(&secrets);

    for i in 0..encrypted_message.len() / 8 {
        let mut byte = 0;
        for j in 0..8 {
            let mut bit: i32; 

            let dot_product = dot(&encrypted_message[i * 8 + j][..(shared_secret.len())].to_vec(), &shared_secret, secrets[0].n);
            if dot_product == encrypted_message[i * 8 + j][shared_secret.len()] {
                bit = 1;
            } else {
                bit = 0;
            }
            byte |= bit << j;
        }
        message.push(byte as u8);
    }

    String::from_utf8(message).unwrap()
}
