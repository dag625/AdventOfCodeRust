use anyhow::Result;
use regex::{Regex, CaptureLocations};

pub mod field_type {
    pub const BIRTH_YEAR : u16 = 0x0001;
    pub const ISSUE_YEAR : u16 = 0x0002;
    pub const EXPIRATION_YEAR : u16 = 0x0004;
    pub const HEIGHT : u16 = 0x0008;
    pub const HAIR_COLOR : u16 = 0x0010;
    pub const EYE_COLOR : u16 = 0x0020;
    pub const PASSPORT_ID : u16 = 0x0040;
    pub const COUNTRY_ID : u16 = 0x0080;
    pub const ALL_CREDENTIAL_FIELDS : u16 = 0x007f;
    pub const ALL_PASSPORT_FIELDS : u16 = 0x00ff;
    pub const NONE : u16 = 0x0000;
    pub const INVALID : u16 = 0xff00;
}

fn parse_type(name: &str) -> u16 {
    return
        if name == "byr" {
            field_type::BIRTH_YEAR
        }
        else if name == "iyr" {
            field_type::ISSUE_YEAR
        }
        else if name == "eyr" {
            field_type::EXPIRATION_YEAR
        }
        else if name == "hgt" {
            field_type::HEIGHT
        }
        else if name == "hcl" {
            field_type::HAIR_COLOR
        }
        else if name == "ecl" {
            field_type::EYE_COLOR
        }
        else if name == "pid" {
            field_type::PASSPORT_ID
        }
        else if name == "cid" {
            field_type::COUNTRY_ID
        }
        else {
            field_type::INVALID
        }
}

fn is_year_valid(s: &String, min: i32, max: i32) -> bool {
    let v = s.parse::<i32>();
    match v {
        Ok(v) => v >= min && v <= max,
        Err(..) => false
    }
}

fn is_height_valid(s: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]{2,3})(in|cm)").unwrap();
    }
    let cap = match RE.captures(s) { Some(v) => v, None => { return false; }};
    let val = match cap.get(1) {
        Some(c)=> { match c.as_str().parse::<i32>() { Ok(v) => v, Err(..) => { return false; } } },
        None => { return false; }
    };
    let inches = match cap.get(2) {
        Some(c)=> { c.as_str() == "in" },
        None => { return false; }
    };
    if inches {
        val >= 59 && val <= 76
    }
    else {
        val >= 150 && val <= 193
    }
}

fn is_hair_color_valid(s: &String) -> bool {
    if s.len() == 7 && s.chars().nth(0).unwrap() == '#' {
        s.chars().skip(1).all(|c| (c >= 'a' && c <= 'f') || (c >= '0' && c <= '9'))
    }
    else {
        false
    }
}

fn is_eye_color_valid(s: &String) -> bool {
    return s == "amb" ||
            s == "blu" ||
            s == "brn" ||
            s == "gry" ||
            s == "grn" ||
            s == "hzl" ||
            s == "oth";
}

fn is_passport_id_valid(s: &String) -> bool {
    if s.len() == 9 {
        s.chars().all(|c|c >= '0' && c <= '9')
    }
    else {
        false
    }
}

pub struct Field {
    field: u16,
    data: String
}

impl Field {
    pub fn is_valid(&self) -> bool {
        match self.field {
            field_type::BIRTH_YEAR => is_year_valid(&self.data, 1920, 2002),
            field_type::ISSUE_YEAR => is_year_valid(&self.data, 2010, 2020),
            field_type::EXPIRATION_YEAR => is_year_valid(&self.data, 2020, 2030),
            field_type::HEIGHT => is_height_valid(&self.data),
            field_type::HAIR_COLOR => is_hair_color_valid(&self.data),
            field_type::EYE_COLOR => is_eye_color_valid(&self.data),
            field_type::PASSPORT_ID => is_passport_id_valid(&self.data),
            field_type::COUNTRY_ID => true,
            _ => false
        }
    }
}

pub struct Id {
    fields: Vec<Field>,
}

