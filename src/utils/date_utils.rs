use astrolabe::DateTime;

pub fn date_to_local_string(in_date: String, lang: &str) -> String {
    let the_date = DateTime::parse(in_date.as_str(), "yyyy-MM-dd").unwrap();
    let (year, month, day) = the_date.as_ymd();

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

        format!("{} {} {}", day, months[(month - 1) as usize], year)
    }
}
