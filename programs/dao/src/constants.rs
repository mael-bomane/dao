pub const DISCRIMINATOR_LENGTH: usize = 8;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const TIMESTAMP_LENGTH: usize = 8;
pub const STRING_LENGTH_PREFIX: usize = 4;
pub const VECTOR_LENGTH_PREFIX: usize = 4;
pub const BUMP_LENGTH: usize = 1;
pub const BOOL_LENGTH: usize = 1;
pub const MAX_DAO_NAME_LENGTH: usize = 50 * 4; // 50 chars max.
pub const MAX_TITLE_LENGTH: usize = 50 * 4; // 50 chars max.
pub const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.
pub const ONE_DAY_IN_SECONDS: i64 = 86400;
pub const TWO_DAY_IN_SECONDS: i64 = ONE_DAY_IN_SECONDS * 2;
pub const ONE_WEEK_IN_SECONDS: i64 = ONE_DAY_IN_SECONDS * 7;
pub const ONE_MONTH_IN_SECONDS: i64 = ONE_DAY_IN_SECONDS * 30;