impl Id {
    pub fn is_valid(&self, allow_credentials: bool, check_fields: bool) -> bool {
        let types = self.fields.iter().fold(field_type::NONE, |acc, f| {
            if acc == field_type::INVALID || f.field == field_type::INVALID { field_type::INVALID }
            else if check_fields && !f.is_valid() { field_type::INVALID }
            else { acc | f.field }
        });
        types == field_type::ALL_PASSPORT_FIELDS || (allow_credentials && types == field_type::ALL_CREDENTIAL_FIELDS)
    }
}

fn get_loc_str<'a, 'b>(file: &'a String, loc: &'b CaptureLocations, idx: usize) -> Result<&'a str> {
    let (start, end) = match loc.get(idx) {
        Some(v) => v,
        None => { return Err(anyhow::Error::msg("Invalid capture.")); }
    };
    Ok(&file[start..end])
}

pub fn parse(file: &String) -> Result<Vec<Id>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r##"([a-z]{3}):([A-Za-z0-9_#-]+)"##).unwrap();
    }
    //We do this as a performance optimization.  Otherwise we need to do something like
    //  file.chars().nth(..) twice inside the loop which makes this run noticably slow.
    let chars = file.chars().collect::<Vec<char>>();
    let mut locs : CaptureLocations = RE.capture_locations();
    let mut start : usize = 0;
    let mut retval = Vec::new();
    let mut working = Vec::new();
    loop {
        let om = RE.captures_read_at(&mut locs, file, start);
        let m = match om { None => { break; }, Some(v) => v };
        working.push(Field{
            field: parse_type(get_loc_str(file, &locs, 1)?),
            data: get_loc_str(file, &locs, 2)?.to_string()});
        if chars.len() >= m.end() + 1 && chars[m.end()] == '\n' && chars[m.end() + 1] == '\n' {
            retval.push(Id{fields: working});
            working = Vec::new();
        }
        start = m.end();
    }
    if !working.is_empty() {
        retval.push(Id{fields: working});
    }
    Ok(retval)
}

//------------------------------------------ Tests ------------------------------------------

#[test]
fn test_is_year_valid() {
    assert!(is_year_valid(&"2005".to_string(), 2000, 2010), "Valid test.");
    assert!(is_year_valid(&"2000".to_string(), 2000, 2010), "Valid min test.");
    assert!(is_year_valid(&"2010".to_string(), 2000, 2010), "Valid max test.");
    assert!(!is_year_valid(&"1999".to_string(), 2000, 2010), "Invalid min test.");
    assert!(!is_year_valid(&"2011".to_string(), 2000, 2010), "Invalid max test.");
    assert!(!is_year_valid(&"George".to_string(), 2000, 2010), "Invalid number test.");
}

#[test]
fn test_is_height_valid() {
    assert!(is_height_valid(&"67in".to_string()), "Valid inches test.");
    assert!(is_height_valid(&"59in".to_string()), "Valid min inches test.");
    assert!(is_height_valid(&"76in".to_string()), "Valid max inches test.");
    assert!(!is_height_valid(&"58in".to_string()), "Invalid min inches test.");
    assert!(!is_height_valid(&"77in".to_string()), "Invalid max inches test.");
    assert!(is_height_valid(&"171cm".to_string()), "Valid centimeters test.");
    assert!(is_height_valid(&"150cm".to_string()), "Valid min centimeters test.");
    assert!(is_height_valid(&"193cm".to_string()), "Valid max centimeters test.");
    assert!(!is_height_valid(&"149cm".to_string()), "Invalid min centimeters test.");
    assert!(!is_height_valid(&"194cm".to_string()), "Invalid max centimeters test.");
    assert!(!is_height_valid(&"20km".to_string()), "Invalid units test.");
    assert!(!is_height_valid(&"2in".to_string()), "Invalid small number test.");
    assert!(!is_height_valid(&"2200cm".to_string()), "Invalid large number test.");
    assert!(!is_height_valid(&"ruin".to_string()), "Invalid no number test.");
}