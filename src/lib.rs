use std::{env, process};

use tabular::Tabular;

enum FlagType {
    Long,
    Short,
}

struct Args {
    args: Vec<Arg>,
}

impl Args {
    fn with_defaults() -> Args {
        Args { 
            args: vec!(
                Arg::with_name("help")
                    .short("h")
                    .long("help")
                    .internal(true),
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .internal(true)
            ) 
        }
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

    fn match_short(&self, short_name: &String) -> Option<&Arg> {
        for arg in &self.args {
            if arg.short == short_name {
                return Some(arg);
            }
        }

        return None;
    }

    fn match_positional(&mut self) -> Option<&Arg> {
        for arg in &mut self.args {
            if arg.is_positional() && !arg.is_matched() {
                arg.matched = true;
                return Some(arg);
            }
        }
        return None;
    }

    fn get_options(&self) -> Vec<&Arg> {
        let mut options = vec!();
        for opt in &self.args {
            if !opt.is_positional() {
                options.push(opt);
            }
        }
        return options;
    }

    fn get_positionals(&self) -> Vec<&Arg> {
        let mut positionals = vec!();
        for arg in &self.args {
            if arg.is_positional() {
                positionals.push(arg);
            }
        }
        return positionals;
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

impl IntoIterator for ArgMatches {
    type Item = ArgMatch;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.matches.into_iter()
    }
}

impl ArgMatches {
    fn new() -> ArgMatches {
        ArgMatches { matches: vec!() }
    }

    fn add(&mut self, name: String, value: Option<String>) -> &Self {
        self.matches.push(ArgMatch {
            name: name,
            value: value,
        });

        return self;
    }

    pub fn value_of(&self, name: &str) -> Option<&String> {
        for arg in &self.matches {
            if name == arg.name {
                return arg.value.as_ref();
            }
        }
        return None;
    }

    pub fn is_present(&self, name: &str) -> bool {
        for arg in &self.matches {
            if name == arg.name {
                return true
            }
        }
        return false;
    }

    fn get_matches(&self) -> &Vec<ArgMatch> {
        return &self.matches;
    }
}

pub struct Arg {
    name: &'static str,
    short: &'static str,
    long: &'static str,
    takes_value: bool,
    is_required: bool,
    value_name: &'static str,
    help: &'static str,
    matched: bool,
    internal: bool,
}

impl Arg {
    pub fn with_name(name: &'static str) -> Arg {
        Arg {
            name: name,
            short: "",
            long: "",
            takes_value: false,
            is_required: false,
            value_name: "",
            help: "",
            matched: false,
            internal: false
        }
    }

    pub fn short(mut self, short: &'static str) -> Self {
        self.short = short;
        return self;
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.long = long;
        return self;
    }

    pub fn takes_value(mut self, takes_value: bool) -> Self {
        self.takes_value = takes_value;
        return self;
    }

    pub fn required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        return self;
    }

    pub fn value_name(mut self, value_name: &'static str) -> Self {
        self.value_name = value_name;
        return self;
    }

    pub fn help(mut self, help: &'static str) -> Self {
        self.help = help;
        return self;
    }

    fn internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        return self;
    }

    fn is_positional(&self) -> bool {
        return self.short.is_empty() && self.long.is_empty();
    }

