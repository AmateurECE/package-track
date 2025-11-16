use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    parts: Vec<i32>,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parts
                .iter()
                .map(ToString::to_string)
                .intersperse(".".to_string())
                .collect::<String>()
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub struct VersionParseError(String);
impl std::fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse version from {}", self.0)
    }
}

static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+\.)*(\d+)$").unwrap());

impl std::str::FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !VERSION_REGEX.is_match(s) {
            return Err(VersionParseError(s.to_string()));
        }
        // INVARIANT: The parse will never fail for a match of the regex above
        let parts = s.split(".").map(|c| c.parse().unwrap()).collect();
        Ok(Version { parts })
    }
}

#[cfg(test)]
mod test {
    use super::Version;

    #[test]
    fn one_digit_version() {
        assert_eq!("1".parse::<Version>().unwrap(), Version { parts: vec![1] })
    }

    #[test]
    fn two_digit_version() {
        assert_eq!(
            "1.1".parse::<Version>().unwrap(),
            Version { parts: vec![1, 1] }
        )
    }

    #[test]
    fn three_digit_version() {
        assert_eq!(
            "1.2.0".parse::<Version>().unwrap(),
            Version {
                parts: vec![1, 2, 0]
            }
        )
    }

    #[test]
    fn four_digit_version() {
        assert_eq!(
            "1.0.2.1".parse::<Version>().unwrap(),
            Version {
                parts: vec![1, 0, 2, 1]
            }
        )
    }

    #[test]
    fn longer_version_compare() {
        assert!("1".parse::<Version>().unwrap() < "1.0".parse().unwrap());
    }

    #[test]
    fn same_length_version_compare() {
        assert!("1.0".parse::<Version>().unwrap() < "1.1".parse().unwrap());
    }

    #[test]
    fn version_equality() {
        assert!("1.0".parse::<Version>().unwrap() == "1.0".parse().unwrap());
    }

    #[test]
    fn major_version_compare() {
        assert!("1.0".parse::<Version>().unwrap() < "2.0".parse().unwrap());
    }
}
