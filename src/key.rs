use crate::error::Error;
use crate::error::Result;
use regex::Regex;
pub trait Extractor {
    fn extract(&self, source: &str) -> Result<String>;
}

pub struct RegexExtractor {
    patterns: Vec<String>,
}

impl Extractor for RegexExtractor {
    fn extract(&self, source: &str) -> Result<String> {
        for pattern in &self.patterns {
            let key = self.extract_one(source, &pattern)?;
            if key.len() > 0 {
                return Ok(key);
            }
        }

        return Ok("".to_string());
    }
}

impl RegexExtractor {
    pub fn new(patterns: Vec<String>) -> RegexExtractor {
        return RegexExtractor { patterns };
    }

    fn extract_one(&self, source: &str, pattern: &str) -> Result<String> {
        let re = match Regex::new(pattern) {
            Ok(re) => re,
            Err(e) => {
                return Err(Error::new(e.to_string(), "new regex".to_string()));
            }
        };

        for cap in re.captures_iter(source) {
            let mut cap_iter = cap.iter();
            cap_iter.next(); // ignore first one

            let mut ret = Vec::<&str>::new();
            for c in cap_iter {
                match c.and_then(|x| Some(x.as_str())) {
                    Some(x) => ret.push(x),
                    None => (),
                }
            }

            return Ok(ret.join("-"));
        }

        return Ok("".to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::key::{Extractor, RegexExtractor};

    #[test]
    fn test_regex_extractor() {
        struct Testcase<'a> {
            s: &'a str,
            patterns: Vec<String>,
            expected_key: &'a str,
        }

        for ts in vec![
            Testcase {
                s: "",
                patterns: vec![],
                expected_key:"",
            },
            Testcase {
                s: "",
                patterns: vec![r"\d{".to_string()],
                expected_key:"",
            },
            Testcase {
                s: "",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key:"",
            },
            Testcase {
                s: "2021-02-14",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: "2021-02-14",
            },
            Testcase {
                s: "PBS.The.Brain.with.David.Eagleman.S01E01.What.is.Reality.720p.x264.HEVCguy.eng.srt",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: "01-01",
            },
        ] {
        let e = RegexExtractor::new(ts.patterns);
            match e.extract(ts.s) {
                Ok(k) => assert_eq!(ts.expected_key, &k),
                Err(e) => println!("{}", e), // TODO: expected error
            }
        }
    }
}
