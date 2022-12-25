use std::io::{self, prelude::*, stdin, BufReader};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, ToString};

struct StringReader {
    reader: Box<dyn std::io::BufRead>,
}

impl StringReader {
    fn read_line(&mut self) -> String {
        let mut result = String::new();
        self.reader
            .read_line(&mut result)
            .expect("Error reading from the command line");
        String::from(result.trim())
    }
}

pub struct CommandLineReader {
    string_reader: StringReader,
}

impl CommandLineReader {
    pub fn new() -> Self {
        CommandLineReader {
            string_reader: StringReader {
                reader: Box::new(BufReader::new(stdin())),
            },
        }
    }

    pub fn ask(&mut self, question: &str) -> String {
        loop {
            print!("{}", question);
            io::stdout().flush().expect("Error flushing stdout");
            let answer = self.string_reader.read_line();
            if !answer.is_empty() {
                return answer;
            }
        }
    }

    /// use strum_macros::{Display, EnumIter, EnumString}
    pub fn options<T: IntoEnumIterator + std::fmt::Display + std::str::FromStr + Copy>(
        &mut self,
        question: &str,
    ) -> T {
        let options_string = T::iter()
            .enumerate()
            .map(|(i, variant)| format!("{}) {}", i, variant))
            .collect::<Vec<_>>()
            .join(", ");
        loop {
            println!("{} [{}]: ", question, options_string);
            let answer = self.string_reader.read_line();
            if let Ok(result_index) = answer.parse::<usize>() {
                let options: Vec<T> = T::iter().collect();
                if result_index < options.len() {
                    return options[result_index];
                }
            }
            if let Ok(result) = answer.parse::<T>() {
                return result;
            };
        }
    }
}

struct Cli {
    reader: CommandLineReader,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            reader: CommandLineReader::new(),
        }
    }

    pub fn show_creating_file() {}
}
