use std::{collections::HashMap, str};

use crate::result::ZK4LWResult;

const LINE_SEPARATOR: &'static str = "\n";
const KEY_VAL_EQUAL_SEPARATOR: &'static str = "=";
const KEY_VAL_TAB_SEPARATOR: &'static str = "\t";

fn bytes_to_key_value<'a>(input_utf8: &'a str, separator: &'static str) -> ZK4LWResult<HashMap<&'a str, &'a str>> {
    Ok(input_utf8
        .split_terminator(LINE_SEPARATOR)
        .map(|line| line.split_terminator(separator))
        .map(|mut key_val_seq| (key_val_seq.next(), key_val_seq.next()))
        .filter(|(k, v)| k.is_some() && v.is_some())                        //< Skip lines that don't split by given separator
        .map(|(k, v)| (k.unwrap().trim().into(), v.unwrap().trim().into()))
        .collect::<HashMap<&str, &str>>())
}

/// Parses multi-line `&str` made of key/value strings separated by tab (`\t`), into a `HashMap`
pub fn tab_separated_bytes_to_key_value(input_utf8: &str) -> ZK4LWResult<HashMap<&str, &str>> {
    bytes_to_key_value(input_utf8, KEY_VAL_TAB_SEPARATOR)
}

/// Parses multi-line `&str` made of key/value strings separated by equal (`=`), into a `HashMap`
pub fn equal_separated_bytes_to_key_value(input_utf8: &str) -> ZK4LWResult<HashMap<&str, &str>> {
    bytes_to_key_value(input_utf8, KEY_VAL_EQUAL_SEPARATOR)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::parsing::{
        equal_separated_bytes_to_key_value,
        tab_separated_bytes_to_key_value,
    };

    #[test]
    fn should_parse_tab_separated_bytes_to_key_value() {
        // Parse 'mntr' responses
        let mntr_3_4_resp = fs::read_to_string("../../fixtures/3.4/mntr.response").unwrap();
        let mntr_3_4_map = tab_separated_bytes_to_key_value(&mntr_3_4_resp).unwrap();
        assert_eq!(22, mntr_3_4_map.len());

        let mntr_3_5_resp = fs::read_to_string("../../fixtures/3.5/mntr.response").unwrap();
        let mntr_3_5_map = tab_separated_bytes_to_key_value(&mntr_3_5_resp).unwrap();
        assert_eq!(21, mntr_3_5_map.len());

        let mntr_3_6_resp = fs::read_to_string("../../fixtures/3.6/mntr.response").unwrap();
        let mntr_3_6_map = tab_separated_bytes_to_key_value(&mntr_3_6_resp).unwrap();
        assert_eq!(488, mntr_3_6_map.len());
    }

    #[test]
    fn should_parse_equal_separated_bytes_to_key_value() {
        // Parse 'conf' responses
        let conf_3_4_resp = fs::read_to_string("../../fixtures/3.4/conf.response").unwrap();
        let conf_3_4_map = equal_separated_bytes_to_key_value(&conf_3_4_resp).unwrap();
        assert_eq!(14, conf_3_4_map.len());

        let conf_3_5_resp = fs::read_to_string("../../fixtures/3.5/conf.response").unwrap();
        let conf_3_5_map = equal_separated_bytes_to_key_value(&conf_3_5_resp).unwrap();
        assert_eq!(23, conf_3_5_map.len());

        let conf_3_6_resp = fs::read_to_string("../../fixtures/3.6/conf.response").unwrap();
        let conf_3_6_map = equal_separated_bytes_to_key_value(&conf_3_6_resp).unwrap();
        assert_eq!(24, conf_3_6_map.len());

        // Parse 'envi' responses
        let envi_3_4_resp = fs::read_to_string("../../fixtures/3.4/envi.response").unwrap();
        let envi_3_4_map = equal_separated_bytes_to_key_value(&envi_3_4_resp).unwrap();
        assert_eq!(15, envi_3_4_map.len());

        let envi_3_5_resp = fs::read_to_string("../../fixtures/3.5/envi.response").unwrap();
        let envi_3_5_map = equal_separated_bytes_to_key_value(&envi_3_5_resp).unwrap();
        assert_eq!(18, envi_3_5_map.len());

        let envi_3_6_resp = fs::read_to_string("../../fixtures/3.6/envi.response").unwrap();
        let envi_3_6_map = equal_separated_bytes_to_key_value(&envi_3_6_resp).unwrap();
        assert_eq!(18, envi_3_6_map.len());
    }
}