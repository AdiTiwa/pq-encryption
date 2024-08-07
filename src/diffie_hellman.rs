use crate::heisenberg::Hn;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub private: i32,
    pub public: Hn,
}

pub fn generate_public(g: Hn) -> Key {
    let a = rand::thread_rng().gen_range(1..g.n);

    let mut public = g.H;
    for _ in 1..a {
        public = &public + &g.H;
    }
    Key {
        public: Hn { H: public, n: g.n },
        private: a,
    }
}

pub fn generate_secret(k: Key, other: Hn) -> Hn {
    let mut secret = other;
    for _ in 1..k.private {
        secret = &secret + &other;
    }
    secret
}
