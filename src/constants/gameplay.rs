pub const PLAYER_DEFAULT_ANGLE: f32 = 0.0;
pub const PLAYER_DEFAULT_HEALTH: u8 = 100;
#[cfg(test)]
pub const PLAYER_MAX_HEALTH: u8 = 100;
#[cfg(test)]
pub const PLAYER_COLLISION_RADIUS: f32 = 8.0;
pub const MAP_CENTER_DIVISOR: f32 = 2.0;
pub const MAP_BOUNDS_MIN_SPAN: f32 = 1.0;
pub const INPUT_POLL_MILLIS: u64 = 16;
pub const INPUT_HELD_ACTION_STALE_TIMEOUT_MILLIS: u64 = 150;
#[cfg(test)]
pub const PICKUP_HEALTH_RADIUS: f32 = 18.0;
#[cfg(test)]
pub const PICKUP_HEALTH_AMOUNT: u8 = 15;

#[cfg(test)]
pub const THING_TYPE_STIMPACK: u16 = 2011;
#[cfg(test)]
pub const THING_TYPE_MEDIKIT: u16 = 2012;
#[cfg(test)]
pub const THING_TYPE_SOULSPHERE: u16 = 2013;
#[cfg(test)]
pub const THING_TYPE_BARREL: u16 = 2035;
