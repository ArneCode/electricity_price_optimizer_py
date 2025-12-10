const MINUTES_PER_DAY: usize = 24 * 60;
pub struct ElectricityPrognoses {
    data: [i32; MINUTES_PER_DAY],
}

pub struct PricePrognoses {
    data: [i32; MINUTES_PER_DAY],
}
