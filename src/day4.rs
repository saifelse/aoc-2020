use regex::Regex;
use lazy_static::lazy_static;

const TOTAL_FIELDS: i32 = 7;  // byr, ~cid~, ecl, eyr, hcl, hgt, iyr, pid

#[aoc(day4, part1, re_str)]
pub fn solve_part4(input: &str) -> i32 {
    lazy_static! {
        // Process each field:value token sucessively.
        // Two newlines in a row signals the end of a passport.
        static ref PASSPORT_RE: Regex = Regex::new(r"(.{3}):\S+(\n\n)?").unwrap();
    }
    let mut valid: i32 = 0;
    let mut fields_matched: i32 = 0; 
    for caps in PASSPORT_RE.captures_iter(input) {
        // if we're matching a non-cid field, count it.
        // we assume that no fields are duplicated,
        // and no extra fields are included.
        if &caps[1] != "cid" {
            fields_matched += 1;
            if fields_matched == TOTAL_FIELDS {
                valid += 1;
            }
        }
        // we've hit the end of the passport, so reset.
        if caps.get(2).is_some() {
            fields_matched = 0;
        }
    }
    return valid;
}


#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    lazy_static! {
        static ref PASSPORT_FIELD_RE: Regex = Regex::new(r"(.{3}):(\S+)(\n\n)?").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    let mut valid: i32 = 0;
    let mut fields_matched: i32 = 0; 
    for caps in PASSPORT_FIELD_RE.captures_iter(input) {
        let field: &str = &caps[1];
        let value: &str = &caps[2];
        let is_match: bool = match field {
            // cid (Country ID) - ignored, missing or not.
            "cid" => false,
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            "byr" => match value.parse::<i32>() {
                Ok(byr) => (1920..2003).contains(&byr),
                _ => false,
            },
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            "iyr" => match value.parse::<i32>() {
                Ok(iyr) => (2010..2021).contains(&iyr),
                _ => false,
            },
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            "eyr" => match value.parse::<i32>() {
                Ok(eyr) => (2020..2031).contains(&eyr),
                _ => false,
            },
            // hgt (Height) - a number followed by either cm or in:
            "hgt" => match HGT_RE.captures(value) {
                Some(hcaps) => match &hcaps[2] {
                    // If in, the number must be at least 59 and at most 76.
                    "in" => match &hcaps[1].parse::<i32>() {
                        Ok(hgt_in) => (59..77).contains(hgt_in),
                        _ => panic!(),
                    },
                    // If cm, the number must be at least 150 and at most 193.
                    "cm" => match &hcaps[1].parse::<i32>() {
                        Ok(hgt_cm) => (150..194).contains(hgt_cm),
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                None => false,
            },
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            "hcl" => HCL_RE.is_match(value),
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            "ecl" => ECL_RE.is_match(value),
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            "pid" => PID_RE.is_match(value),
            _ => panic!()
        };
        fields_matched += is_match as i32;
        // Avoid double counting a password that ends with `cid`
        if is_match && fields_matched == TOTAL_FIELDS {
            valid += 1;
        }
        // we've hit the end of the passport, so reset.
        if caps.get(3).is_some() {
            fields_matched = 0;
        }
    }
    return valid;
}


#[aoc(day4, part2, count)]
pub fn solve_part2_count(input: &str) -> i32 {
    lazy_static! {
        // NB: It looks like aoc_runner is stripping the trailing \n\n whitespace from `input`.
        static ref PASSPORT_RE: Regex = Regex::new(r"(?s)(.+?)(\n\n|$)").unwrap();
        static ref PASSPORT_FIELD_RE: Regex = Regex::new(r"(.{3}):(\S+)").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    PASSPORT_RE.captures_iter(input).filter(|caps| {
        PASSPORT_FIELD_RE.captures_iter(&caps[1]).filter(|field_cap| {
            let field: &str = &field_cap[1];
            let value: &str = &field_cap[2];
            match field {
                // cid (Country ID) - ignored, missing or not.
                "cid" => false,
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                "byr" => match value.parse::<i32>() {
                    Ok(byr) => (1920..2003).contains(&byr),
                    _ => false,
                },
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                "iyr" => match value.parse::<i32>() {
                    Ok(iyr) => (2010..2021).contains(&iyr),
                    _ => false,
                },
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                "eyr" => match value.parse::<i32>() {
                    Ok(eyr) => (2020..2031).contains(&eyr),
                    _ => false,
                },
                // hgt (Height) - a number followed by either cm or in:
                "hgt" => match HGT_RE.captures(value) {
                    Some(hcaps) => match &hcaps[2] {
                        // If in, the number must be at least 59 and at most 76.
                        "in" => match &hcaps[1].parse::<i32>() {
                            Ok(hgt_in) => (59..77).contains(hgt_in),
                            _ => panic!(),
                        },
                        // If cm, the number must be at least 150 and at most 193.
                        "cm" => match &hcaps[1].parse::<i32>() {
                            Ok(hgt_cm) => (150..194).contains(hgt_cm),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                    None => false,
                },
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                "hcl" => HCL_RE.is_match(value),
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                "ecl" => ECL_RE.is_match(value),
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                "pid" => PID_RE.is_match(value),
                _ => panic!("unexpected field")
            }
        }).count() as i32 == TOTAL_FIELDS
    }).count() as i32
}
