#[derive(Debug, Copy, Clone)]
pub enum Token
{
	Lowercase(char),
	Uppercase(char),

	True,
	False,

	Negate,
	Or,
	And,

	Equals,
	NotEquals,

	LeadsTo,
	Biconditional,

	Open,
	Close,
}

pub struct Lexer
{
	chars: Vec<char>,
	pointer: usize,
}

impl Lexer
{
	pub fn new(equation: String) -> Self {
		Self {
			chars: equation.chars().collect(),
			pointer: 0,
		}
	}

	fn peek(&mut self) -> Option<char>
	{
		while self.chars.get(self.pointer)?.is_whitespace() {
			self.pointer += 1;
		}
		let c = *self.chars.get(self.pointer)?;
		self.pointer += 1;
		Some(c)
	}
}

impl Iterator for Lexer
{
	type Item = Token;

	fn next(&mut self) -> Option<Token>
	{
		let token = match self.peek()? {
			'|' | '+' | '∨' => Token::Or,
			'&' | '*' | '∧' => Token::And,
			'-' | '¬' => Token::Negate,

			'!' => {
				self.skip(1);
				match self.peek()? {
					'=' => Token::NotEquals,
					_ => return Some(Token::Negate),
				}
			}

			'=' | '≡' => Token::Equals,
			'≢' => Token::NotEquals,

			'→' => Token::LeadsTo,
			'↔' => Token::Biconditional,

			'T' => Token::True,
			'F' => Token::False,

			'(' => Token::Open,
			')' => Token::Close,

			c if c.is_ascii_lowercase() => Token::Lowercase(c),
			c if c.is_ascii_uppercase() => Token::Uppercase(c),

			_ => return None
		};
		self.skip(1);

		Some(token)
	}
}