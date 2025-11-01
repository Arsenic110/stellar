use crate::stellar_core::solar_system::Orbit;
use crate::stellar_core::solar_system::Star;

pub fn generate_star(solar_mass: f64, age_gy: f64, metallicity: f64) -> Star
{
    let lifespan = 10.0 / solar_mass.powf(2.5);

    //stars are main sequence for 90% of their lifetime. last 10% is just red giant
    //then beyond that its remnants, and before, it'll be a protostar.
    let (radius, luminosity, temperature, lifespan, spectral_type) = 
    match age_gy / lifespan {
        x if x < 0.0 => get_protostar_properties(solar_mass, age_gy, metallicity),
        x if (0.0..0.9).contains(&x) => get_main_sequence_properties(solar_mass, age_gy, metallicity),
        x if (0.9..1.0).contains(&x) => get_giant_properties(solar_mass, age_gy, metallicity),
        x if x > 1.0 => get_remnant_properties(solar_mass, age_gy, metallicity),
        _ => get_main_sequence_properties(solar_mass, age_gy, metallicity) //lol idk, fallback?
    };

    Star { mass: solar_mass, radius, luminosity, temperature, lifespan, spectral_type, orbit: Orbit::default() }
}

fn get_protostar_properties(mass: f64, _age_gy: f64, metallicity: f64) -> (f64, f64, f64, f64, String) {
    //mass in solar masses. age in GY. metallicity in solar units

    let radius = mass.powf(0.95);

    let lifespan_modifier = 1.0 - (metallicity - 1.0) * 0.1;
    let lifespan = 10000000000.0 / mass.powf(2.5) * lifespan_modifier;

    //in solar units
    let luminosity_modifier = 0.8 - (metallicity - 1.0) * 0.1;
    let luminosity = match mass {
        x if (0.0..0.43).contains(&x) => 0.23 * mass.powf(2.3),
        x if (0.43..2.0).contains(&x) => mass.powf(4.0),
        x if (2.0..20.0).contains(&x) => 1.5 * mass.powf(3.5),
        x if x >  20.0 => 3200.0 * mass,
        _ => 0.0
    } * luminosity_modifier;

    //in kelvin
    let temperature_modifier = 0.8 - (metallicity - 1.0) * 0.05;
    let temperature = (luminosity / radius.powi(2)).powf(0.25) * 5772.0 * temperature_modifier;

    //this should be overhauled as type should not depend solely on temperature
    let spectral_type: String = match temperature {
        x if (10000..40000).contains(&(x as i32)) && mass > 1.8 => "HAeBe".to_string(),
        x if (0..10000).contains(&(x as i32)) => "TT".to_string(),
        _ => ".".to_string()
    };

    return (radius, luminosity, temperature, lifespan, spectral_type);
}

fn get_main_sequence_properties(mass: f64, age_gy: f64, metallicity: f64) -> (f64, f64, f64, f64, String) {
    //mass in solar masses. age in GY. metallicity in solar units

    let radius = mass.powf(0.8);

    let lifespan_modifier = 1.0 - (metallicity - 1.0) * 0.1;
    let lifespan = 10000000000.0 / mass.powf(2.5) * lifespan_modifier;

    //in solar units
    let luminosity_modifier = 1.0 - (metallicity - 1.0) * 0.1 + (age_gy - 5.0) * 0.01;
    let luminosity = match mass {
        x if (0.0..0.43).contains(&x) => 0.23 * mass.powf(2.3),
        x if (0.43..2.0).contains(&x) => mass.powf(4.0),
        x if (2.0..20.0).contains(&x) => 1.5 * mass.powf(3.5),
        x if x >  20.0 => 3200.0 * mass,
        _ => 0.0
    } * luminosity_modifier;

    //in kelvin
    let temperature_modifier = 1.0 - (metallicity - 1.0) * 0.05 + (age_gy - 5.0) * 0.005;
    let temperature = (luminosity / radius.powi(2)).powf(0.25) * 5772.0 * temperature_modifier;

    let spectral_type: char = match temperature {
        x if x > 30000.0 => 'O',
        x if (10000..30000).contains(&(x as i32)) => 'B',
        x if (7500..10000).contains(&(x as i32)) => 'A',
        x if (6000..7500).contains(&(x as i32)) => 'F',
        x if (5200..6000).contains(&(x as i32)) => 'G',
        x if (3700..5200).contains(&(x as i32)) => 'K',
        x if (2100..3700).contains(&(x as i32)) => 'M',
        x if (1300..2100).contains(&(x as i32)) => 'L',
        x if (600..1300).contains(&(x as i32)) => 'T',
        x if (0..600).contains(&(x as i32)) => 'Y',
        _ => '.'
    };

    return (radius, luminosity, temperature, lifespan, spectral_type.to_string());
}

