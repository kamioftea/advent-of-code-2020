//! This is my solution for [Advent of Code - Day 4](https://adventofcode.com/2020/day/4) -
//! _Passport Processing_
//!
//! Today is themed around data normalisation and validation. The data is all presented in
//! `key:value` pairs, but each record can be over multiple lines, and may be missing some of the
//! fields. Part two adds in needing to validate the data differently based on the key.
//!
//! Parsing and normalising is done by [`parse_passports`]. The validation for both parts is handled
//! by methods implemented for [`Passport`], especially ['Passport::has_valid_fields`] and [`Passport::is_valid'].

use std::fs;
use std::collections::HashMap;
use regex::Regex;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let data = contents.as_str();
    let count = parse_passports(data)
        .iter()
        .filter(|pass| pass.has_valid_fields())
        .count();

    println!("There are {} passports with 'valid' fields", count);

    let count = parse_passports(data)
        .iter()
        .filter(|pass| pass.is_valid())
        .count();

    println!("There are {} 'valid' passports", count);
}

/// Holds the data for a possibly valid passport
#[derive(Debug, Eq, PartialEq)]
struct Passport<'a> {
    /// _Birth Year_ - four digits; at least 1920 and at most 2002.
    byr: Option<&'a str>,
    /// _Country ID_ - ignored, missing or not.
    cid: Option<&'a str>,
    /// _Eye Color_ - exactly one of: amb blu brn gry grn hzl oth.
    ecl: Option<&'a str>,
    /// _Expiration Year_ - four digits; at least 2020 and at most 2030.
    eyr: Option<&'a str>,
    /// _Hair Color_ - a # followed by exactly six characters 0-9 or a-f.
    hcl: Option<&'a str>,
    /// Height_)_- a number followed by either cm or in
    hgt: Option<&'a str>,
    /// _Issue Year_ - four digits; at least 2010 and at most 2020.
    iyr: Option<&'a str>,
    /// _Passport ID_ - a nine-digit number, including leading zeroes.
    pid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    /// Convert a map built from the import data into a Passport. See also [`parse_passports`]
    fn from_map(map: HashMap<&str, &'a str>) -> Passport<'a> {
        Passport {
            /// Birst
            byr: map.get("byr").map(|str| *str),
            cid: map.get("cid").map(|str| *str),
            ecl: map.get("ecl").map(|str| *str),
            eyr: map.get("eyr").map(|str| *str),
            hcl: map.get("hcl").map(|str| *str),
            hgt: map.get("hgt").map(|str| *str),
            iyr: map.get("iyr").map(|str| *str),
            pid: map.get("pid").map(|str| *str),
        }
    }

    /// Solution to part one, just needs to check all required fields are present.
    ///
    /// # Example from Text
    /// ```
    /// let valid: Vec<bool> =
    ///     parse_passports(PART_1_DATA).into_iter().map(|p| p.is_valid()).collect();
    /// assert_eq!(vec!(true, false, true, false), valid);
    /// ```
    fn has_valid_fields(&self) -> bool {
        self.byr.is_some() &&
            self.ecl.is_some() &&
            self.eyr.is_some() &&
            self.hcl.is_some() &&
            self.hgt.is_some() &&
            self.iyr.is_some() &&
            self.pid.is_some()
    }

    /// Solution to part 2/ Validate values based on what they represent.
    ///
    /// Most of the work is delegated to field specific validators
    /// - [`Passport::is_valid_year`]
    /// - [`Passport::is_valid_height`]
    /// - [`Passport::is_valid_hair_colour`]
    /// - [`Passport::is_valid_eye_colour`]
    /// - [`Passport::is_valid_passport_id`]
    ///
    /// # Examples from Tests
    /// ```
    /// let invalid_passports: Vec<bool> =
    ///     parse_passports(
    /// "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    /// byr:1937 iyr:2017 cid:147 hgt:183cm
    ///
    /// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    /// hcl:#cfa07d byr:1929
    ///
    /// hcl:#ae17e1 iyr:2013
    /// eyr:2024
    /// ecl:brn pid:760753108 byr:1931
    /// hgt:179cm
    ///
    /// hcl:#cfa07d eyr:2025 pid:166559648
    /// iyr:2011 ecl:brn hgt:59in")
    ///         .iter()
    ///         .map(|pass| pass.is_valid())
    ///         .collect();
    /// assert_eq!(
    ///     vec!(false, false, false, false),
    ///     invalid_passports
    /// );
    /// let valid_passports: Vec<bool> =
    ///     parse_passports("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    /// hcl:#623a2f
    ///
    /// eyr:2029 ecl:blu cid:129 byr:1989
    /// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
    ///
    /// hcl:#888785
    /// hgt:164cm byr:2001 iyr:2015 cid:88
    /// pid:545766238 ecl:hzl
    /// eyr:2022
    ///
    /// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")
    ///         .iter()
    ///         .map(|pass| pass.is_valid())
    ///         .collect();
    /// assert_eq!(
    ///     vec!(true, true, true, true),
    ///     valid_passports
    /// )
    /// ```
    fn is_valid(&self) -> bool {
        Passport::is_valid_year(self.byr, 1920, 2002)
            && Passport::is_valid_year(self.iyr, 2010, 2020)
            && Passport::is_valid_year(self.eyr, 2020, 2030)
            && Passport::is_valid_height(self.hgt)
            && Passport::is_valid_hair_colour(self.hcl)
            && Passport::is_valid_eye_colour(self.ecl)
            && Passport::is_valid_passport_id(self.pid)
    }

    /// Checks if an optional string is a valid year
    ///
    /// > byr (Birth Year)      - four digits; at least 1920 and at most 2002.
    /// > iyr (Issue Year)      - four digits; at least 2010 and at most 2020.
    /// > eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    ///
    /// # Example from Tests
    /// ```
    /// assert_eq!(true, Passport::is_valid_year(Some("2002"), 1920, 2002));
    /// assert_eq!(false, Passport::is_valid_year(Some("2003"), 1920, 2002));
    /// assert_eq!(false, Passport::is_valid_year(Some("1919"), 1920, 2002));
    /// assert_eq!(false, Passport::is_valid_year(None, 1920, 2002));
    /// ```
    fn is_valid_year(maybe_year: Option<&str>, min: u16, max: u16) -> bool {
        match maybe_year {
            Some(year) if Regex::new(r"^\d{4}$").unwrap().is_match(year) => {
                let as_num = year.parse::<u16>().unwrap();
                return min <= as_num && as_num <= max
            }
            _ => false
        }
    }

    /// Checks if an optional string is a valid height
    ///
    /// > hgt (Height) - a number followed by either cm or in:
    /// > - If cm, the number must be at least 150 and at most 193.
    /// > - If in, the number must be at least 59 and at most 76.
    ///
    /// # Examples from Tests:
    /// ```
    /// assert_eq!(true, Passport::is_valid_height(Some("60in")));
    /// assert_eq!(true, Passport::is_valid_height(Some("190cm")));
    /// assert_eq!(false, Passport::is_valid_height(Some("190in")));
    /// assert_eq!(false, Passport::is_valid_height(Some("190")));
    /// assert_eq!(false, Passport::is_valid_height(None));
    /// ```
    fn is_valid_height(maybe_hgt: Option<&str>) -> bool {
        let hgt_re = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        let hgt =
            maybe_hgt
                .map(|s| hgt_re.captures(s))
                .flatten()
                .map(|cap| (
                    cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                    cap.get(2).unwrap().as_str())
                );

        match hgt {
            Some((cm, "cm")) if cm >= 150 && cm <= 193 => true,
            Some((inch, "in")) if inch >= 59 && inch <= 76 => true,
            _ => false
        }
    }

    /// Checks if an optional string is a valid hair colour
    ///
    /// > hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f
    ///
    /// # Examples from Tests:
    /// ```
    /// assert_eq!(true, Passport::is_valid_hair_colour(Some("#123abc")));
    /// assert_eq!(false, Passport::is_valid_hair_colour(Some("#123abz")));
    /// assert_eq!(false, Passport::is_valid_hair_colour(Some("123abc")));
    /// assert_eq!(false, Passport::is_valid_hair_colour(None));
    /// ```
    fn is_valid_hair_colour(maybe_hcl: Option<&str>) -> bool {
        let hcl_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        match maybe_hcl {
            Some(hcl) if hcl_re.is_match(hcl) => true,
            _ => return false
        }
    }

    /// Checks if an optional string is a valid eye colour
    ///
    /// > ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    ///
    /// # Examples from Tests:
    /// ```
    /// assert_eq!(true, Passport::is_valid_eye_colour(Some("brn")));
    /// assert_eq!(false, Passport::is_valid_eye_colour(Some("wat")));
    /// assert_eq!(false, Passport::is_valid_eye_colour(None));
    /// ```
    fn is_valid_eye_colour(maybe_ecl: Option<&str>) -> bool {
        let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        match maybe_ecl {
            Some(ecl) if ecl_re.is_match(ecl) => true,
            _ => false
        }
    }

    /// Checks if an optional string is a valid passport id
    ///
    /// > pid (Passport ID) - a nine-digit number, including leading zeroes.
    ///
    /// # Examples from Tests:
    /// ```
    /// assert_eq!(true, Passport::is_valid_passport_id(Some("000000001")));
    /// assert_eq!(true, Passport::is_valid_passport_id(Some("123456789")));
    /// assert_eq!(false, Passport::is_valid_passport_id(Some("00000001")));
    /// assert_eq!(false, Passport::is_valid_passport_id(Some("0123456789")));
    /// assert_eq!(false, Passport::is_valid_passport_id(Some("abcdefghi")));
    /// assert_eq!(false, Passport::is_valid_passport_id(None));
    /// ```
    fn is_valid_passport_id(maybe_pid: Option<&str>) -> bool {
        let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
        match maybe_pid {
            Some(pid) if pid_re.is_match(pid) => true,
            _ => false
        }
    }
}

