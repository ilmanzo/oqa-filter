#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use std::io::{self, BufRead, BufReader, Read, Write};
use std::fmt;

/// OpenQA domain URLs
const OPENQA_SUSE_URL: &str = "https://openqa.suse.de/tests/";
const OPENQA_OPENSUSE_URL: &str = "https://openqa.opensuse.org/tests/";

/// Represents `OpenQA` instance domains
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Domain {
    SuseDe,
    OpenSuseOrg,
}

impl Domain {
    /// Returns the base URL for the domain
    const fn base_url(&self) -> &'static str {
        match self {
            Self::SuseDe => OPENQA_SUSE_URL,
            Self::OpenSuseOrg => OPENQA_OPENSUSE_URL,
        }
    }
}

/// Represents an `OpenQA` job with its domain, ID and consecutive count
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct OpenQAJob {
    domain: Domain,
    id: u32,
    consecutive_count: u32,
}

impl OpenQAJob {
    /// Creates a new Job from a URL string
    fn from_url(url: &str) -> Option<Self> {
        let url = url.trim();
        if let Some(id) = url.strip_prefix(OPENQA_SUSE_URL) {
            Some(Self {
                domain: Domain::SuseDe,
                id: id.parse().ok()?,
                consecutive_count: 0,
            })
        } else if let Some(id) = url.strip_prefix(OPENQA_OPENSUSE_URL) {
            Some(Self {
                domain: Domain::OpenSuseOrg,
                id: id.parse().ok()?,
                consecutive_count: 0,
            })
        } else {
            None
        }
    }

    /// Returns true if this test is consecutive with another test
    fn is_consecutive_with(&self, other: &Self) -> bool {
        self.domain == other.domain 
            && self.id + self.consecutive_count + 1 == other.id
    }

    fn all_same_domain(tests: &[Self]) -> bool {
        if tests.is_empty() {
            return false;
        }
        let first_domain = &tests[0].domain;
        tests.iter().all(|test| test.domain == *first_domain)
    }

    fn format_compact_output(tests: &[Self]) -> String {
        if tests.is_empty() {
            return String::new();
        }

        let has_consecutive = tests.iter().any(|t| t.consecutive_count > 0);
        let base_url = if has_consecutive {
            tests[0].domain.base_url()
        } else {
            tests[0].domain.base_url().trim_end_matches("/tests/")
        };

        let ids: Vec<_> = tests.iter()
            .map(|t| {
                if t.consecutive_count > 0 {
                    format!("{}+{}", t.id, t.consecutive_count)
                } else {
                    t.id.to_string()
                }
            })
            .collect();

        if has_consecutive {
            format!("{}{}", base_url, ids.join(" "))
        } else {
            format!("{} {}", base_url, ids.join(","))
        }
    }
}

impl fmt::Display for OpenQAJob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.domain.base_url(),
            self.id,
            if self.consecutive_count > 0 {
                format!("+{}", self.consecutive_count)
            } else {
                String::new()
            }
        )
    }
}

/// Processes input lines and outputs aggregated `OpenQA` test URLs
pub fn process_input<R: Read, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut tests: Vec<OpenQAJob> = BufReader::new(input)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            line.split("->")
                .nth(1)
                .map(str::trim)
                .and_then(OpenQAJob::from_url)
        })
        .collect();

    tests.sort();
    tests.dedup();

    // Aggregate consecutive tests
    let mut i = 0;
    while i < tests.len().saturating_sub(1) {
        if tests[i].is_consecutive_with(&tests[i + 1]) {
            tests[i].consecutive_count += 1;
            tests.remove(i + 1);
        } else {
            i += 1;
        }
    }

    let output_str = if OpenQAJob::all_same_domain(&tests) {
        OpenQAJob::format_compact_output(&tests)
    } else {
        tests.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    };

    writeln!(output, "openqa-mon {output_str}")
}

fn main() -> io::Result<()> {
    process_input(io::stdin(), io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_dedup() {
        let input = Cursor::new("foo -> https://openqa.suse.de/tests/123\nbar -> https://openqa.opensuse.org/tests/456\nbaz -> https://openqa.suse.de/tests/123\n");
        let mut output = Vec::new();
        process_input(input, &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "openqa-mon https://openqa.suse.de/tests/123 https://openqa.opensuse.org/tests/456\n"
        );
    }

    #[test]
    fn test_noisy_input() {
        let input = Cursor::new("Cloning parents of sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            1 job has been created:\n
             - sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418915\n
            Cloning parents of sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            1 job has been created:\n
             - sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418917\n
            Cloning parents of sle-15-SP3-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            Cloning parents of sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit\n
            1 job has been created:\n
             - sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit -> https://openqa.opensuse.org/tests/16418919");
             let mut output = Vec::new();
             process_input(input, &mut output).unwrap();
             let expected = "openqa-mon https://openqa.suse.de/tests/16418915 https://openqa.suse.de/tests/16418917 https://openqa.opensuse.org/tests/16418919\n";
             assert_eq!(String::from_utf8(output).unwrap(), expected);
            
    }

    #[test]
    fn test_consecutive_ids() {
        let input = Cursor::new(
            "foo -> https://openqa.suse.de/tests/123\n\
             bar -> https://openqa.suse.de/tests/124\n\
             baz -> https://openqa.suse.de/tests/125\n"
        );
        let mut output = Vec::new();
        process_input(input, &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "openqa-mon https://openqa.suse.de/tests/123+2\n"
        );
    }

    #[test]
    fn test_compact_output() {
        let input = Cursor::new(
            "test1 -> https://openqa.suse.de/tests/123\n\
             test2 -> https://openqa.suse.de/tests/125\n\
             test3 -> https://openqa.suse.de/tests/127\n"
        );
        let mut output = Vec::new();
        process_input(input, &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "openqa-mon https://openqa.suse.de 123,125,127\n"
        );
    }
}