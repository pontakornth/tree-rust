use tree_rust::{print_dir};
use clap::{Arg, App};


fn main() {
    let matches = App::new("TreeRust")
                     .version("0.1")
                     .author("Pontakorn Paesaeng <pontakorn61@gmail.com>")
                     .arg(Arg::with_name("path")
                         .short("p")
                         .index(1)
                         .default_value("./")
                     )
                     .get_matches();
    let path = matches.value_of("path").unwrap();
    print_dir(path, 2);
}