/// Parse the input into a list of passports
///
/// Loop through the lines, and for each line, loop through the matches for a regular expression
/// that matches a record, appending those to a temporary map. Once a blank line is encountered
/// indicating a new record, a Passport is built using [`Passport::from_map`] and appended to the
/// output, then the map is reset.
///
/// # Example from Tests
/// ```
/// assert_eq!(
///     vec!(
///         Passport {
///             byr: Some("1937"),
///             cid: Some("147"),
///             ecl: Some("gry"),
///             eyr: Some("2020"),
///             hcl: Some("#fffffd"),
///             hgt: Some("183cm"),
///             iyr: Some("2017"),
///             pid: Some("860033327")
///         },
///         Passport {
///             byr: Some("1929"),
///             cid: Some("350"),
///             ecl: Some("amb"),
///             eyr: Some("2023"),
///             hcl: Some("#cfa07d"),
///             hgt: None,
///             iyr: Some("2013"),
///             pid: Some("028048884")
///         },
///         Passport {
///             byr: Some("1931"),
///             cid: None,
///             ecl: Some("brn"),
///             eyr: Some("2024"),
///             hcl: Some("#ae17e1"),
///             hgt: Some("179cm"),
///             iyr: Some("2013"),
///             pid: Some("760753108")
///         },
///         Passport {
///             byr: None,
///             cid: None,
///             ecl: Some("brn"),
///             eyr: Some("2025"),
///             hcl: Some("#cfa07d"),
///             hgt: Some("59in"),
///             iyr: Some("2011"),
///             pid: Some("166559648")
///         },
///     ),
///     parse_passports(
///         "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
/// byr:1937 iyr:2017 cid:147 hgt:183cm
///
/// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
/// hcl:#cfa07d byr:1929
///
/// hcl:#ae17e1 iyr:2013
/// eyr:2024
/// ecl:brn pid:760753108 byr:1931
/// hgt:179cm
///
/// hcl:#cfa07d eyr:2025 pid:166559648
/// iyr:2011 ecl:brn hgt:59in"/
///     )
/// )
/// ```
fn parse_passports<'a>(data: &'a str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut building: HashMap<&str, &'a str> = HashMap::new();
    let re = Regex::new(r"([a-z]{3}):([^\s]+)").unwrap();
    for line in data.lines() {
        if line.is_empty() {
            passports.push(Passport::from_map(building.clone()));
            building = HashMap::new();
        } else {
            for capture in re.captures_iter(line) {
                building.insert(
                    capture.get(1).unwrap().as_str(),
                    capture.get(2).unwrap().as_str(),
                );
            }
        }
    }

    if !building.is_empty() {
        passports.push(Passport::from_map(building));
    }

    passports
}

