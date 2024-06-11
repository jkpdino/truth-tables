use itertools::Itertools;
use parser::Lexer;

use crate::{parser::Parser, table::get_truth_table, ui::App};

mod parser;
mod solver;
mod table;
mod ui;

fn main() {
    let app = App::default();
    app.run();
}

// get an expression in [x]
// parse the expression [x]
// get a list of the variables [x]
// get a list of the truth table [x]
// evaluate the truth table [x]
// display the truth table [x]