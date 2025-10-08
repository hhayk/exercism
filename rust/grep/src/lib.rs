use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    line_number: bool,
    matching_lines: bool,
    case_insentive: bool,
    invert: bool,
    entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        //        todo!(
        //            "Given the flags {flags:?} implement your own 'Flags' struct to handle flags-related logic"
        //        );
        Flags {
            line_number: flags.contains(&"-n"),
            matching_lines: flags.contains(&"-l"),
            case_insentive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    //  todo!(
    //      "Search the files '{files:?}' for '{pattern}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{flags:?}'"
    //  );
    let mut ret = Vec::new();

    for &path in files {
        let data = fs::read_to_string(path)?;

        for (num, line) in data.lines().enumerate() {
            let (line_to_compare, search_pattern) = if flags.case_insentive {
                (line.to_lowercase(), pattern.to_lowercase())
            } else {
                (line.to_string(), pattern.to_string())
            };

            let condition = {
                let matches = if flags.entire_line {
                    line_to_compare == search_pattern
                } else {
                    line_to_compare.contains(&search_pattern)
                };

                if flags.invert {
                    !matches
                } else {
                    matches
                }
            };

            if condition {
                if flags.matching_lines {
                    ret.push(path.to_string());
                    break;
                }

                let mut ss = String::new();
                if files.len() > 1 {
                    ss.push_str(&format!("{}:", path));
                }
                if flags.line_number {
                    ss.push_str(&format!("{}:{}", num + 1, line));
                } else {
                    ss.push_str(line);
                }
                ret.push(ss);
            }
        }
    }

    Ok(ret)
}
