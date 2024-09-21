mod advancement;
use advancement::Advancement;
mod identifier;
use std::vec::Vec;

// abstract representation of the to-be-generated datapack
pub struct AbstractDatapack {
    advancements: Vec<Advancement>,
}