use colored::Colorize;
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let mut count: u64 = 0;
    let mut delay: f64 = 2.0;

    println!("Let's play a game, shall we? Start with 1 and count along with me.\nThat is, if you can keep up.\n");

    loop {
        count = count + 1;

        if count == 1 {
            print!("{}", style_text(color_text(bold_text(count.to_string()))));
        } else {
            print!(", {}", style_text(color_text(bold_text(count.to_string()))));
        }

        if count == 10000 {
            print!("\n\n");
            break;
        }

        io::stdout().flush().unwrap();

        delay = delay * 95.0 / 100.0;
        thread::sleep(time::Duration::from_secs_f64(delay));
    }

    let message1 = "heh, pathetic".chars().collect::<Vec<char>>();
    let message2 = "like a dying beacon,".chars().collect::<Vec<char>>();
    let message3 = " it shines through you -".chars().collect::<Vec<char>>();
    let message4 = " your mediocrity".chars().collect::<Vec<char>>();
    let message5 = "y o u  i d i o t.".chars().collect::<Vec<char>>();

    thread::sleep(time::Duration::from_secs_f64(4.0));

    for character in message1.iter() {
        thread::sleep(time::Duration::from_secs_f64(0.1));
        print!("{}", character.to_string());
        io::stdout().flush().unwrap();
    }

    print!("\n");

    thread::sleep(time::Duration::from_secs_f64(2.0));

    for character in message2.iter() {
        thread::sleep(time::Duration::from_secs_f64(0.1));
        print!("{}", character.to_string());
        io::stdout().flush().unwrap();
    }

    thread::sleep(time::Duration::from_secs_f64(1.0));

    for character in message3.iter() {
        thread::sleep(time::Duration::from_secs_f64(0.1));
        print!("{}", character.to_string());
        io::stdout().flush().unwrap();
    }

    thread::sleep(time::Duration::from_secs_f64(1.0));

    for character in message4.iter() {
        thread::sleep(time::Duration::from_secs_f64(0.1));
        print!("{}", character.to_string());
        io::stdout().flush().unwrap();
    }

    thread::sleep(time::Duration::from_secs_f64(0.3));

    print!("\n");

    thread::sleep(time::Duration::from_secs_f64(3.0));

    for character in message5.iter() {
        thread::sleep(time::Duration::from_secs_f64(0.3));
        print!("{}", character.to_string());
        io::stdout().flush().unwrap();
    }

    print!("\n");

    thread::sleep(time::Duration::from_secs_f64(2.0));
}

fn bold_text(original: String) -> String {
    let random = rand::thread_rng().gen_range(1..4);

    if random == 1 | 2 | 3 {
        original.bold().to_string()
    } else {
        original
    }
}

fn style_text(original: String) -> String {
    let random = rand::thread_rng().gen_range(1..7);

    match random {
        1 => original.dimmed().to_string(),
        2 => original.italic().to_string(),
        3 => original.normal().to_string(),
        4 => original.reverse().to_string(),
        5 => original.strikethrough().to_string(),
        6 => original.underline().to_string(),
        _ => original.blink().to_string(),
    }
}

fn color_text(original: String) -> String {
    let random = rand::thread_rng().gen_range(1..9);

    match random {
        1 => original.blue().to_string(),
        2 => original.cyan().to_string(),
        3 => original.green().to_string(),
        4 => original.magenta().to_string(),
        5 => original.purple().to_string(),
        6 => original.red().to_string(),
        7 => original.white().to_string(),
        8 => original.yellow().to_string(),
        _ => original.blink().to_string(),
    }
}