fn get_giant_properties(mass: f64, age_gy: f64, metallicity: f64) -> (f64, f64, f64, f64, String) {
    
    let radius_base = 10.0 * (mass / 1.0).powf(0.3); // Base radius, scaled with mass
    let radius = radius_base * (1.0 + 0.5 * (age_gy / 10.0).powf(0.5)); // Age increases radius

    let luminosity_base = 100.0 * (mass / 1.0).powf(3.5); // Base luminosity
    let luminosity = luminosity_base * (1.0 + 0.7 * (age_gy / 10.0).powf(0.7)); // Age increases luminosity

    let temperature_base = 5000.0 - 100.0 * (mass / 1.0); // Base temperature, depends on mass
    let temperature = temperature_base - 50.0 * (age_gy / 10.0); // Age decreases temperature

    let lifespan_modifier = 1.0 - (metallicity - 1.0) * 0.1;
    let lifespan = 10000000000.0 / mass.powf(2.5) * lifespan_modifier;

    let spectral_type = match temperature {
        x if x > 7000.0 => 'O',
        x if (5000..6000).contains(&(x as i32)) => 'B',
        x if (4000..5000).contains(&(x as i32)) => 'A',
        x if (3500..4000).contains(&(x as i32)) => 'F',
        x if (2500..3500).contains(&(x as i32)) => 'G',
        x if (1000..2500).contains(&(x as i32)) => 'K',
        x if (0..1000).contains(&(x as i32)) => 'M',
        _ => '.'
    };

    return (radius, luminosity, temperature, lifespan, spectral_type.to_string());
}

fn get_remnant_properties(mass: f64, _age_gy: f64, _metallicity: f64) -> (f64, f64, f64, f64, String) {

    //love to see these 4 constants together, because it means funky shit goes down
    let const_g = 0.000000000066743;    //gravitational constant
    let const_c: f64 = 299792458.0;          //m/s - speed of light
    let _hbar = 1.0545718e-34;           // Reduced Planck constant (J s)
    let _k_b = 1.380649e-23;             // Boltzmann constant (J/K)
    let s_b = 5.670374e-8;              //Stefan-Boltzmann constant

    let remnant_type = match mass {
        x if x > 3.0 => "BH",                       //black hole
        x if (1.4..3.1).contains(&x) => "NS",  //neutron star
        _ => "WD"                                        //white dwarf
    };

    let radius = match remnant_type {
        "BH" => (2.0 * const_g * mass) / const_c.powi(2), //schwarzchild radius
        "NS" => 0.001 * mass, //overhaul needed - neutron stars shrink as mass increases.
        _ => 0.015 * mass
    };

    let temperature = match remnant_type {
        //hawking radiation!
        "BH" => 0.0,
        "NS" => 1000000.0 - 100000.0 * (mass / 1.0),
        _ => 10000.0 - 500.0 * (mass / 1.0)
    };

    let luminosity = match remnant_type {
        "BH" => 0.0, //it should glow at hawking temp but its wholly negligible
        _ => 4.0 * std::f64::consts::PI * radius.powi(2) * s_b * temperature.powi(4) / 3.38e7
    };

    let lifespan = std::f64::MAX; //lifespan becomes irrelevant here

    let spectral_type = match remnant_type {
        "BH" => 'X',    //black hole
        "NS" => 'N',    //neutron star
        _ => 'W'        //white dwarf
    };

    return (radius, luminosity, temperature, lifespan, spectral_type.to_string());

}