use rawrgs::{App, Arg};

fn main() {
    let app = App::new("Test")
        .author("Harrison Grieve")
        .version("0.1")
        .about("A way to test a CLI app")
        .arg(
            Arg::with_name("gender")
                .long("gender")
                .help("Specify your gender")
        )
        .arg(
            Arg::with_name("human")
                .short("H")
                .help("Specify if you are human or not")
        );

    let matches = app.get_matches();
    println!("{:?}", matches.is_present("gender"));
    println!("{:?}", matches.is_present("human"))
}