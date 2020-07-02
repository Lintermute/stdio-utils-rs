// Copyright 2020 Andreas Waidler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub use anyhow::{
    Context,
    Result,
};
pub use criterion::Criterion;

use anyhow::ensure;
use std::{
    fs,
    fs::{
        File,
        OpenOptions,
    },
    process::{
        Command,
        Stdio,
    },
};

/// Wraps an expression such as `println!("Hello, world!")` into
/// closures necessary for running it as criterion benchmark.
/// Furthermore, this macro calls `unwrap()` on the result
/// in order to abort the benchmark on failures, thereby
/// avoiding silent failures and corrupted benchmark statistics.
#[macro_export]
macro_rules! wrap {
    ($routine:expr $(,$expected_result:expr)?) => {
        |bencher| bencher.iter(|| {
            #[allow(unused_variables)]
            let actual_result = ($routine)
                .expect("Failed to run benchmark function");
            $(
                if (actual_result != $expected_result) {
                    panic!(
                        "Got unexpected result {} (expected {})",
                        actual_result, $expected_result
                    );
                }
            )?
        });
    }
}

pub fn create_test_data_file(filename: &str, lines: u32) -> Result<()>
{
    let test_data_file = fcreate(filename)?;

    let n = lines.to_string();
    let cmd = "shuf";
    let status = Command::new(cmd)
        .args(&["-i", "0-999", "-n", n.as_ref(), "-r"])
        .stdout(test_data_file)
        .status()
        .with_context(|| format!("Failed to start program `{}`", cmd))?
        .code()
        .with_context(|| format!("Failed to read exit status of `{}`", cmd))?;

    ensure!(
        status == 0,
        "Program \"{}\" returned error code {}",
        cmd,
        status
    );

    Ok(())
}

pub fn delete_test_data_file(filename: &str) -> Result<()>
{
    fs::remove_file(filename)
        .with_context(|| format!("Failed to delete test file \"{}\"", filename))
}

fn fcreate(filename: &str) -> Result<File>
{
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .read(false)
        .open(filename)
        .with_context(|| format!("Failed to create file \"{}\"", filename))
}

pub fn fopen(filename: &str) -> Result<File>
{
    File::open(filename)
        .with_context(|| format!("Failed to open file \"{}\"", filename))
}

pub fn run_ours(test_data_filename: &str) -> Result<isize>
{
    let binary = env!("CARGO_BIN_EXE_stdio-utils");
    let input = fopen(test_data_filename)?;

    let no_args: &[&str] = &[];
    run(binary, no_args, input)
}

pub fn run<A, S, I>(cmd: &str, args: A, input: I) -> Result<isize>
where
    A: IntoIterator<Item = S>,
    S: std::convert::AsRef<std::ffi::OsStr>,
    I: Into<Stdio>,
{
    let output = Command::new(cmd)
        .args(args)
        .stdin(input)
        .output()
        .with_context(|| format!("Failed to run benchmark program {}", cmd))?;

    let status = output.status;
    ensure!(
        status.success(),
        "Failure: {} returned {:?}",
        cmd,
        status.code()
    );

    Ok(String::from_utf8(output.stdout)?.trim().parse()?)
}
