use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Passport<'a> {
    byr: Option<&'a str>,
    cid: Option<&'a str>,
    ecl: Option<&'a str>,
    eyr: Option<&'a str>,
    hcl: Option<&'a str>,
    hgt: Option<&'a str>,
    iyr: Option<&'a str>,
    pid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn from_map(map: HashMap<&str, &'a str>) -> Passport<'a> {
        Passport {
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

    // all fields required except cid
    fn has_valid_fields(&self) -> bool {
        self.byr.is_some() &&
            self.ecl.is_some() &&
            self.eyr.is_some() &&
            self.hcl.is_some() &&
            self.hgt.is_some() &&
            self.iyr.is_some() &&
            self.pid.is_some()
    }

    // You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:
    //
    //     byr (Birth Year) - four digits; at least 1920 and at most 2002.
    //     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    //     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    //     hgt (Height) - a number followed by either cm or in:
    //         If cm, the number must be at least 150 and at most 193.
    //         If in, the number must be at least 59 and at most 76.
    //     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    //     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    //     pid (Passport ID) - a nine-digit number, including leading zeroes.
    //     cid (Country ID) - ignored, missing or not.
    fn is_valid(&self) -> bool {
        match self.byr {
            None => return false,
            Some(byr) if !<Passport<'a>>::is_valid_year(byr, 1920, 2002) => return false,
            _ => ()
        };

        match self.iyr {
            None => return false,
            Some(iyr) if !<Passport<'a>>::is_valid_year(iyr, 2010, 2020) => return false,
            _ => ()
        };

        match self.eyr {
            None => return false,
            Some(byr) if !<Passport<'a>>::is_valid_year(byr, 2020, 2030) => return false,
            _ => ()
        };

        let hgt_re = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        let hgt =
            self.hgt
                .map(|s| hgt_re.captures(s))
                .flatten()
                .map(|cap| (
                    cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                    cap.get(2).unwrap().as_str())
                );
        
        match hgt {
            Some((cm, "cm")) if cm >= 150 && cm <= 193 => (),
            Some((inch, "in")) if inch >= 59 && inch <= 76 => (),
            _ => return false
        }
        
        let hcl_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        match self.hcl {
            Some(hcl) if hcl_re.is_match(hcl) => (),
            _ => return false
        }
        
        let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        match self.ecl {
            Some(ecl) if ecl_re.is_match(ecl) => (),
            _ => return false
        }
        
        let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
        match self.pid {
            Some(pid) if pid_re.is_match(pid) => (),
            _ => return false
        }

        true
    }

    fn is_valid_year(year: &str, min: u16, max: u16) -> bool {
        if Regex::new(r"^\d{4}$").unwrap().is_match(year) {
            let as_num = year.parse::<u16>().unwrap();
            return min <= as_num && as_num <= max
        }

        false
    }
}

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
