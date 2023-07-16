use anyhow::Context;
use regex::Regex;

use crate::error::Result;

pub trait Extractor {
    fn extract(&self, source: &str) -> Option<String>;
}

pub struct RegexExtractor {
    re_list: Vec<Regex>,
}

impl Extractor for RegexExtractor {
    fn extract(&self, source: &str) -> Option<String> {
        for pattern in &self.re_list {
            if let Some(key) = self.extract_one(source, pattern) {
                return Some(key);
            }
        }

        return None;
    }
}

impl RegexExtractor {
    pub fn new(patterns: &Vec<String>) -> Result<'static, RegexExtractor> {
        let mut re_list = vec![];

        for pattern in patterns {
            let re = Regex::new(&pattern).with_context(|| format!("new regex {}", pattern))?;
            re_list.push(re);
        }

        return Ok(RegexExtractor { re_list });
    }

    fn extract_one(&self, source: &str, re: &Regex) -> Option<String> {
        for cap in re.captures_iter(source) {
            let mut cap_iter = cap.iter();
            cap_iter.next(); // skip original name

            let mut ret = Vec::<&str>::new();
            for c in cap_iter {
                match c.and_then(|x| Some(x.as_str())) {
                    Some(x) => ret.push(x),
                    None => (),
                }
            }

            if ret.len() > 0 {
                return Some(ret.join("-"));
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::key::{Extractor, RegexExtractor};

    #[test]
    fn test_bad_regex_extractor() {
        let e = RegexExtractor::new(&vec![r"\d{".to_string()]);
        assert!(e.is_err());
        println!("return err: {:?}", e.err().unwrap());
    }

    #[test]
    fn test_regex_extractor() {
        struct Testcase<'a> {
            s: &'a str,
            patterns: Vec<String>,
            expected_key: Option<String>,
        }

        for ts in vec![
            Testcase {
                s: "",
                patterns: vec![],
                expected_key: None,
            },
            Testcase {
                s: "",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: None,
            },
            Testcase {
                s: "2021-02-14",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: Some("2021-02-14".to_string()),
            },
            Testcase {
                s: "PBS.The.Brain.with.David.Eagleman.S01E01.What.is.Reality.720p.x264.HEVCguy.eng.srt",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: Some("01-01".to_string()),
            },
            Testcase {
                s: "九号秘事S4E02.1080p.orange字幕组.简体&英文.srt",
                patterns: vec![
                    r"(?i)S0(\d{1})(?i)E(\d{2})".to_string(),
                    r"(?i)S(\d{1})(?i)E(\d{2})".to_string(),
                    r"(\d{1})(?i)x(\d{2})".to_string(),
                    r"(\d{1})(\d{2})".to_string(),
                ],
                expected_key: Some("4-02".to_string()),
            },
        ] {
            let e = RegexExtractor::new(&ts.patterns).unwrap();
            assert_eq!(ts.expected_key, e.extract(ts.s));
        }
    }
}
