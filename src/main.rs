use rand::seq::SliceRandom;
use regex::Regex;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn parse(input: &str) -> Vec<(String, String)> {
    let r = Regex::new(
        r"(?x)
        (Q:|)\ *\n*(?P<Q>(.|\n)*?)
        A: *\n*(?P<A>(.|\n)*?)(Q:|\z)
        ",
    )
    .unwrap();
    fn part(caps: &regex::Captures, name: &str) -> String {
        "  ".to_string()
            + &caps
                .name(name)
                .expect("parse error")
                .as_str()
                .trim_end()
                .replace('\n', "\r\n  ")
    }

    r.captures_iter(input)
        .map(|caps| (part(&caps, "Q"), part(&caps, "A")))
        .collect()
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut flashcards = parse(
        &std::fs::read_to_string(
            std::env::args()
                .nth(1)
                .expect("please specify the flashcard file in the first argument"),
        )
        .expect("couldn't read file"),
    );
    flashcards.shuffle(&mut rng);

    let mut i = 0;
    let mut on = false;

    print!("{}", termion::cursor::Hide);

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    fn print_screen(flashcards: &Vec<(String, String)>, i: usize, on: bool) {
        print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);

        if i == flashcards.len() {
            print!("\r\n  finished! press left to go back, right to reshuffle and start over.");
        } else {
            print!(
                "\r\n  Use the arrow keys to move around, and press q to exit.\r\n\r\n  ({}/{})\r\n\r\n",
                i + 1,
                flashcards.len(),
            );
            print!("{}\r\n\r\n", flashcards[i].0);
            if on {
                print!("{}\r\n\r\n", flashcards[i].1);
            }
        }
    }

    print_screen(&flashcards, i, on);
    stdout.flush().unwrap();

    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') => break,
            Key::Left => {
                if i > 0 {
                    i -= 1;
                    on = false;
                }
            }
            Key::Right => {
                if i == flashcards.len() {
                    flashcards.shuffle(&mut rng);
                    i = 0;
                } else {
                    i += 1;
                }
                on = false;
            }
            Key::Down | Key::Up => on = !on,
            _ => {}
        }
        print_screen(&flashcards, i, on);
        stdout.flush().unwrap();
    }
    print!("{}", termion::cursor::Show);
    stdout.flush().unwrap();
}
