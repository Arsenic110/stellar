

///Returns the Moon mass in kilo times n.
pub fn moons(n: f64) -> f64 {
    n * 7.346e22
}

///Returns the Earth mass in kilo times n.
pub fn earths(n: f64) -> f64 {
    n * 5.972e24
}

///Returns Jupiter mass in kilo times n.
pub fn jupiters(n: f64) -> f64 {
    n * 1.8982e27
}

///Returns Sol mass in kilo times n.
pub fn sols(n: f64) -> f64 {
    n * 1.9885e30
}

///Returns the mass in moon masses times n.
pub fn to_moon(n: f64) -> f64 {
    n / 7.346e22
}

///Returns the mass in earth masses times n.
pub fn to_earth(n: f64) -> f64 {
    n / 5.972e24
}

///Returns mass in jupiter masses times n.
pub fn to_jupiter(n: f64) -> f64 {
    n / 1.8982e27
}

///Returns mass in solar masses times n.
pub fn to_solar(n: f64) -> f64 {
    n / 1.9885e30
}