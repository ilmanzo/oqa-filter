use std::fmt;

/// Represents `OpenQA` instance domains
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Domain(&'static str);

impl Domain {
    pub const SUSE_DE: Self = Self("https://openqa.suse.de/tests/");
    pub const OPENSUSE_ORG: Self = Self("https://openqa.opensuse.org/tests/");

    const fn base_url(&self) -> &'static str {
        self.0
    }
}

/// Represents an `OpenQA` job with its domain, ID and consecutive count
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OpenQAJob {
    pub domain: Domain,
    pub id: u32,
    pub consecutive_count: u32,
}

impl OpenQAJob {
    /// Creates a new Job from a URL string
    pub fn from_url(url: &str) -> Option<Self> {
        let url = url.trim();
        if let Some(id) = url.strip_prefix(Domain::SUSE_DE.base_url()) {
            Some(Self::new(Domain::SUSE_DE, id))
        } else if let Some(id) = url.strip_prefix(Domain::OPENSUSE_ORG.base_url()) {
            Some(Self::new(Domain::OPENSUSE_ORG, id))
        } else {
            None
        }
    }

    fn new(domain: Domain, id: &str) -> Self {
        Self {
            domain,
            id: id.parse().unwrap_or_default(),
            consecutive_count: 0,
        }
    }

    /// Returns true if this test is consecutive with another test
    pub fn is_consecutive_with(&self, other: &Self) -> bool {
        self.domain == other.domain 
            && self.id + self.consecutive_count + 1 == other.id
    }

    pub fn all_same_domain(tests: &[Self]) -> bool {
        if tests.is_empty() {
            return false;
        }
        let first_domain = &tests[0].domain;
        tests.iter().all(|test| test.domain == *first_domain)
    }

    pub fn format_compact_output(jobs: &[Self]) -> String {
        if jobs.is_empty() {
            return String::new();
        }

        let has_consecutive = jobs.iter().any(|t| t.consecutive_count > 0);
        let base_url = if has_consecutive {
            jobs[0].domain.base_url()
        } else {
            jobs[0].domain.base_url().trim_end_matches("/tests/")
        };

        let ids: Vec<_> = jobs.iter()
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
