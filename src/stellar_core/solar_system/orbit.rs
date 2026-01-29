use std::f64::consts::PI;
use bevy::prelude::*;
pub const G: f64 = 6.6743015e-11;

//N.B.: This module uses SI units.
//Make sure that distances should be in meters, masses in kilograms, and periods in seconds.

#[derive(Clone)]
pub struct Orbit {
    parent: Entity,
    pub apoapsis: f64,
    pub periapsis: f64,
    pub offset: f64,
}

impl Default for Orbit {
    fn default() -> Self {
        Orbit { parent: Entity::PLACEHOLDER, apoapsis: 0.0, periapsis: 0.0, offset: 0.0 }
    }
}

impl std::fmt::Debug for Orbit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "A{:.0} P{:.0} E{:.2}",
                self.apoapsis,
                self.periapsis,
                self.eccentricity()
            )
            .as_str()
        )
    }
}

impl Orbit {

    pub fn new(apoapsis: f64, periapsis: f64, offset: f64) -> Orbit {
        Orbit { parent: Entity::PLACEHOLDER, apoapsis, periapsis, offset }
    }
    pub fn semi_major_axis(self: &Self) -> f64 {
        (self.apoapsis + self.periapsis) / 2.0
    }
    pub fn semi_minor_axis(self: &Self) -> f64 {
        (self.apoapsis * self.periapsis).sqrt()
    }
    pub fn semi_latus(self: &Self) -> f64 {
        ((self.apoapsis.powi(-1) + self.periapsis.powi(-1)) / 2.0).powi(-1)
    }
    pub fn eccentricity(self: &Self) -> f64 {
        (self.apoapsis - self.periapsis) / (self.apoapsis + self.periapsis)
    }
    pub fn period(self: &Self, mass: f64, parent_mass: f64) -> f64 {
        (self.semi_major_axis().powi(3) / (G * (mass + parent_mass))).sqrt() * 2.0 * PI
    }

    ///Average angular velocity (rad/s).
    pub fn mean_motion(&self, mass: f64, parent_mass: f64) -> f64 {
        (G * (mass + parent_mass) / self.semi_major_axis().powi(3)).sqrt()
    }

    //Solve Kepler's Equation with Newton-Raphson method
    fn eccentric_anomaly(&self, mean_anomaly: f64, e: f64) -> f64 {
        let mut e_anom = mean_anomaly;  //initial guess
        for _ in 0..10 {    //usually converges in <5 iterations
            let f = e_anom - e * e_anom.sin() - mean_anomaly;
            let f_prime = 1.0 - e * e_anom.cos();
            e_anom -= f / f_prime;
        }
        e_anom
    }

    ///Position at time t (seconds) relative to parent, in meters.
    pub fn position_at_time(&self, t: f64, mass: f64, parent_mass: f64) -> (f64, f64) {
        let a = self.semi_major_axis();
        let e = self.eccentricity();
        let b = a * (1.0 - e * e).sqrt();

        //get angular velocity
        let n = self.mean_motion(mass, parent_mass);
        let m_anom = n * t + self.offset;

        //solve for eccentric anomaly
        let e_anom = self.eccentric_anomaly(m_anom, e);

        //convert to (x, y) in orbital plane
        let x = a * (e_anom.cos() - e);
        let y = b * e_anom.sin();

        (x, y)
    }
}