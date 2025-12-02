use crate::core::{Circuit, display_result};

pub mod core;

fn main() -> Result<(), ()> {
    let mut c = Circuit::new(2);
    c.h(0)?;
    c.cx(0, 1)?;
    let res = c.run()?;
    display_result(&res);
    Ok(())
}
