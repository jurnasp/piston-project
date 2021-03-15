pub mod color {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const GREY: [f32; 4] = [0.25, 0.25, 0.25, 0.0];
    pub const DEBUG: [f32; 4] = [1.0, 0.0, 0.0, 0.5];
}

pub mod window {
    pub const SIZE: (u32, u32) = (1024, 512);
}

pub mod player {
    pub const SIZE: f64 = 20.0;
    pub const SPEED: f64 = 250.0;
}

pub mod chaser {
    pub const SIZE: f64 = 30.0;
    pub const SPEED: f64 = 225.0;
}
