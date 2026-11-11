use chrono::{Datelike, NaiveDate};

pub fn date_to_local_string(in_date: &str, lang: &str, date_fmt: &str) -> String {
    let date = NaiveDate::parse_from_str(in_date, date_fmt);

    match date {
        Ok(value) => {
            let (year, month, day) = (value.year(), value.month(), value.day());
            if lang == "es" {
                let months = [
                    "Enero",
                    "Febrero",
                    "Marzo",
                    "Abril",
                    "Mayo",
                    "Junio",
                    "Julio",
                    "Agosto",
                    "Septiembre",
                    "Octubre",
                    "Noviembre",
                    "Diciembre",
                ];

                format!("{} de {} de {}", day, months[(month - 1) as usize], year)
            } else {
                let months = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];

                format!("{} {} {}", months[(month - 1) as usize], day, year)
            }
        }
        Err(_error) => {
            eprintln!("ocurrio un error al parsear la fecha");
            in_date.into()
        }
    }
}
