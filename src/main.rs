use pqc_heisenberg::diffie_hellman::{Key, generate_public, generate_secret};
use pqc_heisenberg::lattice_encryption::{encrypt, decrypt};
use pqc_heisenberg::heisenberg::Hn;

use rand::Rng;

fn main() {
    let mut secrets = Vec::new();
    let modulus = 100;

    for i in 0..10 { // equivalent to a 30d lattice
        // public information
        let mut vals = Vec::new();
        for _ in 0..3 {
            vals.push(rand::thread_rng().gen_range(1..modulus) as f64);
        }
        let g = Hn::from_vals(vals, modulus);
        
        // alice
        let a = generate_public(g);
        
        // bob
        let b = generate_public(g);
        
        // alice (for example), and she only has access to b.H
        secrets.push(generate_secret(a.clone(), b.public.clone()));
    }
    
    let encrypted = encrypt(b"Hello, World!", &secrets);
    let decrypted = decrypt(&encrypted, &secrets);

    println!("{:?}", decrypted);
}
