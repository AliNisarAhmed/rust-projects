use rand;
use rand::seq::SliceRandom;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{stdin, Read};

const LETTERS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const SPECIAL_CHARS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];
const NUMBERS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let mut stdin = stdin();

    let mut min_len = String::new();
    let mut num_special_chars = String::new();
    let mut num_numbers = String::new();

    println!("What's the minimum length");

    stdin.read_line(&mut min_len).unwrap();

    let min_len = min_len.trim().parse::<usize>().unwrap();

    println!("How many special characters?");

    stdin.read_line(&mut num_special_chars).unwrap();

    let num_special_chars = num_special_chars.trim().parse::<usize>().unwrap();

    println!("How many numbers?");

    stdin.read_line(&mut num_numbers).unwrap();

    let num_numbers = num_numbers.trim().parse::<usize>().unwrap();

    let password = generate_password(min_len, num_special_chars, num_numbers);

    // copy_password_to_clipboard(password);

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password).unwrap();

    println!("Your password has been copied to the clipboard!");

    println!("Store your password somewhere safe!, then press any button to exit");

    stdin.read(&mut [0]).unwrap();

}

fn generate_password(min_len: usize, num_special_chars: usize, num_numbers: usize) -> String {
    let mut rng = &mut rand::thread_rng();

    let mut sp: Vec<_> = SPECIAL_CHARS
        .choose_multiple(&mut rng, num_special_chars as usize)
        .cloned()
        .collect();

    let mut ns: Vec<char> = NUMBERS
        .choose_multiple(&mut rng, num_numbers as usize)
        .cloned()
        .collect();

    let mut ls: Vec<char> = LETTERS
        .choose_multiple(
            &mut rng,
            (min_len - num_numbers - num_special_chars) as usize,
        )
        .cloned()
        .collect();

    sp.append(&mut ns);
    sp.append(&mut ls);

    sp.shuffle(&mut rng);

    sp.into_iter().collect()

}
