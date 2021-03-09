use std;
pub fn conversion_commander(conversion_type: String, from_unit: String, to_unit: String, units: f64) -> f64 {
    let conversion_type: &str = conversion_type.as_str();

    match conversion_type {
        "temperature" => {
            convert_temperature(&from_unit, &to_unit, units)
        },
        "distance" => {
            convert_length(&from_unit, &to_unit, units)
        },
        _ => {
            println!("Non supported Conversion");
            std::f64::NAN
        }
    }
}

// BEGIN LENGTHS

pub fn convert_length(from_unit: &str, to_unit: &str, units: f64) -> f64 {
    let mut converted: f64;
    converted = match from_unit {
        "AU" => { au_to_m(units) },
        "CM" => { cm_to_m(units) },
        "DM" => { dm_to_m(units) },
        "FT" => { ft_to_m(units) },
        "IN" => { in_to_m(units) },
        "KM" => { km_to_m(units) },
        "LY" => { ly_to_m(units) },
        "M" => { units },
        "MI" => { mi_to_m(units) },
        "MM" => { mm_to_m(units) },
        "NM" => { nm_to_m(units) },
        "PC" => { pc_to_m(units) },
        "YD" => { yd_to_m(units) },
        _ => {
            println!("invalid unit");
            std::f64::NAN
        }
    };

    let m = converted;
    converted = match to_unit {
        "AU" => { m_to_au(m) },
        "CM" => { m_to_cm(m) },
        "DM" => { m_to_dm(m) },
        "FT" => { m_to_ft(m) },
        "IN" => { m_to_in(m) },
        "KM" => { m_to_km(m) },
        "LY" => { m_to_ly(m) },
        "M" => { m },
        "MI" => { m_to_mi(m) },
        "MM" => { m_to_mm(m) },
        "NM" => { m_to_nm(m) },
        "PC" => { m_to_pc(m) },
        "YD" => { m_to_yd(m) },
        _ => {
            println!("invalid unit");
            std::f64::NAN
        }
    };

    // maybe add back??
    /* 
    match () {
        _ if converted < 0.0 => {
            println!("Negative Length");
            std::f64::NAN
        },
        _ => {
            converted
        }
    } */
    converted
}


//from metres
fn m_to_au(m: f64) -> f64 { m / 149_597_870_700. }
fn m_to_cm(m: f64) -> f64 { m * 100. }
fn m_to_dm(m: f64) -> f64 { m * 10. }
fn m_to_ft(m: f64) -> f64 { m * 3.2808 }
fn m_to_in(m: f64) -> f64 { 12. * m_to_ft(m) }
fn m_to_km(m: f64) -> f64 { m / 1_000.}
fn m_to_ly(m: f64) -> f64 { m / 9_460_730_777_119_564. }
fn m_to_mi(m: f64) -> f64 { m / 1_609.344 }
fn m_to_mm(m: f64) -> f64 { m * 1_000.}
fn m_to_nm(m: f64) -> f64 { m / 1_852.}
fn m_to_pc(m: f64) -> f64 { m / au_to_m(648_000. / std::f64::consts::PI) }
fn m_to_yd(m: f64) -> f64 { m / 0.9144 }
//end from metres

//to metres
fn au_to_m(n: f64) -> f64 { n * 149_597_870_700.}
fn cm_to_m(n: f64) -> f64 { n / 100. }
fn dm_to_m(n: f64) -> f64 { n / 10. }
fn ft_to_m(n: f64) -> f64 { n / 3.2808 }
fn in_to_m(n: f64) -> f64 { ft_to_m(n) / 12. }
fn km_to_m(n: f64) -> f64 { n * 1_000. }
fn ly_to_m(n: f64) -> f64 { n * 9_460_730_777_119_564. }
fn mi_to_m(n: f64) -> f64 { n * 1609.344 }
fn mm_to_m(n: f64) -> f64 { n / 1_000. }
fn nm_to_m(n: f64) -> f64 { n * 1_852. }
fn pc_to_m(n: f64) -> f64 { au_to_m(648_000. / std::f64::consts::PI) * n }
fn yd_to_m(n: f64) -> f64 { n * 0.9144 }
//end to metres

// END LENGTHS


