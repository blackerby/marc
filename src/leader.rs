use std::fmt;

const BASE_START: usize = 12;
const BASE_END: usize = 16;
const FIELD_TERMINATOR: usize = 0x1E;
const SUBFIELD_TERMINATOR: usize = 0x1F;
const RECORD_TERMINATOR: usize = 0x1D;
const RECORD_LENGTH_END: usize = 4;

#[derive(Debug)]
struct Leader {
    raw: String,
    record_length: usize,
    offset: usize,
    status: char,
    record_type: char,
    bib_level: char,
    control: char,
    encoding_scheme: char,
    indicator_count: char,
    subfield_length: char,
    encoding_level: char,
    form: char,
    multipart: char,
    field_length: char,
    start_char_pos_length: char,
    imp_def_length: char,
    undefined: char,
}

impl Leader {
    fn new(s: &String) -> Result<Leader, &'static str> {
        if s.len() < 24 {
            return Err("incomplete leader");
        } else {
            let record_length = s[..=RECORD_LENGTH_END].parse().unwrap();
            let offset = s[BASE_START..=BASE_END].parse().unwrap();
            let chars: Vec<char> = s.chars().collect();
            Ok(Leader {
                raw: s.to_string(),
                // bytes 00-04
                record_length,
                status: chars[5],
                record_type: chars[6],
                bib_level: chars[7],
                control: chars[8],
                encoding_scheme: chars[9],
                indicator_count: chars[10],
                subfield_length: chars[11],
                // bytes 12-16
                offset,
                encoding_level: chars[17],
                form: chars[18],
                multipart: chars[19],
                // entry map
                field_length: chars[20],
                start_char_pos_length: chars[21],
                imp_def_length: chars[22],
                undefined: chars[23],
            })
        }
    }
}

impl fmt::Display for Leader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=LDR {}", self.raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_leader() {
        let input = String::from("01848nam  2200385 i 4500");
        let leader = Leader::new(&input).unwrap();

        assert_eq!(input, leader.raw);
        assert_eq!(1848, leader.record_length);
        assert_eq!('n', leader.status);
        assert_eq!('a', leader.record_type);
        assert_eq!('m', leader.bib_level);
        assert_eq!(' ', leader.control);
        assert_eq!(' ', leader.encoding_scheme);
        assert_eq!('2', leader.indicator_count);
        assert_eq!('2', leader.subfield_length);
        assert_eq!(385, leader.offset);
        assert_eq!(' ', leader.encoding_level);
        assert_eq!('i', leader.form);
        assert_eq!(' ', leader.multipart);
        assert_eq!('4', leader.field_length);
        assert_eq!('5', leader.start_char_pos_length);
        assert_eq!('0', leader.imp_def_length);
        assert_eq!('0', leader.undefined);
    }

    #[test]
    fn display() {
        let input = String::from("01848nam  2200385 i 4500");
        let leader = Leader::new(&input).unwrap();
        assert_eq!(format!("=LDR {}", input), leader.to_string());
    }
}
