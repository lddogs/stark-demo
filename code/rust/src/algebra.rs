use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;

pub fn xgcd(x: &BigInt, y: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut old_r, mut r) = (x.clone(), y.clone());
    let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());
    let (mut old_t, mut t) = (BigInt::zero(), BigInt::one());

    while r != BigInt::zero() {
        let quotient = &old_r / &r;
        let temp_r = r.clone();
        r = old_r - &quotient * &r;
        old_r = temp_r;

        let temp_s = s.clone();
        s = old_s - &quotient * &s;
        old_s = temp_s;

        let temp_t = t.clone();
        t = old_t - &quotient * &t;
        old_t = temp_t;
    }

    (old_s, old_t, old_r)
}

#[derive(Clone)]
pub struct FieldElement {
    pub value: BigInt,
    pub field: Field,
}

// Implement Debug for FieldElement
impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement({} mod {})", self.value, self.field.p)
    }
}

impl FieldElement {
    pub fn new(value: BigInt, field: Field) -> Self {
        FieldElement { value, field }
    }

    pub fn inverse(&self) -> Self {
        self.field.inverse(self)
    }

    pub fn pow(&self, exponent: &BigInt) -> Self {
        let mut acc = FieldElement::new(BigInt::one(), self.field.clone());
        let mut val = self.clone();
        let mut exp = exponent.clone();

        while exp > BigInt::zero() {
            if &exp & BigInt::one() == BigInt::one() {
                acc = acc * val.clone();
            }
            val = val.clone() * val.clone();
            exp >>= 1;
        }
        acc
    }

    pub fn pow_u64(&self, exponent: u64) -> Self {
        self.pow(&BigInt::from(exponent))
    }

    pub fn is_zero(&self) -> bool {
        self.value == BigInt::zero()
    }
    pub fn normalized_value(&self) -> BigInt {
        let modulus = &self.field.p;
        let value = &self.value;
        
        if value < &BigInt::zero() {
            (value % modulus + modulus) % modulus
        } else {
            value % modulus
        }
    }

    pub fn is_equivalent_to(&self, other: &Self) -> bool {
        self.normalized_value() == other.normalized_value()
    }
}

impl std::ops::Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.field.add(&self, &rhs)
    }
}

impl std::ops::Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.field.multiply(&self, &rhs)
    }
}

impl std::ops::Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.field.subtract(&self, &rhs)
    }
}

impl std::ops::Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.field.divide(&self, &rhs)
    }
}

impl std::ops::Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.field.negate(&self)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Field {
    pub p: BigInt,
}

// Implement Debug for Field
impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field(p: {})", self.p)
    }
}

impl Field {
    pub fn new(p: BigInt) -> Self {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement::new(BigInt::zero(), self.clone())
    }

    pub fn one(&self) -> FieldElement {
        FieldElement::new(BigInt::one(), self.clone())
    }

    pub fn multiply(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((&left.value * &right.value) % &self.p, self.clone())
    }

    pub fn add(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((&left.value + &right.value) % &self.p, self.clone())
    }

    pub fn subtract(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((&self.p + &left.value - &right.value) % &self.p, self.clone())
    }

    pub fn negate(&self, operand: &FieldElement) -> FieldElement {
        FieldElement::new((&self.p - &operand.value) % &self.p, self.clone())
    }

    pub fn inverse(&self, operand: &FieldElement) -> FieldElement {
        let (a, _, _) = xgcd(&operand.value, &self.p);
        FieldElement::new(((&a % &self.p) + &self.p) % &self.p, self.clone())
    }

    pub fn divide(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        assert!(!right.is_zero(), "divide by zero");
        let (a, _, _) = xgcd(&right.value, &self.p);
        FieldElement::new(&left.value * &a % &self.p, self.clone())
    }

    pub fn main() -> Self {
        let p = BigInt::from(1u32) + BigInt::from(407u32) * (BigInt::from(1u32) << 119);
        Field::new(p)
    }

    pub fn generator(&self) -> FieldElement {
        assert_eq!(self.p, BigInt::from(1u32) + BigInt::from(407u32) * (BigInt::from(1u32) << 119), 
                   "Do not know generator for other fields beyond 1+407*2^119");
        FieldElement::new(BigInt::from(85408008396924667383611388730472331217u128), self.clone())
    }

    pub fn primitive_nth_root(&self, n: u32) -> FieldElement {
        if self.p == BigInt::from(1u32) + BigInt::from(407u32) * (BigInt::from(1u32) << 119) {
            let max_n = BigInt::from(1u32) << 119;
            assert!(BigInt::from(n) <= max_n && n.is_power_of_two(), 
                    "Field does not have nth root of unity where n > 2^119 or not power of two.");
            let mut root = FieldElement::new(BigInt::from(85408008396924667383611388730472331217u128), self.clone());
            let mut order = BigInt::from(1u32) << 119;
            while order != BigInt::from(n) {
                root = root.pow(&BigInt::from(2u32));
                order /= 2;
            }
            root
        } else {
            panic!("Unknown field, can't return root of unity.");
        }
    }

    pub fn sample(&self, byte_array: &[u8]) -> FieldElement {
        let mut acc = BigInt::zero();
        for &b in byte_array {
            acc = (acc << 8) | BigInt::from(b);
        }
        FieldElement::new(&acc % &self.p, self.clone())
    }
}