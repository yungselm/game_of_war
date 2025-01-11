mod cards;
mod deck;
mod player;

use pyo3::prelude::*;
use cards::*;
use deck::*;

/// A Python module implemented in Rust.
#[pymodule]
fn game_of_war(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Deck>()?;
    m.add_class::<Card>()?;
    m.add_class::<Player>()?;
    Ok(())
}
