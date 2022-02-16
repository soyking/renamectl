use regex::Regex;

pub fn extract_key_for_patterns<'a>(s: &'a str, patterns: Vec<&str>) -> String {
    for pattern in patterns {
        let key = extract_key_for_pattern(s, pattern);
        if key.len() > 0 {
            return key;
        }
    }

    return "".to_string();
}

pub fn extract_key_for_pattern<'b>(s: &'b str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    for cap in re.captures_iter(s) {
        let mut ret = Vec::<&str>::new();
        let mut cap_iter = cap.iter();
        cap_iter.next(); // ignore first one
        for c in cap_iter {
            match c.and_then(|x| Some(x.as_str())) {
                Some(x) => ret.push(x),
                None => {}
            }
        }

        return ret.join("-");
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_extract_keys() {
        struct Testcase<'a> {
            s: &'a str,
            patterns: Vec<&'a str>,
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
                patterns: vec![""],
                expected_key:"",
            },
            Testcase {
                s: "",
                patterns: vec![r"S(\d{2})E(\d{2})", r"(\d{4})-(\d{2})-(\d{2})"],
                expected_key:"",
            },
            Testcase {
                s: "2021-02-14",
                patterns: vec![r"S(\d{2})E(\d{2})", r"(\d{4})-(\d{2})-(\d{2})"],
                expected_key: "2021-02-14",
            },
            Testcase {
                s: "PBS.The.Brain.with.David.Eagleman.S01E01.What.is.Reality.720p.x264.HEVCguy.eng.srt",
                patterns: vec![r"S(\d{2})E(\d{2})", r"(\d{4})-(\d{2})-(\d{2})"],
                expected_key: "01-01",
            },
        ] {
            let keys = super::extract_key_for_patterns(ts.s, ts.patterns);
            assert_eq!(ts.expected_key, keys);
        }
    }
}
