#![deny(clippy::use_self)]

use macros::serialize;

#[derive(serialize)]
pub enum Direction {
    East,
}
