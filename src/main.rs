use crate::core::{Circuit, Gate, display_result};

pub mod core;

pub fn test_circuit() -> Result<(), ()> {
    let mut c = Circuit::new(3);
    c.h(0)?;
    c.x(1)?;
    c.h(1)?;
    c.cnx(vec![0, 1], 2)?;
    let res = c.run()?;
    display_result(&res);
    Ok(())
}

fn main() -> Result<(), ()> {
    let mut c = Circuit::new(3);
    c.h(0)?;
    c.x(1)?;
    c.h(1)?;
    c.cnx(vec![0, 1], 2)?;
    let res = c.run()?;
    display_result(&res);
    println!("--------------------");
    let mut c = Circuit::new(2);
    c.h(1)?;
    c.add_gate(Gate::h(0).controlled(vec![1]).ok_or(())?)?;
    display_result(&c.run()?);
    println!("--------------------");
    let mut c = Circuit::new(2);
    // c.x(0)?;
    c.h(0)?;
    // c.h(1)?;
    let res = c.run()?;
    display_result(&res);
    Ok(())
}
