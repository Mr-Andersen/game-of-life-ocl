pub const TABLE_HEIGHT: usize = 360;
pub const TABLE_WIDTH: usize = 720;

pub type TableLin = [u32; TABLE_WIDTH * TABLE_HEIGHT];
pub type TableMat = [[u32; TABLE_WIDTH]; TABLE_HEIGHT];

pub const CELL_WIDTH: usize = 1;
pub const CELL_HEIGHT: usize = 1;

pub const WIN_WIDTH: usize = CELL_WIDTH * TABLE_WIDTH;
pub const WIN_HEIGHT: usize = CELL_HEIGHT * TABLE_HEIGHT;