    fn is_matched(&self) -> bool {
        return self.matched;
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
            args: Args::with_defaults(),
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

    pub fn arg(mut self, mut arg: Arg) -> Self {
        if arg.long.is_empty() && !arg.short.is_empty() {
            arg.long = arg.name;
        }

        if self.args.get_positionals().len() > 1 {
            arg.is_required = true;
        }

        self.args = self.args.push(arg);
        return self;
    }

    pub fn parse_argument(&mut self, current_arg: String) -> Result<(), String> {
        if current_arg.starts_with("--") {
            self.consume_flag(&current_arg, FlagType::Long)?
        } else if current_arg.starts_with("-") {
            let short_group = current_arg.trim_start_matches("-");
            for short_flag in short_group.chars() {
                self.consume_flag(&short_flag.to_string(), FlagType::Short)?;
            }
        } else {
            self.consume_positional(&current_arg)?;
        }

        Ok(())
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

            let match_result = self.parse_argument(current_arg);

            if let Err(err) = match_result {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        self.check_internal_flags();
        if let Err(err) = self.validate_positionals() {
            eprintln!("{}", err);
            process::exit(1);
        }

        return self.matches;
    }
    
    fn consume_positional(&mut self, current_arg: &String) -> Result<(), String> {
        let match_result = self.args.match_positional();

        if let Some(arg) = match_result {
            self.matches.add(arg.name.to_string(), Some(current_arg.to_string()));
        } else {
            return Err(format!("Invalid positional argument: {}", current_arg))
        }
        
        Ok(())
    }

    fn consume_flag(&mut self, current_arg: &String, flag_type: FlagType) -> Result<(), String> {
        let mut input_option_name = String::new();
        let mut input_option_value = String::new();

        if current_arg.contains("=") {
            let mut has_hit_pivot = false;
            for ch in current_arg.trim_start_matches("-").chars() {
                if ch == '=' {
                    has_hit_pivot = true;
                } else {
                    if has_hit_pivot {
                        input_option_value.push(ch);
                    } else {
                        input_option_name.push(ch);
                    }
                }
            }
        } else {
            input_option_name = current_arg.trim_start_matches("-").to_string();
        }

        let match_result = match flag_type {
            FlagType::Long => self.args.match_long(&input_option_name),
            FlagType::Short => self.args.match_short(&input_option_name),
        };

        if let Some(matched_arg) = match_result {
            let mut error = String::new();
            if matched_arg.takes_value && input_option_value.is_empty() {
                error = format!("Option {} takes a value but none was supplied", input_option_name)
            } else if !matched_arg.takes_value && !input_option_value.is_empty() {
                error = format!(
                    "Option {} does not take a value but {} was supplied",
                    input_option_name, input_option_value
                );
            } else {
                let value_to_add = match input_option_value.is_empty() {
                    true => None,
                    false => Some(input_option_value),
                };

                self.matches.add(matched_arg.name.to_string(), value_to_add);
            }

            if !error.is_empty() {
                return Err(error);
            }
        } else {
            return Err(format!(
                "{} is not a valid option. Try --help for usage instructions",
                input_option_name
            ));
        }
        Ok(())
    }

    fn check_internal_flags(&self) {
        for flag in self.matches.get_matches() {
            if flag.name == "version" {
                self.display_version();
                std::process::exit(0);
            } else if flag.name == "help" {
                self.display_help();
                std::process::exit(0);
            }
        }
    }
    
    fn validate_positionals(&self) -> Result<(), String> {
        let positionals = self.args.get_positionals();
        let matches = self.matches.get_matches();

        if positionals.len() == 1 && !positionals[0].is_required {
            return Ok(())
        }

        'outer: for arg in positionals {
            for matched in matches {
                if arg.name == matched.name {
                    continue 'outer;
                }
            }
            return Err(format!("Missing required argument: {}. Try --help for usage instructions", arg.name.to_uppercase()));
        }

        return Ok(())
    }

    fn display_help(&self) {
        let options_vec = self.args.get_options();
        let positionals_vec = self.args.get_positionals();

        let mut usage_string = vec![self.name.to_string()];

        let has_options = !options_vec.is_empty();
        if has_options {
            usage_string.push("[OPTIONS]".to_string())
        }

        if !positionals_vec.is_empty() {
            for arg in positionals_vec {
                usage_string.push(arg.name.to_uppercase())
            }
        }

        println!("{}\n", self.about);
        println!("USAGE:");
        println!("  {}\n", usage_string.join(" "));

        if has_options {
            println!("OPTIONS:");

            let opt_table: Vec<Vec<String>> = options_vec.iter()
                .map(|o| {
                    if o.short.is_empty() {
                        vec![" ".to_string(), o.short.to_string(), format!("--{}", o.long), o.help.to_string()]
                    } else {
                        vec![" ".to_string(), format!("-{}", o.short), format!("--{}", o.long), o.help.to_string()]
                    }
                })
                .collect();

            println!("{}", opt_table.to_table());
        }
    }

    fn display_version(&self) {
        println!("{} {}", self.name, self.version)
    }
}
