

pub trait Shot {
    fn getEm(&self) -> u32;
    fn getExp(&self) -> u32;
    fn getKin(&self) -> u32;
    fn getThe(&self) -> u32;
}

pub trait Resistance {
    fn getEmResistance(&self) -> f32;
    fn getExpResistance(&self) -> f32;
    fn getKinResistance(&self) -> f32;
    fn getTheResistance(&self) -> f32;
}

pub trait HitPoints {
    fn getMaxHitPoints(&self) -> u32;
    fn getHitPoints(&self) -> u32;
    fn getPassiveRepair(&self) -> f32;
    fn getActiveRepair(&self) -> f32;
}


fn dealDamage(target: &HitPoints, shot: &Shot) ->


#[derive(Debug)]
pub struct Defense {
    hit_points: u32,
    em_resistance: f32,
    exp_resistance: f32,
    kin_resistance: f32,
    the_resistance: f32,
    passive_restore: f32,
    active_restore: f32,
}
impl Resistance for Defense {
    fn getEmResistance(&self) -> f32 { self.em_resistance }
    fn getExpResistance(&self) -> f32 { self.exp_resistance }
    fn getKinResistance(&self) -> f32  { self.kin_resistance }
    fn getTheResistance(&self) -> f32  { self.the_resistance }
}


fn main() {
    println!("Hello, world!");
}
