use crate::cli::Cli;
use rand::Rng;

const LOWERCASES: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const UPPERCASES: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL_CHARS: [char; 29] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '_', '+', '[', ']', '{', '}', '|',
    ';', ':', '\'', '\"', ',', '.', '<', '>', '/', '?',
];

fn determine_char_pool(mode: &str) -> Vec<char> {
    mode.chars().flat_map(|c| get_pool_by_mod(c)).collect()
}

fn get_pool_by_mod(c: char) -> Vec<char> {
    match c {
        'u' => UPPERCASES.to_vec(),
        'l' => LOWERCASES.to_vec(),
        'n' => NUMBERS.to_vec(),
        's' => SPECIAL_CHARS.to_vec(),
        _ => vec![],
    }
}

fn get_initial_password(cli: &Cli) -> Vec<char> {
    let mut password: Vec<char> = Vec::new();
    for s in cli.mode.chars() {
        let v = get_pool_by_mod(s);
        let rnd = rand::thread_rng().gen_range(0..v.len());
        password.push(v[rnd])
    }
    password
}

pub fn generate_password(cli: &Cli) -> String {
    let mut password: Vec<char> = Vec::new();
    let pool = determine_char_pool(&cli.mode);
    let initial_password: Vec<char> = get_initial_password(&cli);
    let pool_length = pool.len();
    for _item in 0..cli.length {
        let rand = rand::thread_rng().gen_range(0..pool_length);
        password.push(pool[rand])
    }
    password.extend(initial_password);
    password.into_iter().collect::<String>()
}
