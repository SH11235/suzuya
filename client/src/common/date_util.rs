use regex::Regex;

pub fn date_time_with_timezone_to_string(date_time_with_timezone: &Option<String>) -> String {
    match date_time_with_timezone {
        Some(date_time_with_timezone) => {
            // yyyy-mm-ddThh:mm:ss+00:00 の形式であるか正規表現でチェックする
            let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}$").unwrap();
            if re.is_match(&date_time_with_timezone) {
                let date_time_with_timezone = date_time_with_timezone.replace("T", " ");
                let date_time_with_timezone = date_time_with_timezone.replace("+00:00", "");
                let date_time_with_timezone = date_time_with_timezone.replace("-", "/");
                let date_time_with_timezone = date_time_with_timezone.split(" ").collect::<Vec<&str>>();
                let date_time_with_timezone = date_time_with_timezone[0];
                date_time_with_timezone.to_string()
            } else {
                "".to_string()
            }
            
        }
        None => "".to_string(),
    }
}

pub fn parse_date(date: &Option<String>) -> String {
    match date {
        Some(date) => (&date[0..10]).to_string(),
        None => "".to_string(),
    }
}

pub fn date_string_to_iso_string(date_string: Option<String>) -> Option<String> {
    match date_string {
        Some(date_string) => {
            let year = date_string[0..4].parse::<u32>().unwrap();
            // 月は0から始まるので1を引く
            let month = date_string[5..7].parse::<i32>().unwrap() - 1;
            let day = date_string[8..10].parse::<i32>().unwrap();
            Some(js_sys::Date::new_with_year_month_day(year, month, day).to_iso_string().into())
        },
        None => None,
    }
}

// test
#[cfg(test)]
mod tests {
    use super::date_time_with_timezone_to_string;

    #[test]
    fn test_date_time_with_timezone_to_string() {
        let date_time_with_timezone = "2022-03-03T00:00:00+00:00".to_string();
        let date_time_with_timezone = date_time_with_timezone_to_string(&Some(date_time_with_timezone));
        assert_eq!(date_time_with_timezone, "2022/03/03");

        // yyyy-mm-ddThh:mm:ss+00:00 以外の形式の場合は空文字列を返す
        let date_time_with_timezone = "2022-03-03T00:00:00".to_string();
        let date_time_with_timezone = date_time_with_timezone_to_string(&Some(date_time_with_timezone));
        assert_eq!(date_time_with_timezone, "");
    }
}
