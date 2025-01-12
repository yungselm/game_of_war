pub mod cards;
pub mod deck;
pub mod player;
pub mod game;

use pyo3::prelude::*;
use cards::*;
use deck::*;
use player::*;
use game::*;

/// A Python module implemented in Rust.
#[pymodule]
fn game_of_war(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Deck>()?;
    m.add_class::<Card>()?;
    m.add_class::<Player>()?;
    m.add_class::<Game>()?;
    Ok(())
}
