#[macro_use]
extern crate clap;
use clap::App;

extern crate mkvm;
use mkvm::yaml;


fn main() {
    let main = load_yaml!("main.yaml");
    let args =
        App::from_yaml(main)
            .version(crate_version!())
            .author(crate_authors!())
            .get_matches();

    let verbose = args.occurrences_of("verbose");
    let debug   = args.occurrences_of("debug");
    println!("verbose {}", verbose);
    println!("debug   {}", debug);

    match args.subcommand() {
        ("define", Some(define)) => {
            match define.subcommand() {
                ("vm", Some(vm)) => {
                    let files = vm.values_of("INPUT").unwrap().collect::<Vec<_>>();
                    yaml::files(&files);
                },
                ("", None) => {},
                _ => unreachable!(),
            };
        },
        ("", None) => {},
        _ => unreachable!(),
    }
}
