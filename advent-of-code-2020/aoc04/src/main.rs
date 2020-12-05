// TODO: this is broken...

use regex::Regex;

type Res<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn passport_isvalid(&self) -> bool {
        self.hgt == true &&
         self.eyr == true &&
         self.ecl == true &&
         self.pid == true &&
         self.hcl == true &&
         self.byr == true &&
         self.iyr == true
    }
}

fn regex_check(regex: &str, passport_string: &str) -> bool {
    Regex::new(regex).unwrap().is_match(&passport_string)
}

fn main() -> Res<()> {
    let input: String = std::fs::read_to_string("input/input.txt").unwrap();

    let s: Vec<String> =  input.split("\n")
        .map(|l| l.parse()
        .unwrap())
        .collect();

    let valid_passports: Vec<Passport> = s.into_iter().map(|passport| {
        println!("passport:: not formatted: {}",  passport);
        println!("passport:: {:?}",  Passport {
            byr: regex_check(r"byr:(\S+)", &passport),
            iyr: regex_check(r"iyr:(\S+)", &passport),
            eyr: regex_check(r"eyr:(\S+)", &passport),
            hgt: regex_check(r"hgt:(\S+)", &passport),
            hcl: regex_check(r"hcl:(\S+)", &passport),
            ecl: regex_check(r"ecl:(\S+)", &passport),
            pid: regex_check(r"pid:(\S+)", &passport),
            cid: regex_check(r"cid:(\S+)", &passport),
        });
        return Passport {
            byr: regex_check(r"byr:(\S+)", &passport),
            iyr: regex_check(r"iyr:(\S+)", &passport),
            eyr: regex_check(r"eyr:(\S+)", &passport),
            hgt: regex_check(r"hgt:(\S+)", &passport),
            hcl: regex_check(r"hcl:(\S+)", &passport),
            ecl: regex_check(r"ecl:(\S+)", &passport),
            pid: regex_check(r"pid:(\S+)", &passport),
            cid: regex_check(r"cid:(\S+)", &passport),
        };
    }).collect();

    let mut i: u32 = 0;
    for pass in &valid_passports {
        if pass.passport_isvalid() {
            i += 1
        }
    }

    println!("{}", i);


    // valid_passports.collect::<Vec<Passport>>().into_iter().filter(|line|
    //     line.passport_isvalid()
    // ).collect();

    println!("valid passports: {:?}", valid_passports);

    Ok(())
}