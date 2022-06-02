use rawrgs::{App, Arg};

fn main() {
    let app = App::new("Test")
        .author("Harrison Grieve")
        .version("0.1")
        .about("A way to test a CLI app")
        .arg(
            Arg::with_name("test")
                .long("test")
                .takes_value(true)
        );

    let matches = app.get_matches();
    println!("{:?}", matches);
    println!("{:?}", matches.value_of("test"));
}