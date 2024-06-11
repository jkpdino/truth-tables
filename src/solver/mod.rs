use std::collections::HashMap;

use itertools::Itertools;

use crate::{parser::{Expr, Operation}, table};

mod finder;

pub struct TruthTable {
	pub input_vars: Vec<char>,

	pub inputs: Vec<Vec<bool>>,
	pub outputs: Vec<bool>,
}

pub fn solve_for_table(expr: Expr) -> TruthTable
{
	let variables = finder::find_variables(&expr);
	let ordered_variables = variables.iter()
									 .cloned()
									 .sorted()
									 .collect_vec();

	let truth_table_inputs = table::get_truth_table(variables.len());

	let truth_table_inputs_map = truth_table_inputs.iter()
		.map(|vars| ordered_variables
			.iter()
			.cloned()
			.zip(vars.iter().cloned())
			.collect::<HashMap<_, _>>())
		.collect_vec();

	let truth_table_outputs = truth_table_inputs_map.iter()
		.map(|vars| solve(&expr, vars))
		.collect_vec();
	
	TruthTable {
		input_vars: ordered_variables,
		inputs: truth_table_inputs,
		outputs: truth_table_outputs
	}
}

fn solve(expr: &Expr, variables: &HashMap<char, bool>) -> bool {
	match expr {
		Expr::Variable(c) => variables[c],
		Expr::True => true,
		Expr::False => false,
		Expr::Operation(op, l, r) => {
			let l = solve(&l, variables);
			let r = solve(&r, variables);

			match op {
				Operation::Conjunction => l & r,
				Operation::Disjunction => l | r,
				Operation::Equality => l == r,
				Operation::Inequality => l != r,
				Operation::Conditional => !l | r,
			}
		}
		Expr::Negate(n) => !solve(&n, variables),
	}
}