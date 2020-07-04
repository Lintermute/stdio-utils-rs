stdio-utils-rs
--------------

A minimal Rust program that sums integers read from stdin:

	• printf "1\n4\n-1\n3\n" | stdio-utils
	7

Run `cargo bench --bench quick_comparison_against_other_solutions`
to compare stdio-utils-rs against other common solutions
as discussed on the stackoverflow question
[“Shell command to sum integers, one per line”][stackoverflow].

This project was originally created as a personal project
to get started with Rust development.

[stackoverflow]: https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line
