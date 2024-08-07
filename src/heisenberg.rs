use std::ops::Add;
use std::fmt;

/*
 * Heisenberg.rs
 * ==============
 * Definition for Heisenberg Group,
 *
 * | 1  a  c |
 * | 0  1  b |
 * | 0  0  1 |
 *
*/

#[derive(Clone, Copy)]
pub struct H { // Heisenberg Group
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl H {
    pub fn new(a: f64, b: f64, c: f64) -> H {
        H { a, b, c }
    }
}

impl<'a, 'b> Add<&'b H> for &'a H {
    type Output = H;

    fn add(self, other: &'b H) -> H {
        H {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c + (self.a * other.b),
        }
    }
}

impl fmt::Debug for H {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hn { // H_n is H where a, b, c <= F_n
    pub H : H,
    pub n : i32, // modulus
}

impl Hn {
    pub fn new(a: f64, b: f64, c: f64, n: i32) -> Hn {
        Hn {
            H: H::new(a % n as f64, b % n as f64, c % n as f64),
            n,
        }
    }

    pub fn from_vals(vals: Vec<f64>, n: i32) -> Hn {
        assert_eq!(vals.len(), 3);

        Hn {
            H: H::new(vals[0] % n as f64, vals[1] % n as f64, vals[2] % n as f64),
            n,
        }
    }
}

impl<'a, 'b> Add<&'b Hn> for &'a Hn {
    type Output = Hn;

    fn add(self, other: &'b Hn) -> Hn {
        assert_eq!(self.n, other.n);

        let mut r = Hn {
            H: &self.H + &other.H,
            n: self.n,
        };

        r.H.a = r.H.a % r.n as f64;
        r.H.b = r.H.b % r.n as f64;
        r.H.c = r.H.c % r.n as f64;

        r
    }
}
