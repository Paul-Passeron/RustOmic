use crate::core::{C, Circuit, Gate, ONE, Z, is_identity, is_unit, norm};

use faer::mat;

fn approx(a: C, b: C) -> bool {
    norm(a - b) < 1e-5
}

#[test]
fn test_identity_check() {
    let i = mat![[ONE, Z], [Z, ONE]];
    assert!(is_identity(&i));

    let not_i = mat![[ONE, ONE], [Z, ONE]];
    assert!(!is_identity(&not_i));
}

#[test]
fn test_is_unit() {
    // Hadamard should be unitary
    let h = Gate::h(0).mat.clone();
    assert!(is_unit(&h));

    // Non-unitary matrix
    let bad = mat![[ONE, ONE], [ONE, ONE]];
    assert!(!is_unit(&bad));
}

#[test]
fn test_gate_h_construction() {
    let g = Gate::h(0);
    assert_eq!(g.targets, vec![0]);
    assert!(is_unit(&g.mat));
    assert_eq!(g.mat.nrows(), 2);
}

#[test]
fn test_gate_x_construction() {
    let g = Gate::x(0);
    assert_eq!(g.targets, vec![0]);
    assert!(is_unit(&g.mat));

    // X |0⟩ = |1⟩
    let mut c = Circuit::new(1);
    c.x(0).unwrap();
    let res = c.run().unwrap();
    assert!(approx(res["1"], ONE));
    assert!(approx(res["0"], Z));
}

#[test]
fn test_gate_cx_construction() {
    let g = Gate::cx(0, 1).unwrap();
    assert_eq!(g.targets.len(), 2);
    assert!(is_unit(&g.mat));
}

#[test]
fn test_turn_big_h_single_qubit() {
    // H on qubit 0 of 2-qubit system
    let g = Gate::h(0);
    let big = g.turn_big(2);

    // H₀ should act like H ⊗ I
    let h = g.mat.clone();

    // Kronecker manually
    let kron = mat![
        [h[(0, 0)], h[(0, 1)], Z, Z],
        [h[(1, 0)], h[(1, 1)], Z, Z],
        [Z, Z, h[(0, 0)], h[(0, 1)]],
        [Z, Z, h[(1, 0)], h[(1, 1)]],
    ];

    for i in 0..4 {
        for j in 0..4 {
            assert!(approx(big[(i, j)], kron[(i, j)]));
        }
    }
}

#[test]
fn test_run_h_single_qubit() {
    let mut c = Circuit::new(1);
    c.h(0).unwrap();
    let res = c.run().unwrap();

    let x = C::from_f64(1.0 / (2.0_f64).sqrt());
    assert!(approx(res["0"], x));
    assert!(approx(res["1"], x));
}

#[test]
fn test_bell_state() {
    // Create |Φ+⟩ = (|00⟩ + |11⟩)/√2
    let mut c = Circuit::new(2);
    c.h(0).unwrap();
    c.cx(0, 1).unwrap();

    let res = c.run().unwrap();
    let x = C::from_f64(1.0 / (2.0_f64).sqrt());

    assert!(approx(res["00"], x));
    assert!(approx(res["11"], x));
    assert!(approx(res["01"], Z));
    assert!(approx(res["10"], Z));
}

#[test]
fn test_circuit_vector_output_format() {
    let c = Circuit::new(3);
    let v = c.get_vec(5).unwrap();

    // vector should have 8 entries, with index 5 = 1
    for i in 0..8 {
        if i == 5 {
            assert!(approx(v[i], ONE));
        } else {
            assert!(approx(v[i], Z));
        }
    }
}
