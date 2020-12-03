use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;
use std::{env, fmt::Display};
use std::{ffi::OsStr, fs};
use tempdir::TempDir;

use anyhow::{Context, Result};

pub static TEST_STAGE_PATH: &str = "punditTestStage";

#[derive(Debug)]
pub struct TestEnv {
    pub dir: TempDir,
    pub executable: PathBuf,
}

pub struct TestOutput {
    pub env: TestEnv,
    pub success: bool,
    pub output: String,
    pub stderr: String,
}

#[allow(dead_code)] // Somehow rust doesnt realize I use these in other modules.
#[derive(Debug, Clone)]
pub enum TestArg<'a> {
    NormalArg(&'a str),
    AbsolutePath(&'a Path),
    RelativePath(&'a str),
}

impl<'a> TestArg<'a> {
    fn convert_to_string(&'a self, dir: &Path) -> Result<String> {
        dbg!(self);
        dbg!(dir);
        match self {
            TestArg::NormalArg(s) => Ok(s.to_string()),
            TestArg::AbsolutePath(p) => Ok(p.to_str().unwrap().to_owned()),
            TestArg::RelativePath(p) => Ok(dir.join(p).to_str().unwrap().to_owned()),
        }
    }
}

fn convert_args<'a>(args: &'a [TestArg<'a>], dir: &'a Path) -> Result<Vec<String>> {
    args.iter().map(|arg| arg.convert_to_string(dir)).collect()
}

pub fn setup_test(executable_name: String, setups_folder: &Path, test_name: &str) -> TestEnv {
    let test_dir = env::current_exe().expect("build exe");
    let build_dir = test_dir
        .parent()
        .expect("deps")
        .parent()
        .expect("build dir");
    let executable = build_dir.join(executable_name);
    let env = TestEnv {
        executable: executable.to_path_buf(),
        dir: TempDir::new(TEST_STAGE_PATH).expect("Setup test directory"),
    };
    let source = setups_folder.join(test_name);
    copy(source, &env.dir).expect("Copying test files");
    env
}

pub fn get_shell_command_output<T: Display + AsRef<OsStr>>(
    command: &str,
    args: &[T],
) -> (bool, String, String) {
    print!("Running {}", command);
    for arg in args.iter() {
        print!(" {}", arg);
    }
    println!("");
    let child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to run command: {}", command));

    let output = child.wait_with_output().expect("Failed to read stdout");
    let exit_code = output.status;
    (
        exit_code.success(),
        str::from_utf8(&output.stdout)
            .expect("Failed to decode stdout as utf8")
            .to_owned(),
        str::from_utf8(&output.stderr)
            .expect("Failed to decode stderr as utf8")
            .to_owned(),
    )
}

pub fn run_pundit(env: &TestEnv, args: &[TestArg]) -> Result<(bool, String, String)> {
    let mut new_args = vec![TestArg::AbsolutePath(env.dir.path())];
    new_args.extend_from_slice(args);
    Ok(get_shell_command_output(
        env.executable.to_str().unwrap(),
        &convert_args(&new_args, &env.dir.path())?,
    ))
}

#[allow(dead_code)] // Not sure why I need this to begin with?
pub fn run_pundit_on_setup_with_args(
    binary_name: String,
    setups_folder: &Path,
    setup_name: &str,
    args: &[TestArg],
) -> Result<TestOutput> {
    let env = setup_test(binary_name, setups_folder, setup_name);
    let output = run_pundit(&env, args)?;
    Ok(TestOutput {
        env,
        success: output.0,
        output: output.1,
        stderr: output.2,
    })
}

// pub fn run_pundit_on_env_with_args(env: TestEnv, args: &[TestArg]) -> TestOutput {
//     let output = run_pundit(&env, args);
//     TestOutput {
//         env,
//         success: output.0,
//         output: output.1,
//         stderr: output.2,
//     }
// }

// Taken from 'Doug' from
// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();
        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)
                .context(format!("Creating directory {}", dest.to_str().unwrap()))?;
        }

        for entry in fs::read_dir(&working_path).context(format!(
            "Reading directory {}",
            &working_path.to_str().unwrap()
        ))? {
            let entry = entry.context(format!(
                "Reading entry in directory {}",
                &working_path.to_str().unwrap()
            ))?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path).context(format!(
                            "Error copying {} to {}",
                            &path.to_str().unwrap(),
                            &dest_path.to_str().unwrap()
                        ))?;
                    }
                    None => {}
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)] // Not sure why I need this to begin with?
pub fn show_output(out: &TestOutput) {
    println!("Pundit stdout:\n{}", &out.output);
    println!("Pundit stderr:\n{}", &out.stderr);
}

#[allow(dead_code)] // Not sure why I need this to begin with?
pub fn get_pundit_executable() -> String {
    if cfg!(windows) {
        "pundit.exe".to_owned()
    } else {
        "pundit".to_owned()
    }
}

#[allow(dead_code)] // Not sure why I need this to begin with?
pub fn get_ankitool_executable() -> String {
    if cfg!(windows) {
        "ankitool.exe".to_owned()
    } else {
        "ankitool".to_owned()
    }
}
