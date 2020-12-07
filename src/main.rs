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
                     .arg(Arg::with_name("depth")
                         .short("d")
                         .takes_value(true)
                         .default_value("-1")
                         )
                     .get_matches();
    // TODO: Better error handling
    let path = matches.value_of("path").unwrap();
    let max_depth = matches.value_of("depth").unwrap().parse::<i32>().unwrap();
    print_dir(path, 2, 1, max_depth);
}
