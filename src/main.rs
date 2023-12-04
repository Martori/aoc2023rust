mod helpers;
mod days;

use crate::days::day01;

fn main() {
    let day = 1;
    match day {
        1 => day01::solve(),
        _ => unimplemented!(),
    }
}
