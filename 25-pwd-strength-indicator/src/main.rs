pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Strong,
    VeryStrong,
}

impl ToString for PasswordStrength {
    fn to_string(&self) -> String {
        match self {
            PasswordStrength::VeryWeak => String::from("very weak"),
            PasswordStrength::Weak => String::from("weak"),
            PasswordStrength::Strong => String::from("strong"),
            PasswordStrength::VeryStrong => String::from("very strong"),
        }
    }
}

fn main() {
    println!("Please enter a password to strength check");

    let stdin = std::io::stdin();

    let mut password = String::new();

    stdin.read_line(&mut password).unwrap();

    let password = password.trim().to_owned();

    let pwd_str: PasswordStrength = calculate_pwd_strength(&password);

    println!(
        "The password '{password}' is a {pwd_str_string} password.",
        pwd_str_string = pwd_str.to_string()
    )
}

fn calculate_pwd_strength(pwd: &String) -> PasswordStrength {
    let len = pwd.len();
    let all_numbers = pwd.chars().all(|c| c.is_numeric());
    let all_letters = pwd.chars().all(|c| c.is_ascii_alphabetic());
    let any_number = pwd.chars().any(|c| c.is_numeric());
    let any_letter = pwd.chars().any(|c| c.is_ascii_alphabetic());
    let any_special_char = pwd.chars().any(|c| c.is_ascii_punctuation());

    if len < 8 && all_numbers {
        return PasswordStrength::VeryWeak;
    }
    if len < 8 && all_letters {
        return PasswordStrength::Weak;
    }

    if len >= 8 && any_letter && any_number && !any_special_char {
        return PasswordStrength::Strong;
    }

    if len >= 8 && any_letter && any_number && any_special_char {
        return PasswordStrength::VeryStrong;
    }

    PasswordStrength::Weak
}
