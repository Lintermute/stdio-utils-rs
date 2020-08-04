stdio-utils-rs
--------------

A minimal Rust program that sums integers read from stdin:

	> printf "1\n4\n-1\n3\n" | stdio-utils
	7

As a practical use-case, we can make [ripgrep][rg] report
the total number of matches across several files.
For example, to count occurrences of the pattern `the`
in the files `README.md` and `LICENSE` in this repository:

	> rg -i the README.md LICENSE --count-matches
	LICENSE:18
	README.md:3
	> rg -i the README.md LICENSE --count-matches | cut -d: -f2 | stdio-utils
	21

Run `cargo bench --bench quick_comparison_against_other_solutions`
to compare stdio-utils-rs against other common solutions
as discussed on the stackoverflow question
[“Shell command to sum integers, one per line”][stackoverflow].

This project was originally created as a personal project
to get started with Rust development.

[rg]: https://github.com/BurntSushi/ripgrep
[stackoverflow]: https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line
