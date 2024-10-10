use crate::algebra::FieldElement;
use std::ops::{Add, Sub, Mul, Div, Neg};
use num_bigint::BigInt;
use std::fmt;

#[derive(Clone)]
pub struct Polynomial {
    coefficients: Vec<FieldElement>,
}

// Implement Debug for Polynomial
impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Polynomial")
            .field("coefficients", &self.coefficients)
            .finish()
    }
}

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial { coefficients }
    }

    pub fn degree(&self) -> i32 {
        if self.coefficients.is_empty() {
            return -1;
        }
        let zero = self.coefficients[0].field.zero();
        if self.coefficients.iter().all(|c| *c == zero) {
            return -1;
        }
        self.coefficients.iter().enumerate()
            .rev()
            .find(|(_, c)| **c != zero)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }

    pub fn leading_coefficient(&self) -> FieldElement {
        self.coefficients[self.degree() as usize].clone()
    }

    pub fn is_zero(&self) -> bool {
        self.degree() == -1
    }

    pub fn divide(numerator: &Self, denominator: &Self) -> Option<(Self, Self)> {
        if denominator.is_zero() {
            return None;
        }
        if numerator.degree() < denominator.degree() {
            return Some((Polynomial::new(vec![]), numerator.clone()));
        }
        let field = &denominator.coefficients[0].field;
        let mut remainder = numerator.clone();
        let mut quotient_coefficients = vec![field.zero(); (numerator.degree() - denominator.degree() + 1) as usize];
        let leading_denominator = denominator.leading_coefficient();
        
        while remainder.degree() >= denominator.degree() {
            let coefficient = field.divide(&remainder.leading_coefficient(), &leading_denominator);
            let shift = remainder.degree() - denominator.degree();
            quotient_coefficients[shift as usize] = coefficient.clone();
            
            for (i, coeff) in denominator.coefficients.iter().enumerate() {
                let idx = shift as usize + i;
                let subtrahend = field.multiply(&coefficient, coeff);
                remainder.coefficients[idx] = field.subtract(&remainder.coefficients[idx], &subtrahend);
            }
            
            while !remainder.coefficients.is_empty() && remainder.coefficients.last().unwrap().is_zero() {
                remainder.coefficients.pop();
            }
        }
        
        Some((Polynomial::new(quotient_coefficients), remainder))
    }

    // Add this new method
    pub fn shift(&self, shift: usize) -> Self {
        let mut new_coeffs = vec![self.coefficients[0].field.zero(); shift];
        new_coeffs.extend(self.coefficients.clone());
        Polynomial::new(new_coeffs)
    }

    pub fn interpolate_domain(domain: &[FieldElement], values: &[FieldElement]) -> Self {
        assert_eq!(domain.len(), values.len(), "number of elements in domain does not match number of values -- cannot interpolate");
        assert!(!domain.is_empty(), "cannot interpolate between zero points");
        
        let field = domain[0].field.clone();
        let x = Polynomial::new(vec![field.zero(), field.one()]);
        let mut acc = Polynomial::new(vec![]);
        
        for i in 0..domain.len() {
            let mut prod = Polynomial::new(vec![values[i].clone()]);
            for j in 0..domain.len() {
                if j == i {
                    continue;
                }
                prod = prod * (x.clone() - Polynomial::new(vec![domain[j].clone()])) * 
                       Polynomial::new(vec![(domain[i].clone() - domain[j].clone()).inverse()]);
            }
            acc = acc + prod;
        }
        acc
    }

    pub fn zerofier_domain(domain: &[FieldElement]) -> Self {
        let field = domain[0].field.clone();
        let x = Polynomial::new(vec![field.zero(), field.one()]);
        domain.iter().fold(Polynomial::new(vec![field.one()]), |acc, d| {
            acc * (x.clone() - Polynomial::new(vec![d.clone()]))
        })
    }

    pub fn evaluate(&self, point: &FieldElement) -> FieldElement {
        let mut xi = point.field.one();
        let mut value = point.field.zero();
        for c in &self.coefficients {
            value = value + c.clone() * xi.clone();
            xi = xi * point.clone();
        }
        value
    }

    pub fn evaluate_domain(&self, domain: &[FieldElement]) -> Vec<FieldElement> {
        domain.iter().map(|d| self.evaluate(d)).collect()
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        if self.is_zero() {
            return Polynomial::new(vec![]);
        }
        if exponent == BigInt::from(0) {
            return Polynomial::new(vec![self.coefficients[0].field.one()]);
        }
        let mut acc = Polynomial::new(vec![self.coefficients[0].field.one()]);
        let mut base = self.clone();
        let mut exp = exponent;
        while exp > BigInt::from(0) {
            if &exp & BigInt::from(1) != BigInt::from(0) {
                acc = acc * base.clone();
            }
            base = base.clone() * base.clone();
            exp >>= 1;
        }
        acc
    }

    pub fn scale(&self, factor: &FieldElement) -> Self {
        Polynomial::new(
            self.coefficients.iter().enumerate()
                .map(|(i, c)| factor.pow(&BigInt::from(i)) * c.clone())
                .collect()
        )
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.is_zero() {
            return other;
        } else if other.is_zero() {
            return self;
        }
        let field = self.coefficients[0].field.clone();
        let max_len = self.coefficients.len().max(other.coefficients.len());
        let mut coeffs = vec![field.zero(); max_len];
        for (i, c) in self.coefficients.iter().enumerate() {
            coeffs[i] = coeffs[i].clone() + c.clone();
        }
        for (i, c) in other.coefficients.iter().enumerate() {
            coeffs[i] = coeffs[i].clone() + c.clone();
        }
        Polynomial::new(coeffs)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_zero() || other.is_zero() {
            return Polynomial::new(vec![]);
        }
        let field = self.coefficients[0].field.clone();
        let mut buf = vec![field.zero(); self.coefficients.len() + other.coefficients.len() - 1];
        for (i, c1) in self.coefficients.iter().enumerate() {
            for (j, c2) in other.coefficients.iter().enumerate() {
                buf[i+j] = buf[i+j].clone() + c1.clone() * c2.clone();
            }
        }
        Polynomial::new(buf) // Sử dụng new để loại bỏ các hệ số 0 ở đầu
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let (quo, rem) = Polynomial::divide(&self, &other).expect("Division by zero polynomial");
        assert!(rem.is_zero(), "Cannot perform polynomial division because remainder is not zero");
        quo
    }
}

impl Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Polynomial::new(self.coefficients.into_iter().map(|c| -c).collect())
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficients.len() != other.coefficients.len() {
            return false;
        }
        self.coefficients
            .iter()
            .zip(other.coefficients.iter())
            .all(|(a, b)| a.is_equivalent_to(b))
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.coefficients.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
}

pub fn test_colinearity(points: &[(FieldElement, FieldElement)]) -> bool {
    let domain: Vec<FieldElement> = points.iter().map(|(x, _)| x.clone()).collect();
    let values: Vec<FieldElement> = points.iter().map(|(_, y)| y.clone()).collect();
    let polynomial = Polynomial::interpolate_domain(&domain, &values);
    polynomial.degree() == 1
}