// Global ValueParser equivalent to PHP Utils\ValueParser
// Formats numbers according to `control` and optional `preci` using
// thousands separators `.` and decimal separator `,`.

pub fn parse_value(control: u32, valor: &str, preci: Option<u32>) -> String {
    match control {
        16 => parse_number_integer_with_suffix(valor),
        17 => parse_number_decimal(valor, preci.unwrap_or(0) as usize),
        18 => parse_percentage(valor),
        19 => parse_number_integer(valor),
        _ => valor.to_string(),
    }
}

fn parse_number_integer_with_suffix(valor: &str) -> String {
    if let Some(pos) = valor.find('(') {
        let (num_part, suffix) = valor.split_at(pos);
        let formatted = format_number(num_part.trim().parse::<f64>().unwrap_or(0.0), 0);
        format!("{}{}", formatted, suffix)
    } else {
        parse_number_integer(valor)
    }
}

fn parse_number_integer(valor: &str) -> String {
    let n = valor.trim().parse::<f64>().unwrap_or(0.0);
    format_number(n.round(), 0)
}

fn parse_number_decimal(valor: &str, precision: usize) -> String {
    if let Ok(n) = valor.trim().parse::<f64>() {
        // round to precision
        let factor = 10f64.powi(precision as i32);
        let rounded = (n * factor).round() / factor;
        format_number(rounded, precision)
    } else {
        "ND".to_string()
    }
}

fn parse_percentage(valor: &str) -> String {
    let n = valor.trim().parse::<f64>().unwrap_or(0.0) * 100.0;
    if (valor.trim().parse::<f64>().unwrap_or(0.0)) <= 0.05 {
        let s = format_number((n * 10.0).round() / 10.0, 1);
        format!("{}%", s)
    } else {
        // PHP rounds to 1 then shows 0 decimals
        let rounded = (n * 10.0).round() / 10.0;
        let s = format_number(rounded, 0);
        format!("{}%", s)
    }
}

fn format_number(n: f64, decimals: usize) -> String {
    // Build base string with '.' thousands separator and ',' decimal separator.
    let s = if decimals == 0 {
        format!("{:.0}", n)
    } else {
        format!("{:.*}", decimals, n)
    };
    // split integer and fractional parts
    let mut parts = s.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();
    let int_with_sep = add_thousands_sep(int_part);
    if let Some(frac) = frac_part {
        format!("{},{}", int_with_sep, frac)
    } else {
        int_with_sep
    }
}

fn add_thousands_sep(int_part: &str) -> String {
    let mut chars: Vec<char> = int_part.chars().collect();
    let mut out = String::new();
    let mut count = 0;
    for ch in chars.iter().rev() {
        if count != 0 && count % 3 == 0 {
            out.push('.');
        }
        out.push(*ch);
        count += 1;
    }
    out.chars().rev().collect()
}
