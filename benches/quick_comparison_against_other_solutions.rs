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

#[macro_use]
mod lib;

use lib::*;
use std::process::{
    Command,
    Stdio,
};

fn main() -> Result<()>
{
    let filename = "test_data.txt";

    let mut c = Criterion::default().sample_size(10).configure_from_args();

    create_test_data_file(filename, 10_000_000)
        .context("Failed to create test inputs")?;

    let exp =
        run_ours(filename).context("Failed to compute expected test result")?;

    c.bench_function(env!("CARGO_PKG_NAME"), wrap!(run_ours(filename), exp));
    c.bench_function("python", wrap!(run_python_variant(filename), exp));
    c.bench_function("awk", wrap!(run_awk_variant(filename), exp));
    c.bench_function("paste|bc", wrap!(run_bc_variant(filename), exp));

    delete_test_data_file(filename)
        .context("Failed to clean up test input file")?;

    Criterion::default().configure_from_args().final_summary();

    Ok(())
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
