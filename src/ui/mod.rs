use std::{io, thread::{self}, time::Duration, error::Error};

use crossterm::{execute, terminal::{EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyEvent, KeyCode, KeyEventKind, KeyModifiers}};
use itertools::Itertools;
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders, Paragraph, Table, Row, Cell}, layout::{Rect, Layout, Direction, Constraint}, style::{Style, Color}, Frame, text::Text};

use crate::{parser::{Lexer, Parser, Expr}, solver::{TruthTable, self}};

pub enum State {
	Input,
	Display,
	Closed
}

pub struct App {
	state: State,

	last2: char,
	last: char,

	input: String,
	truth_table: Option<TruthTable>
}

impl Default for App {
    fn default() -> Self {
        Self {
			state: State::Input,
			last: ' ',
			last2: ' ',
			input: Default::default(),
			truth_table: None,
		}
    }
}

impl App {
	pub fn run(mut self) -> Result<(), Box<dyn Error>> {
		// setup terminal
		enable_raw_mode();
		let mut stdout = io::stdout();
		execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
		let backend = CrosstermBackend::new(stdout);
		let mut terminal = Terminal::new(backend).unwrap();

		loop {
			terminal.draw(|f| ui(f, &self));

			self.process_input();

			if let State::Closed = self.state {
				break
			}
		}

		// cleanup terminal
		disable_raw_mode();
		execute!(
			terminal.backend_mut(),
			LeaveAlternateScreen,
			DisableMouseCapture
		);
		terminal.show_cursor();

		Ok(())
	}

	pub fn process_input(&mut self) {
		if let Ok(Event::Key(key)) = event::read() {
			if key.modifiers.contains( KeyModifiers::CONTROL ) && key.code == KeyCode::Char('z') {
				self.state = State::Closed;
				return
			}

			match self.state {
				State::Input => self.process_input_input(key),
				State::Display => self.process_input_display(key),
				State::Closed => {}
			}
		}

		
	}

	fn input_char(&mut self, c: char) {
		match (self.last2, self.last, c) {
			(_, '!', '=') => {
				self.input.pop();
				self.input.push('≢');
			}

			('<', '-', '>') => {
				self.input.pop();
				self.input.pop();
				self.input.push('↔');
			}

			(_, '-', '>') => {
				self.input.pop();
				self.input.push('→');
			}

			_ => {
				self.last2 = self.last;
				self.last = c;

				let adj_c = match c {
					'|' | '+' => '∨',
					'&' | '*' => '∧',
					'!' | '-' => '¬',
					'=' => '≡',
					c => c
				};

				self.input.push(adj_c);
			}
		}
	}

	pub fn process_input_input(&mut self, key: KeyEvent) {


		match key.code {
			KeyCode::Enter => {
				self.submit();
				/* submit */
			},

			KeyCode::Char(c) => {
				self.input_char(c);
			}

			KeyCode::Backspace => {
				self.input.pop();
			}

			_ => {}
		}
	}

	pub fn process_input_display(&mut self, key: KeyEvent) {
		self.input.clear();

		self.state = State::Input;

		match key.code {
			KeyCode::Char(c) => {
				self.input_char(c);
			}

			_ => {}
		}
	}

	pub fn submit(&mut self) {
		let lexer = Lexer::new(self.input.clone());
		let mut parse: Parser = lexer.collect();

		if let Some(expr) = parse.parse() {
			self.truth_table = Some(solver::solve_for_table(expr));
			self.state = State::Display;
		} else {
			// there is an error
			self.input.clear();
		}
	}
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
	match app.state {
		State::Input => ui_input(f, app),
		State::Display => ui_display(f, app),
		State::Closed => {}
	}
}

pub fn ui_input<B: Backend>(f: &mut Frame<B>, app: &App) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([
			Constraint::Length(3),
			Constraint::Min(1),
		].as_ref())
		.split(f.size());

	let msg = Text::from(app.input.as_str());
	let input_message = Paragraph::new(msg)
		.block(Block::default()
			.borders(Borders::ALL)
			.title("Proposition"));

	f.render_widget(input_message, chunks[0]);
}

pub fn ui_display<B: Backend>(f: &mut Frame<B>, app: &App) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([
			Constraint::Length(3),
			Constraint::Min(1),
		].as_ref())
		.split(f.size());

	let msg = Text::from(app.input.as_str());
	let input_message = Paragraph::new(msg)
		.block(Block::default()
			.borders(Borders::ALL)
			.title("Proposition"));

	f.render_widget(input_message, chunks[0]);

	if let Some(truth_table) = &app.truth_table {
		let rows = truth_table.inputs
			.iter()
			.zip(&truth_table.outputs)
			.map(|(inputs, output)| Row::new(
					inputs.iter()
						  .map(|b| match b {
							true => Cell::from("T"),
							false => Cell::from("F"),
						  })
						  .chain(std::iter::once(match output {
							true => Cell::from("T"),
							false => Cell::from("F"),
						  }))
						  .collect_vec()
				))
			.collect_vec();

		let widths = truth_table.input_vars.iter()
			.map(|_| Constraint::Length(1))
			.chain(std::iter::once(Constraint::Length(app.input.len() as u16)))
			.collect_vec();

		let header = Row::new(
			truth_table.input_vars.iter()
				.map(|c| {
					format!("{c}")
				})
				.chain(std::iter::once(app.input.clone()))
				.collect_vec()
		);

		let table = Table::new(rows)
			.style(Style::default().fg(Color::White))
			.widths(&widths)
			.header(header)
			.block(Block::default()
			.borders(Borders::ALL)
			.title("Truth Table"));

		f.render_widget(table, chunks[1]);
	}
}