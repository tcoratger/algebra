use crate::hashing::curve_maps::parity;
use ark_test_curves::bls12_381::{Fq, Fq2, Fq6};

#[test]
fn test_parity_of_prime_field_elements() {
    let a1 = Fq::from(0);
    let a2 = Fq::from(1);
    let a3 = Fq::from(10);
    assert!(!parity(&a1));
    assert!(parity(&a2));
    assert!(!parity(&a3));
}

#[test]
fn test_parity_of_quadratic_extension_elements() {
    let element_test1 = Fq2::new(Fq::from(0), Fq::from(1));
    let element_test2 = Fq2::new(Fq::from(1), Fq::from(0));
    let element_test3 = Fq2::new(Fq::from(10), Fq::from(5));
    let element_test4 = Fq2::new(Fq::from(5), Fq::from(10));
    assert!(parity(&element_test1), "parity is the oddness of first non-zero coefficient of element represented over the prime field" );
    assert!(parity(&element_test2));
    assert!(!parity(&element_test3));
    assert!(parity(&element_test4));
}

#[test]
fn test_parity_of_cubic_extension_elements() {
    let a1 = Fq2::new(Fq::from(0), Fq::from(0));
    let a2 = Fq2::new(Fq::from(0), Fq::from(1));
    let a3 = Fq2::new(Fq::from(1), Fq::from(0));
    let a4 = Fq2::new(Fq::from(1), Fq::from(1));
    let a5 = Fq2::new(Fq::from(0), Fq::from(2));

    let element_test1 = Fq6::new(a1, a2, a3);
    let element_test2 = Fq6::new(a2, a3, a4);
    let element_test3 = Fq6::new(a3, a4, a1);
    let element_test4 = Fq6::new(a4, a1, a2);
    let element_test5 = Fq6::new(a1, a5, a2);

    assert!(parity(&element_test1), "parity is the oddness of first non-zero coefficient of element represented over the prime field");
    assert!(parity(&element_test2), "parity is the oddness of first non-zero coefficient of element represented over the prime field");
    assert!(parity(&element_test3));
    assert!(parity(&element_test4));
    assert!(!parity(&element_test5));
}