#[cfg(test)]
mod tests {
    use day_4::{parse_passports, Passport};

    static PART_1_DATA: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static PART_2_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static PART_2_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn can_parse_passports() {
        assert_eq!(
            vec!(
                Passport {
                    byr: Some("1937"),
                    cid: Some("147"),
                    ecl: Some("gry"),
                    eyr: Some("2020"),
                    hcl: Some("#fffffd"),
                    hgt: Some("183cm"),
                    iyr: Some("2017"),
                    pid: Some("860033327"),
                },
                Passport {
                    byr: Some("1929"),
                    cid: Some("350"),
                    ecl: Some("amb"),
                    eyr: Some("2023"),
                    hcl: Some("#cfa07d"),
                    hgt: None,
                    iyr: Some("2013"),
                    pid: Some("028048884"),
                },
                Passport {
                    byr: Some("1931"),
                    cid: None,
                    ecl: Some("brn"),
                    eyr: Some("2024"),
                    hcl: Some("#ae17e1"),
                    hgt: Some("179cm"),
                    iyr: Some("2013"),
                    pid: Some("760753108"),
                },
                Passport {
                    byr: None,
                    cid: None,
                    ecl: Some("brn"),
                    eyr: Some("2025"),
                    hcl: Some("#cfa07d"),
                    hgt: Some("59in"),
                    iyr: Some("2011"),
                    pid: Some("166559648"),
                },
            ),
            parse_passports(PART_1_DATA)
        )
    }

