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

use anyhow::{
    ensure,
    Context,
    Result,
};
use criterion::Criterion;
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
macro_rules! wrap {
    ($routine:expr, $expected_result:expr) => {
        |bencher| {
            bencher.iter(|| {
                let actual_result = ($routine).unwrap();
                if (actual_result != $expected_result) {
                    panic!(
                        "Got unexpected result {} (expected {})",
                        actual_result, $expected_result
                    );
                }
            })
        }
    };
}

fn main() -> Result<()>
{
    let filename = "test_data.txt";

    let mut c = Criterion::default().sample_size(10).configure_from_args();

    create_test_data_file(filename).context("Failed to create test inputs")?;

    let expected_result = run_stdio_utils(filename)
        .context("Failed to compute expected test result")?;

    c.bench_function(
        env!("CARGO_PKG_NAME"),
        wrap!(run_stdio_utils(filename), expected_result),
    );
    c.bench_function(
        "python",
        wrap!(run_python_variant(filename), expected_result),
    );
    c.bench_function("awk", wrap!(run_awk_variant(filename), expected_result));
    c.bench_function(
        "paste|bc",
        wrap!(run_bc_variant(filename), expected_result),
    );

    delete_test_data_file(filename)?;

    Criterion::default().configure_from_args().final_summary();

    Ok(())
}

fn create_test_data_file(filename: &str) -> Result<()>
{
    let test_data_file = fcreate(filename)?;

    let cmd = "shuf";
    let status = Command::new(cmd)
        .args(&["-i", "0-99", "-n", "10000000", "-r"])
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

fn delete_test_data_file(filename: &str) -> Result<()>
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

fn fopen(filename: &str) -> Result<File>
{
    File::open(filename)
        .with_context(|| format!("Failed to open file \"{}\"", filename))
}

fn run_stdio_utils(test_data_filename: &str) -> Result<isize>
{
    let binary = env!("CARGO_BIN_EXE_stdio-utils");
    let input = fopen(test_data_filename)?;

    let no_args: &[&str] = &[];
    run(binary, no_args, input)
}

fn run_python_variant(test_data_filename: &str) -> Result<isize>
{
    let input = fopen(test_data_filename)?;
    let code = "import sys; print(sum(map(int, sys.stdin)))";

    run("python", &["-c", code], input)
}

fn run_awk_variant(test_data_filename: &str) -> Result<isize>
{
    let input = fopen(test_data_filename)?;
    let code = "{s+=$1} END {printf \"%.0f\", s}";

    run("awk", &[code], input)
}

fn run_bc_variant(test_data_filename: &str) -> Result<isize>
{
    let input = fopen(test_data_filename)?;

    let paste = Command::new("paste")
        .args(&["-s", "-d+", "-"])
        .stdin(input)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to invoke program `paste`")?
        .stdout
        .with_context(|| "Failed to get stdout of `paste` process")?;

    let no_args: &[&str] = &[];
    run("bc", no_args, paste)
}

fn run<A, S, I>(cmd: &str, args: A, input: I) -> Result<isize>
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
