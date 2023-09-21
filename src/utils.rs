// use time::macros::format_description;
// use time::Date;
//
// pub fn date_to_string(date: &Date) -> String {
//     let format = format_description!("[year]-[month]-[day]");
//     format!("{}", date.format(&format).unwrap())
// }

use std::collections::HashMap;

pub fn params_to_query_string(params: &HashMap<String, String>) -> String {
    let mut query = String::new();
    for (key, value) in params {
        if !query.is_empty() {
            query.push('&')
        }
        query.push_str(&key);
        query.push('=');
        query.push_str(&value);
    }
    query
}