    #[test]
    fn can_validate_year() {
        assert_eq!(true, Passport::is_valid_year(Some("2002"), 1920, 2002));
        assert_eq!(false, Passport::is_valid_year(Some("2003"), 1920, 2002));
        assert_eq!(false, Passport::is_valid_year(Some("1919"), 1920, 2002));
        assert_eq!(false, Passport::is_valid_year(None, 1920, 2002));
    }

    #[test]
    fn can_validate_height() {
        assert_eq!(true, Passport::is_valid_height(Some("60in")));
        assert_eq!(true, Passport::is_valid_height(Some("190cm")));
        assert_eq!(false, Passport::is_valid_height(Some("190in")));
        assert_eq!(false, Passport::is_valid_height(Some("190")));
        assert_eq!(false, Passport::is_valid_height(None));
    }

    #[test]
    fn can_validate_hair_colour() {
        assert_eq!(true, Passport::is_valid_hair_colour(Some("#123abc")));
        assert_eq!(false, Passport::is_valid_hair_colour(Some("#123abz")));
        assert_eq!(false, Passport::is_valid_hair_colour(Some("123abc")));
        assert_eq!(false, Passport::is_valid_hair_colour(None));
    }

    #[test]
    fn can_validate_eye_colour() {
        assert_eq!(true, Passport::is_valid_eye_colour(Some("brn")));
        assert_eq!(false, Passport::is_valid_eye_colour(Some("wat")));
        assert_eq!(false, Passport::is_valid_eye_colour(None));
    }

    #[test]
    fn can_validate_passport_ids() {
        assert_eq!(true, Passport::is_valid_passport_id(Some("000000001")));
        assert_eq!(true, Passport::is_valid_passport_id(Some("123456789")));
        assert_eq!(false, Passport::is_valid_passport_id(Some("00000001")));
        assert_eq!(false, Passport::is_valid_passport_id(Some("0123456789")));
        assert_eq!(false, Passport::is_valid_passport_id(Some("abcdefghi")));
        assert_eq!(false, Passport::is_valid_passport_id(None));
    }

    #[test]
    fn can_validate_passports() {
        let valid: Vec<bool> =
            parse_passports(PART_1_DATA).into_iter().map(|p| p.is_valid()).collect();
        assert_eq!(vec!(true, false, true, false), valid);
    }

    #[test]
    fn can_validate_passport_fields() {
        let invalid_passports: Vec<bool> =
            parse_passports(PART_2_INVALID)
                .iter()
                .map(|pass| pass.is_valid())
                .collect();

        assert_eq!(
            vec!(false, false, false, false),
            invalid_passports
        );

        let valid_passports: Vec<bool> =
            parse_passports(PART_2_VALID)
                .iter()
                .map(|pass| pass.is_valid())
                .collect();

        assert_eq!(
            vec!(true, true, true, true),
            valid_passports
        )
    }
}
