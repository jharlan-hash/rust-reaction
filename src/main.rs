use std::{time};
use ratatui::widgets::Paragraph;
use ratatui::prelude::*;

use crossterm::{terminal::{
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
}, event::KeyCode::Char};

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::{Block, Borders};

fn render_widgets(f: &mut Frame, typed_characters: &str, last_char: &char, first_char: &char) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    f.render_widget(
        Paragraph::new(typed_characters.to_string())
            .block(Block::default().title("Input".to_string()).borders(Borders::ALL)),
        layout[0],
    );

    f.render_widget(
        Paragraph::new(format!("Value of last_char = '{}', value of first_char = '{}' ", last_char, first_char).to_string())
            .block(Block::default().title("Output".to_string()).borders(Borders::ALL)),
        layout[1],
    );

    Ok(())
}

fn main() -> Result<()>{
    let mut typed_characters = String::new();
    enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    terminal.draw(|f| { let _ = render_widgets(f, &typed_characters, &'0', &'0'); })?;

    loop {
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
        if crossterm::event::poll(time::Duration::from_millis(250))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    let matched_key = match key.code {
                        Char('q') => {
                            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen)?;
                            disable_raw_mode()?;
                            break;
                        },
                        Char(' ') => ' ',
                        Char('0') => '0',
                        Char('1') => '1',
                        Char('2') => '2',
                        Char('3') => '3',
                        Char('4') => '4',
                        Char('5') => '5',
                        Char('6') => '6',
                        Char('7') => '7',
                        Char('8') => '8',
                        Char('9') => '9',
                        Char('a') => 'a',
                        Char('b') => 'b',
                        Char('c') => 'c',
                        Char('d') => 'd',
                        Char('e') => 'e',
                        Char('f') => 'f',
                        Char('g') => 'g',
                        Char('h') => 'h',
                        Char('i') => 'i',
                        Char('k') => 'k',
                        Char('l') => 'l',
                        Char('m') => 'm',
                        Char('n') => 'n',
                        Char('o') => 'o',
                        Char('p') => 'p',
                        Char('r') => 'r',
                        Char('s') => 's',
                        Char('t') => 't',
                        Char('u') => 'u',
                        Char('v') => 'v',
                        Char('w') => 'w',
                        Char('x') => 'x',
                        Char('y') => 'y',
                        Char('z') => 'z',
                        Char('A') => 'A',
                        Char('B') => 'B',
                        Char('C') => 'C',
                        Char('D') => 'D',
                        Char('E') => 'E',
                        Char('F') => 'F',
                        Char('G') => 'G',
                        Char('H') => 'H',
                        Char('I') => 'I',
                        Char('K') => 'K',
                        Char('L') => 'L',
                        Char('M') => 'M',
                        Char('N') => 'N',
                        Char('O') => 'O',
                        Char('P') => 'P',
                        Char('R') => 'R',
                        Char('S') => 'S',
                        Char('T') => 'T',
                        Char('U') => 'U',
                        Char('V') => 'V',
                        Char('W') => 'W',
                        Char('X') => 'X',
                        Char('Y') => 'Y',
                        Char('Z') => 'Z',
                        Char('.') => '.',
                        Char('+') => '+',
                        Char('=') => '=',
                        KeyCode::Backspace => {
                            typed_characters.pop();
                            terminal.draw(|f| { let _ = render_widgets(f, &typed_characters, &'0', &'0'); })?;
                            continue;
                        },
                        _ => continue,
                    };

                    let current_character = matched_key; // 2
                    typed_characters.push(match current_character {
                                        '1' => '₁',
                                        '2' => '₂',
                                        '3' => '₃',
                                        '4' => '₄',
                                        '5' => '₅',
                                        '6' => '₆',
                                        '7' => '₇',
                                        '8' => '₈',
                                        '9' => '₉',
                                        '0' => '₀',
                                        _ => matched_key,
                    });

                    terminal.draw(|f| { let _ = render_widgets(f, &typed_characters, &'0', &'0'); })?;
                }
            }
        }
    }

    crossterm::execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}