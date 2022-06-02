# rawrgs

A CLI argument parser with a builder API heavily inspired by clap

Made for my own usage

## example

### main.rs

```rust
use rawrgs::{App, Arg};

fn main() {
    let app = App::new("Test")
        .author("Harrison Grieve")
        .version("0.1")
        .about("A way to test a CLI app")
        .arg(
            Arg::with_name("gender")
                .short("g")
                .long("gender")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("name")
        )
        .arg(
            Arg::with_name("age")
        );

    let matches = app.get_matches();

    println!("{:?}", matches.value_of("name"));
    println!("{:?}", matches.value_of("age"));
    println!("{:?}", matches.value_of("gender"));
}
```

### output

```bash
$ ./example --gender=Male Roger 33
Some("John")
Some("33")
Some("Male")
```