use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::result::Result;

struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn set(&mut self, field: &str, value: &str) -> Result<(), &str> {
        match field {
            "byr" => self.byr = Some(value.parse::<usize>().unwrap()),
            "iyr" => self.iyr = Some(value.parse::<usize>().unwrap()),
            "eyr" => self.eyr = Some(value.parse::<usize>().unwrap()),
            "hgt" => self.hgt = Some(String::from(value)),
            "hcl" => self.hcl = Some(String::from(value)),
            "ecl" => self.ecl = Some(String::from(value)),
            "pid" => self.pid = Some(String::from(value)),
            "cid" => self.cid = Some(String::from(value)),
            _ => return Err("Invalid field."),
        }
        Ok(())
    }

    fn is_filled(&self) -> bool {
        self.byr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
            && self.iyr.is_some()
    }

    fn valid_year(year: Option<usize>, min: usize, max: usize) -> Option<usize> {
        let yr = year?;
        if yr >= min && yr <= max {
            return Some(yr);
        }
        None
    }

    fn valid_height(height: &Option<String>) -> Option<usize> {
        let hgt = height.as_ref()?;
        if hgt.len() < 3 {
            return None;
        }

        let suffix = &hgt[hgt.len() - 2..];
        let val = hgt[..hgt.len() - 2].parse::<usize>().ok()?;
        match suffix {
            "in" if val >= 59 && val <= 76 => Some(val),
            "cm" if val >= 150 && val <= 193 => Some(val),
            _ => None,
        }
    }

    fn valid_hcl(haircolour: &Option<String>) -> Option<&String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?i)#[a-f0-9]{6}$").unwrap();
        }
        let hcl = haircolour.as_ref()?;
        if RE.is_match(hcl.as_str()) {
            return Some(hcl);
        }
        None
    }

    fn valid_ecl(eyecolour: &Option<String>) -> Option<&String> {
        let ecl = eyecolour.as_ref()?;
        let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if valid.contains(&ecl.as_str()) {
            return Some(ecl);
        }
        None
    }

    fn valid_pid(passportid: &Option<String>) -> Option<&String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        let pid = passportid.as_ref()?;
        if RE.is_match(pid.as_str()) {
            return Some(pid);
        }
        None
    }

    fn is_valid(&self) -> bool {
        Passport::valid_year(self.byr, 1920, 2002).is_some()
            && Passport::valid_year(self.iyr, 2010, 2020).is_some()
            && Passport::valid_year(self.eyr, 2020, 2030).is_some()
            && Passport::valid_height(&self.hgt).is_some()
            && Passport::valid_hcl(&self.hcl).is_some()
            && Passport::valid_ecl(&self.ecl).is_some()
            && Passport::valid_pid(&self.pid).is_some()
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|entry| parse_passport(entry))
        .collect()
}

fn parse_passport(entry: &str) -> Passport {
    let mut passport = Passport::new();
    let fields = entry.split_whitespace();
    for field in fields {
        let mut parts = field.split(':');
        let name = parts.next().unwrap();
        let value = parts.next().unwrap();
        passport.set(name, value).unwrap();
    }
    passport
}

#[aoc(day4, part1)]
fn check_filled(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|&p| p.is_filled()).count()
}

#[aoc(day4, part2)]
fn check_valid(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|&p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::Passport;
    #[test]
    fn yr() {
        assert!(Passport::valid_year(None, 1920, 2002).is_none());
        assert!(Passport::valid_year(Some(1919), 1920, 2002).is_none());
        assert!(Passport::valid_year(Some(1920), 1920, 2002).is_some());
        assert!(Passport::valid_year(Some(2002), 1920, 2002).is_some());
        assert!(Passport::valid_year(Some(2003), 1920, 2002).is_none());
    }
    #[test]
    fn hgt() {
        assert!(Passport::valid_height(&None).is_none());
        assert!(Passport::valid_height(&Some("149cm".to_string())).is_none());
        assert!(Passport::valid_height(&Some("150cm".to_string())).is_some());
        assert!(Passport::valid_height(&Some("193cm".to_string())).is_some());
        assert!(Passport::valid_height(&Some("194cm".to_string())).is_none());
        assert!(Passport::valid_height(&Some("58in".to_string())).is_none());
        assert!(Passport::valid_height(&Some("59in".to_string())).is_some());
        assert!(Passport::valid_height(&Some("76in".to_string())).is_some());
        assert!(Passport::valid_height(&Some("77in".to_string())).is_none());
    }
    #[test]
    fn hcl() {
        assert!(Passport::valid_hcl(&None).is_none());
        assert!(Passport::valid_hcl(&Some("#000000".to_string())).is_some());
        assert!(Passport::valid_hcl(&Some("#FFFFFF".to_string())).is_some());
        assert!(Passport::valid_hcl(&Some("#999999".to_string())).is_some());
        assert!(Passport::valid_hcl(&Some("#036ACF".to_string())).is_some());
        assert!(Passport::valid_hcl(&Some("#gggggg".to_string())).is_none());
        assert!(Passport::valid_hcl(&Some("#111".to_string())).is_none());
        assert!(Passport::valid_hcl(&Some("#9999999".to_string())).is_none());
    }

    #[test]
    fn ecl() {
        assert!(Passport::valid_ecl(&None).is_none());
        assert!(Passport::valid_ecl(&Some("amb".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("blu".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("brn".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("gry".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("grn".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("hzl".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("oth".to_string())).is_some());
        assert!(Passport::valid_ecl(&Some("red".to_string())).is_none());
        assert!(Passport::valid_ecl(&Some("bl".to_string())).is_none());
        assert!(Passport::valid_ecl(&Some("amber".to_string())).is_none());
    }

    #[test]
    fn pid() {
        assert!(Passport::valid_pid(&None).is_none());
        assert!(Passport::valid_pid(&Some("123456789".to_string())).is_some());
        assert!(Passport::valid_pid(&Some("023456789".to_string())).is_some());
        assert!(Passport::valid_pid(&Some("123456780".to_string())).is_some());
        assert!(Passport::valid_pid(&Some("12345678".to_string())).is_none());
        assert!(Passport::valid_pid(&Some("0123456789".to_string())).is_none());
        assert!(Passport::valid_pid(&Some("1234567890".to_string())).is_none());
        assert!(Passport::valid_pid(&Some("1A3456789".to_string())).is_none());
        assert!(Passport::valid_pid(&Some("".to_string())).is_none());
    }
}
