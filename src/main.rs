mod opts;
mod search;

use clap::{Arg, App};
use colored::*;
use crate::search::search::search_directory;
use std::path::Path;

fn main() {

    let matches = App::new("gfr")
        .version("1.0")
        .about("grep fancy recursive")
        .arg(Arg::new("search")
            .short('s')
            .long("search")
            .value_name("SEARCH_TERM")
            .about("regex term to use for search in files")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .value_name("TARGET_FILE")
            .about("regex term to match file names to search in")
            .takes_value(true)
            .required(false))
        .arg(Arg::new("ignore")
            .short('i')
            .long("ignore")
            .value_name("IGNORE_FILE")
            .about("regex term to match files to not search in")
            .takes_value(true)
            .required(false))
        .arg(Arg::new("verbose")
            .short('v')
            .multiple(true)
            .about("sets the level of verbosity")
            .required(false))
        .arg(Arg::new("context")
            .value_name("CONTEXT")
            .short('c')
            .long("context")
            .about("number of lines to print before and after a search term is matched")
            .required(false))
        .arg(Arg::new("maxline")
            .value_name("MAXLINES")
            .short('m')
            .long("maxlines")
            .about("maximum number of lines to parse in file search")
            .required(false))
        .arg(Arg::new("sourcedir")
            .short('d')
            .long("sourcedir")
            .about("directory to begin search at")
            .default_value(".")
            .required(false))
        .get_matches();


    let mut input = opts::opts::Opts::new();

    if let Some(search) = matches.value_of("search") {
        let t = search.to_string();
        input.set_search(&t);
    }

    if let Some(target) = matches.value_of("target") {
        let t = target.to_string();
        input.set_target(&t);
    }

    if let Some(ignore) = matches.value_of("ignore") {
        let t = ignore.to_string();
        input.set_ignore(&t);
    }

    if let Some(context) = matches.value_of("context") {
        let p = context.parse::<usize>().unwrap();
        println!("This is the val of {}", p);
        input.set_context(&p);
    }

    if let Some(maxlines) = matches.value_of("maxlines") {
        let p = maxlines.parse::<usize>().unwrap();
        input.set_maxlines(&p);
    }

    if let Some(sdir) = matches.value_of("sourcedir") {
        input.set_source_dir(&(sdir).to_string());
    }

    println!("this is the input {:?}", input);

    let starting_dir = Path::new(&input.sdir);

    search_directory(&input, starting_dir);

    return;
}
