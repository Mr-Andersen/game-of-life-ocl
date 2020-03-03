pub const TABLE_HEIGHT: usize = 700;
pub const TABLE_WIDTH: usize = 700;

pub type TableLin = [u32; TABLE_WIDTH * TABLE_HEIGHT];
// pub type TableMat = [[u32; TABLE_WIDTH]; TABLE_HEIGHT];

pub const ALIVE: u32 = 0x00ffffff;
pub const DEAD: u32 = 0x00000000;
