use std::collections::{HashMap, HashSet};

use faer::{Col, Mat, mat, traits::fx128};

pub type C = fx128;

const ONE: C = C::from_f64(1.0);
const Z: C = C::ZERO;

pub struct Gate {
    mat: Mat<C>,
    targets: Vec<usize>,
}

pub struct Circuit {
    qubits: usize,
    gates: Vec<Gate>,
}

impl Gate {
    pub fn new(mat: Mat<C>, targets: Vec<usize>) -> Option<Self> {
        if mat.ncols() != mat.nrows() {
            return None;
        }
        let power = (2 as u32).pow(targets.len() as u32) as usize;
        if mat.ncols() != power {
            return None;
        }

        let mut ts = HashSet::new();
        for target in &targets {
            if !ts.insert(*target) {
                return None;
            }
        }

        Some(Self { mat, targets })
    }

    pub fn h(target: usize) -> Self {
        let x = C::from_f64(1.0 / (2.0 as f64).sqrt());
        Self::new(mat![[x, x], [x, -x]], vec![target]).unwrap()
    }

    pub fn x(target: usize) -> Self {
        Self::new(mat![[Z, ONE], [ONE, Z]], vec![target]).unwrap()
    }

    pub fn cx(control: usize, target: usize) -> Option<Self> {
        Self::new(
            mat![
                [ONE, Z, Z, Z],
                [Z, ONE, Z, Z],
                [Z, Z, Z, ONE],
                [Z, Z, ONE, Z],
            ],
            vec![target, control],
        )
    }

    pub fn turn_big(&self, n: usize) -> Mat<C> {
        let power = (2 as u32).pow(n as u32) as usize;
        let mut mat = Mat::zeros(power, power); // Start with zeros, not identity

        for row in 0..power {
            for col in 0..power {
                // Check if non-target qubits are identical between row and col
                let mut non_target_bits_match = true;
                for bit_pos in 0..n {
                    if !self.targets.contains(&bit_pos) {
                        let row_bit = (row >> bit_pos) & 1;
                        let col_bit = (col >> bit_pos) & 1;
                        if row_bit != col_bit {
                            non_target_bits_match = false;
                            break;
                        }
                    }
                }

                if !non_target_bits_match {
                    continue; // stays 0
                }
                let mut small_row = 0;
                let mut small_col = 0;
                for (i, target_idx) in self.targets.iter().enumerate() {
                    if (row >> target_idx) & 1 == 1 {
                        small_row |= 1 << i;
                    }
                    if (col >> target_idx) & 1 == 1 {
                        small_col |= 1 << i;
                    }
                }

                // Copy the gate element to the big matrix
                mat[(row, col)] = self.mat[(small_row, small_col)];
            }
        }

        mat
    }
}

impl Circuit {
    pub fn new(qubits: usize) -> Self {
        Self {
            gates: Vec::new(),
            qubits,
        }
    }

    pub fn get_vec(&self, i: usize) -> Option<Col<C>> {
        let size = (2 as u32).pow(self.qubits as u32) as usize;
        if i >= size {
            return None;
        }
        let mut v = Col::zeros(size);
        v[i] = ONE;
        Some(v)
    }

    pub fn h(&mut self, target: usize) -> Result<(), ()> {
        if target >= self.qubits {
            Err(())
        } else {
            self.gates.push(Gate::h(target));
            Ok(())
        }
    }

    pub fn x(&mut self, target: usize) -> Result<(), ()> {
        if target >= self.qubits {
            Err(())
        } else {
            self.gates.push(Gate::x(target));
            Ok(())
        }
    }

    pub fn cx(&mut self, control: usize, target: usize) -> Result<(), ()> {
        if target >= self.qubits || control >= self.qubits {
            Err(())
        } else {
            let g = Gate::cx(control, target).ok_or(())?;
            self.gates.push(g);
            Ok(())
        }
    }

    pub fn run(&self) -> Result<HashMap<String, C>, ()> {
        let mut current = self.get_vec(0).ok_or(())?;
        for gate in &self.gates {
            let g = gate.turn_big(self.qubits);
            let temp = g * current;
            current = temp;
        }
        let mut res = HashMap::new();
        for (i, x) in current.iter().enumerate() {
            let now = format!("{:0width$b}", i, width = self.qubits);
            res.insert(now, *x);
        }
        Ok(res)
    }
}

pub fn display_result(res: &HashMap<String, C>) {
    let mut strs = res.keys().collect::<Vec<_>>();
    strs.sort();
    for s in strs {
        println!("|{}⟩: {} + i{}", s, res[s].0, res[s].1);
    }
}

fn main() -> Result<(), ()> {
    let mut c = Circuit::new(2);
    c.h(0)?;
    c.cx(0, 1)?;
    let res = c.run()?;
    display_result(&res);
    Ok(())
}
