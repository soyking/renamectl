use regex::Regex;
use std::vec;

pub fn extract_keys(s: &str, pattern: &str) -> Vec<u32> {
    let re = Regex::new(pattern).unwrap();
    for cap in re.captures_iter(s) {
        let mut ret = Vec::<u32>::new();
        for c in cap.iter() {
            match c.and_then(|x| x.as_str().parse::<u32>().ok()) {
                Some(x) => ret.push(x),
                None => {}
            }
        }

        return ret;
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_extract_keys() {
        struct Testcase<'a> {
            s: &'a str,
            pattern: &'a str,
            expected_keys: Vec<u32>,
        }

        for ts in vec![
            Testcase {
                s: "",
                pattern: "",
                expected_keys: vec![],
            },
            Testcase {
                s: "2021-02-14",
                pattern: r"(\d{4})-(\d{2})-(\d{2})",
                expected_keys: vec![2021, 02, 14],
            },
            Testcase {
                s: "PBS.The.Brain.with.David.Eagleman.S01E01.What.is.Reality.720p.x264.HEVCguy.eng.srt",
                pattern: r"S(\d{2})E(\d{2})",
                expected_keys: vec![1, 1],
            },
        ] {
            let keys = super::extract_keys(ts.s, ts.pattern);
            assert_eq!(ts.expected_keys, keys);
        }
    }
}
