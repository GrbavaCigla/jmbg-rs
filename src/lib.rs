mod regions;

#[derive(Debug)]
pub struct JMBG<'a>(pub &'a str);

pub enum Gender {
    Male,
    Female,
}

macro_rules! return_error {
    ($result:expr) => {{
        match $result {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        }
    }};
}

impl JMBG<'_> {
    pub fn day(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[..2].parse::<u8>()
    }

    pub fn month(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[2..4].parse::<u8>()
    }

    pub fn year(&self) -> std::result::Result<u16, std::num::ParseIntError> {
        let mut year_num = match self.0[4..7].parse::<u16>() {
            Ok(_year) => _year,
            Err(err) => return Err(err),
        };

        if year_num > 870 {
            year_num += 1000;
        } else {
            year_num += 2000;
        }

        Ok(year_num)
    }

    pub fn region_num(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[7..9].parse::<u8>()
    }

    pub fn region(&self) -> std::result::Result<(&str, &str), std::num::ParseIntError> {
        let code = match self.region_num() {
            Ok(code) => code,
            Err(err) => return Err(err),
        };
        Ok((regions::region(code), regions::country(code)))
    }

    pub fn birth_num(&self) -> std::result::Result<u16, std::num::ParseIntError> {
        self.0[9..12].parse::<u16>()
    }

    pub fn gender(&self) -> std::result::Result<Gender, std::num::ParseIntError> {
        let code = match self.birth_num() {
            Ok(code) => code,
            Err(err) => return Err(err),
        };

        if code < 500 {
            return Ok(Gender::Male);
        } else {
            return Ok(Gender::Female);
        }
    }

    pub fn checksum(&self) -> std::option::Option<u8> {
        match self.0.chars().nth(12_usize) {
            Some(value) => match value.to_digit(10) {
                Some(value) => Some(value as u8),
                None => return None
            },
            None => return None
        }
    }

    pub fn is_valid(&self) -> std::result::Result<bool, std::num::ParseIntError> {
        let jmbg_int = match self.0.parse::<u64>() {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        let mut digits = [0; 13];
        for i in (0..13).rev() {
            digits[12 - i] = (jmbg_int / 10_u64.pow(i as u32) % 10) as u8;
        }

        let m = 11
            - ((7 * (digits[0] + digits[6])
                + 6 * (digits[1] + digits[7])
                + 5 * (digits[2] + digits[8])
                + 4 * (digits[3] + digits[9])
                + 3 * (digits[4] + digits[10])
                + 2 * (digits[5] + digits[11]))
                % 11);

        let k = match m {
            10 => 0,
            11 => 0,
            _ => m
        };
        
        Ok(digits[12] == k)
    }
}
