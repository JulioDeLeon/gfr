pub(crate) mod opts {
    use regex::Regex;

    #[derive(Debug)]
    pub struct Opts {
        pub(crate) search: Option<Regex>,
        pub(crate) target: Option<Regex>,
        pub(crate) ignore: Option<Regex>,
        pub(crate) sdir: String,
        pub(crate) context: usize,
        pub(crate) maxlines: usize,
        pub(crate) verbose: bool,
    }

    impl Opts {
        pub fn new() -> Opts {
            Opts {
                search: None,
                target: None,
                ignore: None,
                sdir: ".".to_string(),
                context: 0,
                maxlines: 0,
                verbose: false,
            }
        }

        pub fn set_search(&mut self, search: &String) {
            let r = Regex::new(search)
                .expect(&format!("Invalid regex passed for search: {}", search));

            self.search = Some(r);
        }

        pub fn set_target(&mut self, target: &String) {
            let r = Regex::new(target)
                .expect(&format!("Invalid regex passed for target: {}", target));

            self.target = Some(r);
        }

        pub fn set_ignore(&mut self, ignore: &String) {
            let r = Regex::new(ignore)
                .expect(&format!("Invalid regex passed for ignore: {}", ignore));

            self.ignore = Some(r);
        }

        pub fn set_source_dir(&mut self, sdir: &String) {
            self.sdir = sdir.clone();
        }

        pub fn set_context(&mut self, context: &usize) {
            self.context = context.clone()
        }

        pub fn set_maxlines(&mut self, maxlines: &usize) {
            self.maxlines = maxlines.clone();
        }

        pub fn set_verbose(&mut self, v: &bool) {
            self.verbose = v.clone();
        }
    }
}