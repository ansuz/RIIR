use std::error::Error;
use std::fs::File;
use std::fs::remove_file;
use std::io::Write;

static README_FILE_NAME: &'static str = "README.md";

fn clean_up() {
    match remove_file(README_FILE_NAME) {
        Result::Ok(_) => println!("Removed README.md."),
        Result::Err(_) => println!("No README.md found. Continuing...")
    }
}

fn build_intro() -> String {
    "# RIIR
why not Rewrite It In Rust (**RIIR**)

Are you an author or contributor to a software project?

Have you ever been asked to rewrite, or consider rewriting that project in [Rust](https://www.rust-lang.org/)?

If so, you may have been a victim of the RIIR agenda that is sweeping the web.

If this has happened to you, please [report it](https://github.com/ansuz/RIIR/issues/) so that something can be done.
".to_owned()
}

fn build_faq() -> String {
    "## FAQ".to_owned()
}

fn build_ru_srs() -> String {
    "### R U SRS?

No. This is a joke.".to_owned()
}

fn build_y_u_h8() -> String {
    "### Y U HATE RUST SO MUCH?

I don't, actually. I believe that those who spend their time asking people to rewrite their projects are probably not themselves active Rust developers, as those active devs are probably busy [writing memory-safe code](https://trac.torproject.org/projects/tor/ticket/11331).".to_owned()
}

fn build_ru_offending_me() -> String {
    "### R U OFFENDING ME?
![](rust.png)".to_owned()
}

fn build_readme_contents() -> String {
    let list = vec![build_intro(), build_faq(), build_ru_srs(), build_y_u_h8(), build_ru_offending_me()];
    list.join("\n\n") + "\n"
}

fn build() {
    let path = std::path::Path::new(README_FILE_NAME);
    let mut readme = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
        Ok(file) => file
    };

    match readme.write_all(build_readme_contents().as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}", path.display(), why.description())
        },
        Ok(_) => println!("Wrote to {}", path.display())
    }
}

fn main() {
    clean_up();
    build();
}
