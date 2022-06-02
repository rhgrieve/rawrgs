use std::{env};

struct Args {
    args: Vec<Arg>,
}

impl Args {
    fn new() -> Args {
        Args { args: vec![] }
    }

    fn push(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        return self;
    }

    fn match_long(&self, long_name: &String) -> Option<&Arg> {
        for arg in &self.args {
            if arg.long == long_name {
                return Some(arg);
            }
        }

        return None;
    }
}

#[derive(Debug)]
pub struct ArgMatch {
    name: String,
    value: Option<String>,
}

#[derive(Debug)]
pub struct ArgMatches {
    matches: Vec<ArgMatch>,
}

impl ArgMatches {
    fn new() -> ArgMatches {
        ArgMatches { matches: vec![] }
    }

    fn add(mut self, name: String, value: Option<String>) -> Self {
        self.matches.push(ArgMatch {
            name: name,
            value: value,
        });

        return self;
    }

    pub fn value_of(self, name: &str) -> Option<String> {
        for arg in self.matches {
            if name == arg.name {
                return arg.value
            } 
        }
        return None
    }
}

pub struct Arg {
    name: &'static str,
    short: &'static str,
    long: &'static str,
    pub takes_value: bool,
    value_name: &'static str,
    help: &'static str,
}

impl Arg {
    pub fn with_name(name: &'static str) -> Arg {
        Arg {
            name: name,
            short: "",
            long: "",
            takes_value: false,
            value_name: "",
            help: "",
        }
    }

    pub fn short(mut self, short: &'static str) -> Arg {
        self.short = short;
        return self;
    }

    pub fn long(mut self, long: &'static str) -> Arg {
        self.long = long;
        return self;
    }

    pub fn takes_value(mut self, takes_value: bool) -> Arg {
        self.takes_value = takes_value;
        return self;
    }

    pub fn value_name(mut self, value_name: &'static str) -> Arg {
        self.value_name = value_name;
        return self;
    }

    pub fn help(mut self, help: &'static str) -> Arg {
        self.help = help;
        return self;
    }
}

pub struct App {
    name: &'static str,
    author: &'static str,
    version: &'static str,
    about: &'static str,
    matches: ArgMatches,
    args: Args,
    bin_path: String,
}

impl App {
    pub fn new(name: &'static str) -> App {
        App {
            name: name,
            author: "",
            version: "",
            about: "",
            args: Args::new(),
            matches: ArgMatches::new(),
            bin_path: String::new(),
        }
    }

    pub fn author(mut self, author: &'static str) -> Self {
        self.author = author;
        return self;
    }

    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
        return self;
    }

    pub fn about(mut self, about: &'static str) -> Self {
        self.about = about;
        return self;
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args = self.args.push(arg);
        return self;
    }

    pub fn get_matches(mut self) -> ArgMatches {
        let args: Vec<String> = env::args().collect();

        let mut ate_path = false;
        for current_arg in args {
            if !ate_path {
                self.bin_path = current_arg;
                ate_path = true;
                continue;
            }
            
            if current_arg.starts_with("--") {
                let mut input_arg_name = String::new();
                let mut input_arg_value = String::new();
            
                if current_arg.contains("=") {
                    let mut has_hit_pivot = false;
                    for ch in current_arg.trim_start_matches("-").chars() {                        
                        if ch == '=' {
                            has_hit_pivot = true;
                        } else {
                            if has_hit_pivot {
                                input_arg_value.push(ch);
                            } else {
                                input_arg_name.push(ch);
                            }
                        }
                    }
                } else {
                    input_arg_name = current_arg.trim_start_matches("-").to_string();
                }

                if let Some(matched_arg) = self.args.match_long(&input_arg_name) {
                    if matched_arg.takes_value && input_arg_value.is_empty() {
                        panic!("Arg {} takes a value but none was supplied", input_arg_name)
                    } else if !matched_arg.takes_value && !input_arg_value.is_empty() {
                        panic!("Arg {} does not take a value but {} was supplied", input_arg_name, input_arg_value)
                    } else {
                        let value_to_add = match input_arg_value.is_empty() {
                            true => None,
                            false => Some(input_arg_value)
                        };

                        self.matches = self.matches.add(matched_arg.name.to_string(), value_to_add);
                    }
                }
            }
                
            // } else if current_arg.starts_with("-") {
                
            // }
        }

        return self.matches;

        // if args.len() <= 1 {
        //     println!("{} {}", self.name, self.version);
        //     println!("Usage: {} [option]", self.name.to_ascii_lowercase())
        // } else {
        //     println!("{:?}", args);
        // }
    }
}