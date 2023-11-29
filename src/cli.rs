use std::fmt::Display;
pub struct Cli {
    pub mode: String,
    pub length: u32,
    pub show_help: bool,
}

impl Cli {
    pub fn build(args: Vec<String>) -> Result<Cli, &'static str> {
        let show_help = Self::must_show_help(&args);
        if show_help {
            return Ok(Self {
                mode: String::new(),
                length: 0,
                show_help: true,
            });
        }

        let mode = match Self::parse_mod_command(&args) {
            Some(res) => res,
            None => return Err("Wrong mode!"),
        };

        let length = match Self::parse_length_command(&args) {
            Some(res) => res,
            None => return Err("Wrong length!"),
        };
        Ok(Self {
            mode: mode,
            length: length,
            show_help: show_help,
        })
    }

    fn must_show_help(args: &[String]) -> bool {
        if args.len() == 1 {
            return true;
        }
        args.iter().any(|arg| arg.contains("--help"))
    }

    fn parse_mod_command(args: &[String]) -> Option<String> {
        for arg in args.iter() {
            if arg.contains("--mod") {
                if let Some((_, value)) = arg.split_once('=') {
                    if Self::validate_mod_value(value) {
                        return Some(value.to_string());
                    } else {
                        return None;
                    }
                }
            }
        }

        None
    }

    fn validate_mod_value(value: &str) -> bool {
        let allowed_chars = ['s', 'l', 'u', 'n'];
        value.chars().all(|c| allowed_chars.contains(&c))
    }

    fn validate_length_value(value: &str) -> Option<u32> {
        match value.parse::<u32>() {
            Ok(length) if (4..=91).contains(&length) => Some(length),
            _ => None,
        }
    }

    fn parse_length_command(args: &[String]) -> Option<u32> {
        for arg in args.iter() {
            if arg.contains("--length") {
                if let Some((_, value)) = arg.split_once("=") {
                    if let Some(length) = Self::validate_length_value(value) {
                        return Some(length);
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(12)
    }

    pub fn help() -> &'static str {
        let help = "
        passgen --mod=slu --length=16
        Generate a random password with specified options.
        Options:
          --mod=<MOD>          Specify the character sets to include in the password. Available mods:
                                 slu - Lowercase, Uppercase, and Special characters.
          --length=<length>    Specify the length of the generated password. Default is 12.
        Examples:
          passgen --mod=slu --length=16
              Generate a password with lowercase, uppercase, and special characters, with a length of 16.
          passgen --mod=slu
              Generate a password with default length (12) using lowercase, uppercase, and special characters.
        ";

        help
    }
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "mode:{}, length:{}, show help:{}",
            self.mode, self.length, self.show_help
        )
    }
}
