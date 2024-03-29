extern crate rustc_serialize;
extern crate docopt;
extern crate git2;

use git2::Repository;
use docopt::Docopt;

mod parse;

const USAGE: &'static str = "
modmod

  updates a single module across all Puppetfiles in a control-repo.
  It is intended to be run in the root dir of a control-repo.


Usage:
  modmod <username-modname> <version>
  modmod ( -h | --help )

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_username_modname: String,
    arg_version: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    parse::update_puppetfile_mods("cbrock", "1.2.3");

    let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
    };

    let result = repo.branches(Some(git2::BranchType::Local));

    let branches = match result {
            Ok(branches) => branches,
            Err(e) => panic!("failed to find branches: {}", e),
    };

    for (i, result2) in branches.enumerate() {
        println!("number {}", i);
        let branch_tuple = match result2 {
            Ok(branch) => branch,
            Err(e) => panic!("failed to find branch: {}", e),
        };

        let branch_option = match branch_tuple.0.name() {
            Ok(option) => option,
            Err(e) => panic!("failed to find branch name: {}", e),
        };

        let branch_name = match branch_option {
            Some(x) => x,
            None    => "No branch name was found.",
        };

        println!("branch {}", branch_name);

        let cb = & mut git2::build::CheckoutBuilder::new();
        // checkout_treeish ???

        // Ah, need to revparse the branch name to get an object.
        // Then checkout the git object.
        //let co_result = repo.checkout_tree(&branch_tuple.0, Some(cb));

    }
}
