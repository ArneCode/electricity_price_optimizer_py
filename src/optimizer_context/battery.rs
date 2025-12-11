pub struct Battery {
    capacity: i32,
    initial_level: i32,
    maximum_charge_rate: i32,
    maximum_output_rate: i32,
    efficiency: f32,
    id: i32,
}

impl Battery {
    pub fn new(
        capacity: i32,
        initial_level: i32,
        maximum_charge_rate: i32,
        maximum_output_rate: i32,
        efficiency: f32,
        id: i32,
    ) -> Self {
        assert!(
            initial_level <= capacity,
            "Initial battery level cannot exceed capacity"
        );
        Self {
            capacity,
            initial_level,
            maximum_charge_rate,
            maximum_output_rate,
            efficiency,
            id,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_max_charge(&self) -> i32 {
        return self.maximum_charge_rate;
    }
    pub fn get_max_output(&self) -> i32 {
        return self.maximum_output_rate;
    }
    pub fn get_capacity(&self) -> i32 {
        return self.capacity;
    }
    pub fn get_initial_level(&self) -> i32 {
        return self.initial_level;
    }
}
