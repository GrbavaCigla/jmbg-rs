mod regions;

#[derive(Debug)]
pub struct JMBG<'a>(pub &'a str);

pub enum Gender {
    Male,
    Female,
}

impl JMBG<'_> {

    /// Returns day of birth.
    pub fn day(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[..2].parse::<u8>()
    }

    /// Returns month of birth.
    pub fn month(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[2..4].parse::<u8>()
    }

    /// Returns year of birth.  
    /// Since JMBG holds last 3 digits of year, I can only guess what is the first digit.  
    /// Algorithm guesses by checking if 3 digit year is greater than 870 and adds 1 to the beginning.  
    /// If it isn't greater than 870, it adds 2 to the beginning.  
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

    /// Returns regional number.
    pub fn region_num(&self) -> std::result::Result<u8, std::num::ParseIntError> {
        self.0[7..9].parse::<u8>()
    }

    /// Returns region and country based on regional number.  
    /// If region is empty or regional number represents foreigner or something similar  
    /// string is empty.
    pub fn region(&self) -> std::result::Result<(&str, &str), std::num::ParseIntError> {
        let code = match self.region_num() {
            Ok(code) => code,
            Err(err) => return Err(err),
        };
        Ok((regions::region(code), regions::country(code)))
    }

    /// Birth number (id).
    pub fn birth_num(&self) -> std::result::Result<u16, std::num::ParseIntError> {
        self.0[9..12].parse::<u16>()
    }

    /// Returns gender. If birth number is less than 500 returns male, else female.
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

    /// Last digit of JMBG is reserved for checksum.
    pub fn checksum(&self) -> std::option::Option<u8> {
        match self.0.chars().nth(12_usize) {
            Some(value) => match value.to_digit(10) {
                Some(value) => Some(value as u8),
                None => return None
            },
            None => return None
        }
    }

    /// Checks if JMBG is valid by checking if checksum passes.  
    /// Formula for checksum is:  
    /// `m = 11 − (( 7*(a+g) + 6*(b+h) + 5*(c+i) + 4*(d+j) + 3*(e+k) + 2*(f+l) ) mod 11)`  
    ///  - If m is between 1 and 9, the number checksum is the same as the number m  
    ///  - If m is 10 or 11 checksum becomes 0  
    /// 
    /// Where a, b, c, d, e, f, g, h, i, j, k, l, m are digits of JMBG.
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
