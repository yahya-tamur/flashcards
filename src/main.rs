#![feature(iter_intersperse)]
use rand::seq::SliceRandom;
use regex::Regex;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn parse(input: &str) -> (Vec<(String, String)>, usize) {
    let card_width: usize = input
        .chars()
        .take_while(|x| x.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let second_line = input.find('\n').unwrap();
    let (_, input) = input.split_at(second_line + 1);

    let r = Regex::new(
        r"(?x)
        (Q.*?:)?\ *\n*(?P<Q>(.|\n)*?)
        A:\ *\n*(?P<A>(.|\n)*?)((Q.*?:)|\z)
        ",
    )
    .unwrap();
    fn part(caps: &regex::Captures, name: &str, card_width: usize) -> String {
        caps.name(name)
            .expect("parse error")
            .as_str()
            .trim_end()
            .lines()
            .map(|line| format!("  │  {:card_width$}  │  \r\n", line))
            .collect()
    }

    (
        r.captures_iter(input)
            .map(|caps| (part(&caps, "Q", card_width), part(&caps, "A", card_width)))
            .collect(),
        card_width,
    )
}

struct State {
    flashcards: Vec<(String, String)>,
    card_width: usize,
    starred: Vec<bool>,
    num_starred: usize,
    indices: Vec<usize>,
    i: usize,
    on: bool,
}

fn main() {
    let mut rng = rand::thread_rng();
    let (flashcards, card_width) = parse(
        &std::fs::read_to_string(
            std::env::args()
                .nth(1)
                .expect("please specify the flashcard file in the first argument"),
        )
        .expect("couldn't read file"),
    );
    let n = flashcards.len();

    let mut s = State {
        flashcards,
        card_width,
        starred: vec![false; n],
        num_starred: 0,
        indices: (0..100).collect(),
        i: 0,
        on: false,
    };

    s.indices.shuffle(&mut rng);

    print!("{}", termion::cursor::Hide);

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    fn print_screen(s: &State) {
        print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);

        if s.i == s.indices.len() {
            print!(
                "\r\n  finished! press left to go back, enter to reshuffle and start over.\r\n\r\n"
            );
            print!(
                "  {} out of {} starred.\r\n\r\n",
                s.num_starred,
                s.flashcards.len()
            );
            print!("  ({}/{})\r\n\r\n", s.indices.len(), s.indices.len());
            if !s.on {
                print!("  will study starred cards only. press up or down to flip\r\n\r\n");
            } else {
                print!("  will study all cards. press up or down to flip\r\n\r\n");
            }
        } else {
            print!("\r\n  use arrow keys to move, enter to star or unstar, and q to exit.\r\n\r\n");
            print!(
                "  {} out of {} starred.\r\n\r\n",
                s.num_starred,
                s.flashcards.len()
            );
            print!(
                "  ({}/{}) {}\r\n\r\n",
                s.i + 1,
                s.indices.len(),
                if s.starred[s.indices[s.i]] {
                    "Starred"
                } else {
                    "Not Starred"
                }
            );
            print!("  ┌──{:─<1$}──┐\r\n", "", s.card_width);
            print!("{}", s.flashcards[s.indices[s.i]].0);
            if s.on {
                print!("  ├──{:─<1$}──┤\r\n", "", s.card_width);
                print!("{}", s.flashcards[s.indices[s.i]].1);
            }
            print!("  └──{:─<1$}──┘\r\n\r\n", "", s.card_width);
        }
    }

    print_screen(&s);
    stdout.flush().unwrap();

    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if s.i == s.indices.len() {
                    if !s.on {
                        s.indices = (0..100).filter(|i| s.starred[*i]).collect();
                    } else {
                        s.indices = (0..100).collect();
                    }
                    s.indices.shuffle(&mut rng);
                    s.i = 0;
                    s.on = false;
                } else {
                    if s.starred[s.indices[s.i]] {
                        s.num_starred -= 1;
                    } else {
                        s.num_starred += 1;
                    }
                    s.starred[s.indices[s.i]] ^= true;
                }
            }
            Key::Char('q') | Key::Ctrl('c') => break,
            Key::Left if s.i > 0 => {
                s.i -= 1;
                s.on = false;
            }
            Key::Right if s.i < s.indices.len() => {
                s.i += 1;
                s.on = false;
            }
            Key::Down | Key::Up => s.on ^= true,
            _ => {}
        }
        print_screen(&s);
        stdout.flush().unwrap();
    }
    print!("{}", termion::cursor::Show);
    stdout.flush().unwrap();
}
