use stark_rust_demo::algebra::{Field, FieldElement};
use stark_rust_demo::univariate::Polynomial;
use rand::Rng;
use num_bigint::BigInt;
use std::time::Instant;

#[test]
fn test_distributivity() {
    let field = Field::main();
    let zero = field.zero();
    let one = field.one();
    let two = FieldElement::new(BigInt::from(2), field.clone());
    let five = FieldElement::new(BigInt::from(5), field.clone());

    let a = Polynomial::new(vec![one.clone(), zero.clone(), five.clone(), two.clone()]);
    let b = Polynomial::new(vec![two.clone(), two.clone(), one.clone()]);
    let c = Polynomial::new(vec![zero.clone(), five.clone(), two.clone(), five.clone(), five.clone(), one.clone()]);

    let lhs = a.clone() * (b.clone() + c.clone());
    let rhs = a.clone() * b + a * c;
    assert_eq!(lhs, rhs, "distributivity fails for polynomials");

    println!("univariate polynomial distributivity success \\o/");
}

#[test]
fn test_division() {
    let field = Field::main();
    let zero = field.zero();
    let one = field.one();
    let two = FieldElement::new(BigInt::from(2), field.clone());
    let five = FieldElement::new(BigInt::from(5), field.clone());

    let a = Polynomial::new(vec![one.clone(), zero.clone(), five.clone(), two.clone()]);
    let b = Polynomial::new(vec![two.clone(), two.clone(), one.clone()]);
    let c = Polynomial::new(vec![zero.clone(), five.clone(), two.clone(), two.clone(), five.clone(), one.clone()]);

    // a should divide a*b, quotient should be b
    let (quo, rem) = Polynomial::divide(&(a.clone() * b.clone()), &a).unwrap();
    assert!(rem.is_zero(), "fail division test 1");
    assert_eq!(quo, b, "fail division test 2");
    // b should divide a*b, quotient should be a
    let (quo, rem) = Polynomial::divide(&(a.clone() * b.clone()), &b).unwrap();
    assert!(rem.is_zero(), "fail division test 3");
    println!("quo: {:?}", quo);
    println!("a: {:?}", a);
    assert_eq!(quo, a, "fail division test 4");

    // c should not divide a*b
    let (quo, rem) = Polynomial::divide(&(a.clone() * b.clone()), &c).unwrap();
    assert!(!rem.is_zero(), "fail division test 5");

    // ... but quo * c + rem == a*b
    assert_eq!(quo * c + rem, a * b, "fail division test 6");

    println!("univariate polynomial division success \\o/");
}

#[test]
fn test_interpolate() {
    let field = Field::main();
    let zero = field.zero();
    let one = field.one();
    let two = FieldElement::new(BigInt::from(2), field.clone());
    let five = FieldElement::new(BigInt::from(5), field.clone());
    
    let values = vec![five.clone(), two.clone(), two.clone(), one.clone(), five.clone()];
    let domain: Vec<FieldElement> = (1..=5)
        .map(|i| FieldElement::new(BigInt::from(i), field.clone()))
        .collect();

    let poly = Polynomial::interpolate_domain(&domain, &values);

    for i in 0..domain.len() {
        assert!(
            poly.evaluate(&domain[i]) == values[i],
            "fail interpolate test 1"
        );
    }

    // evaluation in random point is nonzero with high probability
    assert!(
        poly.evaluate(&FieldElement::new(BigInt::from(363), field.clone())) != zero,
        "fail interpolate test 2"
    );

    assert!(
        poly.degree() == (domain.len() as i32 - 1),
        "fail interpolate test 3"
    );

    println!("univariate polynomial interpolate success \\o/");
}

#[test]
fn test_zerofier() {
    let field = Field::main();
    let mut rng = rand::thread_rng();

    let start_time = Instant::now();

    for trial in 0..100 {
        // Đảm bảo degree ít nhất là 1 và tối đa là 10
        let degree = rng.gen_range(1..=10);
        let mut domain = Vec::new();
        while domain.len() != degree {
            let new = field.sample(&rng.gen::<[u8; 17]>());
            if !domain.contains(&new) {
                domain.push(new);
            }
        }

        println!("Trial {}: degree = {}, domain size = {}", trial + 1, degree, domain.len());

        let zerofier = Polynomial::zerofier_domain(&domain);

        assert_eq!(zerofier.degree(), degree as i32, 
                   "zerofier has degree {}, expected {}", zerofier.degree(), degree);

        for d in &domain {
            assert!(zerofier.evaluate(d).is_zero(), 
                    "zerofier does not evaluate to zero at {}", d);
        }

        let mut random = field.sample(&rng.gen::<[u8; 17]>());
        while domain.contains(&random) {
            random = field.sample(&rng.gen::<[u8; 17]>());
        }

        assert!(!zerofier.evaluate(&random).is_zero(), 
                "zerofier evaluates to zero at random point {}", random);
    }

    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time);

    println!("univariate zerofier test success \\o/");
    println!("Thời gian chạy: {:.2?}", execution_time);
}