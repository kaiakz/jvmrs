extern crate clap;

use clap::{Arg, App};

fn cmd() {
    let matches = App::new("RSJVM")
        .version("0.0.1")
        .author("Kai")
        .about("Rust-JVM")
        .arg(Arg::with_name("cp")
            .long("cp")
            .value_name("CLASSPATH")
            .help("class search path of directories and zip/jar files")     
            .takes_value(true)) 
        .arg(Arg::with_name("classpath")
            .long("classpath")
            .value_name("CLASSPATH")
            .help("class search path of directories and zip/jar files
                    A : separated list of directories, JAR archives, and ZIP archives to search for class files.")
            .takes_value(true))
        .arg(Arg::with_name("xjre")
            .long("Xjre")
            .value_name("JREPATH")
            .takes_value(true))
        .arg(Arg::with_name("Property").help("set a system property"))
        // .arg(Arg::with_name("verbose")
        //     .short("v")
        //     .multiple(true)
        //     .help("verbosity level"))
        .get_matches();

    if let Some(f) = matches.value_of("classpath") {
        println!("path : {}", f);
    }

    if let Some(f) = matches.value_of("cp") {
        println!("path : {}", f);
    }

    if let Some(f) = matches.value_of("xjre") {
        println!("path : {}", f);
    }

}