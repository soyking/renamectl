use crate::error::Result;
use anyhow::Context;
use regex::Regex;

pub trait Extractor {
    fn extract(&self, source: &str) -> Result<String>;
}

pub struct RegexExtractor {
    re_list: Vec<Regex>,
}

impl Extractor for RegexExtractor {
    fn extract(&self, source: &str) -> Result<String> {
        for pattern in &self.re_list {
            let key = self.extract_one(source, pattern)?;
            if key.len() > 0 {
                return Ok(key);
            }
        }

        return Ok("".to_string());
    }
}

impl RegexExtractor {
    pub fn new(patterns: Vec<String>) -> Result<'static, RegexExtractor> {
        let mut re_list = vec![];

        for pattern in patterns {
            let re = Regex::new(&pattern).with_context(|| format!("new regex {}", pattern))?;
            re_list.push(re);
        }

        return Ok(RegexExtractor { re_list });
    }

    fn extract_one(&self, source: &str, re: &Regex) -> Result<String> {
        for cap in re.captures_iter(source) {
            let mut cap_iter = cap.iter();
            cap_iter.next(); // skip first one

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
    use crate::key::{Extractor, RegexExtractor};
    use std::vec;

    #[test]
    fn test_bad_regex_extractor() {
        let e = RegexExtractor::new(vec![r"\d{".to_string()]);
        assert!(e.is_err());
        println!("return err: {:?}", e.err().unwrap());
    }

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
                expected_key: "",
            },
            Testcase {
                s: "",
                patterns: vec![r"S(\d{2})E(\d{2})".to_string(), r"(\d{4})-(\d{2})-(\d{2})".to_string()],
                expected_key: "",
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
            assert!(e.is_ok());
            let e = e.unwrap();
            match e.extract(ts.s) {
                Ok(k) => assert_eq!(ts.expected_key, &k),
                Err(e) => println!("{}", e), // TODO: expected error
            }
        }
    }
}
