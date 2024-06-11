use std::fmt::Display;

#[derive(Debug)]
pub enum Expr
{
	Variable(char),

	True,
	False,

	Operation(Operation, Box<Expr>, Box<Expr>),
	Negate(Box<Expr>)
}

#[derive(Debug)]
pub enum Operation
{
	Conjunction,
	Disjunction,
	Equality,
	Inequality,
	Conditional
}

impl Display for Expr
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Variable(v) => write!(f, "{v}"),
            Expr::True => write!(f, "T"),
            Expr::False => write!(f, "F"),
            Expr::Operation(op, l, r) => write!(f, "({l} {} {r})", op.symbol()),
            Expr::Negate(u) => write!(f, "¬{u}"),
        }
    }
}

impl Operation {
	pub fn symbol(&self) -> char {
		match self {
			Operation::Conjunction => '∧',
			Operation::Disjunction => '∨',
			Operation::Equality => '≡',
			Operation::Inequality => '≢',
			Operation::Conditional => '→',
		}
	}
}