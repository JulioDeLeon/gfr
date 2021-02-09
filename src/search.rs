pub(crate) mod search {
    use crate::opts::opts::Opts;
    use std::path::Path;
    use std::{io, fs};
    use std::io::BufRead;
    use std::fs::File;
    use colored::Colorize;
    use regex::Regex;

    pub fn search_directory(opts: &Opts, dir: &Path) -> io::Result<()> {
        // open the dir
        if !dir.is_dir() {
            return Ok(())
        }
        // get entries
        println!("This is a dir {}", dir.to_str().expect("could not get name").blue());

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let cpath = entry.path();

            if let Some(target) = &opts.target {
                if !target.is_match(cpath.to_str().expect("could not get name")) {
                    continue;
                }
            }

            if let Some(ignore) = &opts.ignore {
                if ignore.is_match(cpath.to_str().expect("could not get name")) {
                    continue;
                }
            }

            if cpath.is_dir() {
                search_directory(opts, &cpath);
            } else {
                 search_file(opts, &cpath);
            }
        }
        //return
        Ok(())
    }

    pub fn search_file(opts: &Opts, file: &Path) {

        let search = match &opts.search {
            None => return,
            Some(s) => s
        };
        // open file
        let fh = File::open(file)
            .expect("something went wrong opening {}");

        let mut ln = 0;
        let mut before: Vec<String> = Vec::new();
        let mut found: bool = false;
        let mut after_c = 0;
        let mut first: bool = false;
        let p = fs::canonicalize(file).expect("could not get absolute path of file");

        for rline in io::BufReader::new(&fh).lines() {
            match rline {
                Ok(line) => {
                    if search.is_match(&line) {
                        if !found && first {
                            println!();
                        }
                        if !first {
                            println!("{}", p.to_str().expect("could not convert to string").yellow());
                            first = true;
                        }
                        found = true;
                        after_c = opts.context;


                        for x in 0..(before.len()) {
                            let i = ln - (opts.context - x);
                            match before.get(0) {
                                Some(t) => {
                                    print_context(i, t);
                                    before.remove(0);
                                },
                                None => {
                                    println!("Issue with some map");
                                    return
                                }
                            }


                        }
                        print_match(ln, &line, search);

                    } else {
                        if !found {
                            before.push(line);
                            if before.len() > opts.context {
                                before.remove(0);
                            }
                        } else {
                            if found && after_c > 0 {
                                print_context(ln, &line);
                                after_c = after_c - 1;
                            } else {
                                found = false;
                            }
                        }
                    }

                    ln = ln + 1;
                },
                Err(msg) => {
                    println!("There was an issue: {}", msg.to_string());
                    return
                }
            }
        }
    }

    fn print_context(ln: usize, line: &String) {
        println!("[{}]\t{}", ln, line);
    }

    fn print_match(ln: usize, line: &String, re: &Regex) {
        println!("[{}]\t{}", ln, line.red());
    }
}