// BEGIN TEMPERATURES
pub fn convert_temperature(from_unit: &str, to_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match to_unit {
        "Celsius" => {
            converted = to_celsius(from_unit, degrees);
        },
        "Delisle" => {
            converted = to_delisle(from_unit, degrees);
        },
        "Fahrenheit" => {
            converted = to_fahrenheit(from_unit, degrees);
        },
        "Kelvin" => {
            converted = to_kelvin(from_unit, degrees);
        },
        "Newton" => {
            converted = to_newton(from_unit, degrees);
        },
        "Rankine" => {
            converted = to_rankine(from_unit, degrees);
        },
        "Reaumer" => {
            converted = to_reaumer(from_unit, degrees);
        },
        "Romer" => {
            converted = to_romer(from_unit, degrees);
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    converted
}

fn to_celsius(from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees;
        },
        "Delisle" => {
            converted = 100.0 - (degrees * 2.0 / 3.0); 
        },
        "Fahrenheit" => {
            converted = (degrees - 32.0) * 5.0 / 9.0;
        },
        "Kelvin" => {
            converted = degrees - 273.15;
        },
        "Newton" => {
            converted = degrees * 100.0 / 33.0;
        },
        "Rankine" => {
            converted = (degrees - 491.67) * 5.0 / 9.0;
        },
        "Reaumur" => { //Réaumur
            converted = degrees * 5.0 / 4.0;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 40.0 / 21.0;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < -273.15 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_delisle (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = (100.0 - degrees) * 3.0 / 2.0;
        },
        "Delisle" => {
            converted = degrees;
        },
        "Fahrenheit" => {
            converted = (212.0 - degrees) * 5.0 / 6.0;
        },
        "Kelvin" => {
            converted = (373.15 - degrees) * 3.0 / 2.0;
        },
        "Newton" => {
            converted = (33.0 - degrees) * 55.0 / 11.0;
        },
        "Rankine" => {
            converted = (671.67 - degrees) * 5.0 / 6.0;
        },
        "Reaumer" => { //Réaumur
            converted = (80.0 - degrees) * 15.0 / 8.0;
        },
        "Romer" => { //Rømer
            converted = (60.0 - degrees) * 20.0 / 7.0;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted > 559.725 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_fahrenheit (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees * 9.0 / 5.0 + 32.0;
        },
        "Delisle" => {
            converted = 212.0 - (degrees * 6.0 / 5.0);
        },
        "Fahrenheit" => {
            converted = degrees;
        },
        "Kelvin" => {
            converted = degrees * 9.0 / 5.0 - 459.67;
        },
        "Newton" => {
            converted = degrees * 60.0 / 11.0 + 32.0;
        },
        "Rankine" => {
            converted = degrees - 459.67;
        },
        "Reaumer" => { //Réaumur
            converted = degrees * 9.0 / 4.0 + 32.0;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 24.0 / 7.0 + 32.0;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < -459.67 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_kelvin (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees + 273.15;
        },
        "Delisle" => {
            converted = 373.15 - (degrees * 2.0 / 3.0);
        },
        "Fahrenheit" => {
            converted = (degrees + 459.67) * 5.0 / 9.0;
        },
        "Kelvin" => {
            converted = degrees;
        },
        "Newton" => {
            converted = (degrees * 100.0 / 33.0) + 273.15;
        },
        "Rankine" => {
            converted = degrees * 5.0 / 9.0;
        },
        "Reaumer" => { //Réaumur
            converted = degrees * 5.0 / 4.0 + 273.15;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 40.0 / 21.0 + 273.15;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < 0.0 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_newton (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees * 33.0 / 100.0;
        },
        "Delisle" => {
            converted = 33.0 - (degrees * 11.0 / 50.0);
        },
        "Fahrenheit" => {
            converted = (degrees - 32.0) * 11.0 / 60.0;
        },
        "Kelvin" => {
            converted = (degrees - 273.15) * 33.0 / 100.0;
        },
        "Newton" => {
            converted = degrees;
        },
        "Rankine" => {
            converted = (degrees - 491.67) * 11.0 / 60.0;
        },
        "Reaumer" => { //Réaumur
            converted = degrees * 33.0 / 100.0;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 22.0 / 35.0;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < -90.1395 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_rankine(from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = (degrees + 273.15) * 9.0 / 5.0;
        },
        "Delisle" => {
            converted = 671.67 - (degrees * 6.0 /5.0);
        },
        "Fahrenheit" => {
            converted = degrees + 459.67;
        },
        "Kelvin" => {
            converted = degrees * 9.0 / 5.0;
        },
        "Newton" => {
            converted = (degrees * 60.0 / 11.0) + 491.67;
        },
        "Rankine" => {
            converted = degrees;
        },
        "Reaumer" => { //Réaumur
            converted = (degrees * 9.0 / 4.0) + 491.67;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 24.0 / 7.0 + 491.67;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < 0.0 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_reaumer (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees * 4.0 / 5.0;
        },
        "Delisle" => {
            converted = 80.0 - (degrees * 8.0 / 15.0);
        },
        "Fahrenheit" => {
            converted = (degrees - 32.0) * 4.0 / 9.0;
        },
        "Kelvin" => {
            converted = (degrees - 273.15) * 4.0 / 5.0;
        },
        "Newton" => {
            converted = degrees * 80.0 / 33.0;
        },
        "Rankine" => {
            converted = (degrees - 491.67) * 4.0 / 9.0;
        },
        "Reaumer" => { //Réaumur
            converted = degrees;
        },
        "Romer" => { //Rømer
            converted = (degrees - 7.5) * 32.0 / 21.0;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < -218.52 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

fn to_romer (from_unit: &str, degrees: f64) -> f64 {
    let mut converted: f64 = std::f64::NAN;
    match from_unit {
        "Celsius" => {
            converted = degrees * 21.0 / 40.0 + 7.5;
        },
        "Delisle" => {
            converted = 60.0 - (degrees * 7.0 / 20.0);
        },
        "Fahrenheit" => {
            converted = (degrees - 32.0) * 7.0 / 24.0 + 7.5;
        },
        "Kelvin" => {
            converted = (degrees - 273.15) * 21.0 / 40.0 + 7.5;
        },
        "Newton" => {
            converted = degrees * 35.0 / 22.0 + 7.5;
        },
        "Rankine" => {
            converted = (degrees - 491.67) * 7.0 / 24.0 + 7.5;
        },
        "Reaumer" => { //Réaumur
            converted = degrees * 21.0 / 32.0 + 7.5;
        },
        "Romer" => { //Rømer
            converted = degrees;
        },
        _ => {
            println!("Invalid Unit");
        },
    }
    if converted < -135.90375 {
        println!("Below absolute zero");
        converted = std::f64::NAN;
    }
    converted
}

//END TEMPERATURES