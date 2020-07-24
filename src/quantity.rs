#[path = "quantity/units.rs"] mod units;

pub enum Quantity {
    Mass(f64, units::MassUnit),
    Volume(f64, units::VolumeUnit),
}
