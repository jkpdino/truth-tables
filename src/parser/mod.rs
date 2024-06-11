mod lexer;
mod expr;

use std::iter::FromIterator;

use itertools::Itertools;
pub use lexer::Lexer;
pub use expr::Expr;
pub use expr::Operation;

use self::{lexer::Token};

pub struct Parser
{
	tokens: Vec<Token>,
	ptr: usize,
}

impl FromIterator<Token> for Parser
{
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Parser { tokens: iter.into_iter().collect_vec(), ptr: 0 }
    }
}

impl Parser
{
	pub fn parse(&mut self) -> Option<Expr> {
		self.parse_expr(0)
	}

	fn parse_expr(&mut self, last_power: u32) -> Option<Expr> {
		let mut expr = self.parse_atom()?;

		loop {
			let next_c = match self.peek()
			{
				Some(c) => c,
				None => break,
			};

			let (power, op) = match next_c {
				Token::And => (30, Operation::Conjunction),
				Token::Or => (20, Operation::Disjunction),
				Token::NotEquals => (10, Operation::Inequality),
				Token::Equals | Token::Biconditional => (10, Operation::Equality),

				Token::LeadsTo => (10, Operation::Conditional),

				_ => break
			};

			if power < last_power {
				break
			}

			self.skip();

			let right = Box::new(self.parse_expr(power)?);
			let left = Box::new(std::mem::replace(&mut expr, Expr::False));

			expr = Expr::Operation(op, left, right);
		}

		return Some(expr)
	}

	fn parse_atom(&mut self) -> Option<Expr>
	{
		let expr = match self.peek()?
		{
			Token::Lowercase(c) => {
				self.skip();
				Expr::Variable(c)
			}

			Token::True => {
				self.skip();
				Expr::True
			},
			Token::False => {
				self.skip();
				Expr::False
			},

			Token::Negate => {
				self.skip();
				let unit = self.parse_expr(40)?;
				Expr::Negate(Box::new(unit))
			},
			Token::Open => {
				self.skip();
				let expr = self.parse_expr(0)?;
				match self.next()? {
					Token::Close => { expr },
					_ => return None
				}
			}

			_ => return None
		};

		Some(expr)
	}

	fn next(&mut self) -> Option<Token>
	{
		let token = self.peek();
		self.skip();
		token
	}

	fn peek(&self) -> Option<Token>
	{
		self.tokens.get(self.ptr).cloned()
	}

	fn skip(&mut self)
	{
		self.ptr += 1;
	}
}