use rand;
use rand::seq::SliceRandom;

fn main() {
    let stdin = std::io::stdin();

    let mut min_len = String::new();
    let mut num_special_chars = String::new();
    let mut num_numbers = String::new();

    println!("What's the minimum length");

    stdin.read_line(&mut min_len).unwrap();

    let min_len = min_len.trim().parse::<u32>().unwrap();

    println!("How many special characters?");

    stdin.read_line(&mut num_special_chars).unwrap();

    let num_special_chars = num_special_chars.trim().parse::<u32>().unwrap();

    println!("How many numbers?");

    stdin.read_line(&mut num_numbers).unwrap();

    let num_numbers = num_numbers.trim().parse::<u32>().unwrap();

    let letters: &[char] = &[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let special_chars: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];
    let numbers: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut rng = &mut rand::thread_rng();

    let mut sp: Vec<_> = special_chars
        .choose_multiple(&mut rng, num_special_chars as usize)
        .cloned()
        .collect();

    let mut ns: Vec<char> = numbers
        .choose_multiple(&mut rng, num_numbers as usize)
        .cloned()
        .collect();

    let mut ls: Vec<char> = letters
        .choose_multiple(
            &mut rng,
            (min_len - num_numbers - num_special_chars) as usize,
        )
        .cloned()
        .collect();

    sp.append(&mut ns);
    sp.append(&mut ls);

    sp.shuffle(&mut rng);

    println!("sample: {}", sp.into_iter().collect::<String>());
}
