use crate::anki::named::get_by_name;
use crate::anki::named::Named;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

pub fn run_fzf(content: &str, args: &[&str]) -> String {
    let mut child = Command::new("fzf")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn fzf");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    str::from_utf8(&output.stdout)
        .expect("Failed to decode fzf output as utf8")
        .trim_end_matches("\n")
        .to_owned()
}

pub fn select_interactively<T: Named + Sized>(objects: &[T]) -> Option<&T> {
    let names_vec: Vec<&str> = objects.iter().map(|o| o.get_name()).collect();
    let names_joined = names_vec.join("\n");
    let selection = run_fzf(&names_joined, &[]);
    get_by_name(objects, &selection)
}
