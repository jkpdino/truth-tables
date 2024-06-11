use std::collections::HashSet;

use crate::parser::Expr;

pub fn find_variables(in_expr: &Expr) -> HashSet<char>
{
	let mut variables = HashSet::new();

	search(in_expr, &mut variables);

	return variables;
}

fn search(expr: &Expr, set: &mut HashSet<char>)
{
	match expr {
		Expr::Variable(c) => {
			set.insert(*c);
		},
		Expr::True => {}
		Expr::False => {}
		Expr::Operation(_, l, r) => {
			search(&l, set);
			search(&r, set);
		}
		Expr::Negate(o) => {
			search(&o, set);
		}
	}
}