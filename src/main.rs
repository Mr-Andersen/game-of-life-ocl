use ocl::{Device, DeviceType, Platform, ProQue, Program};

const TABLE_HEIGHT: usize = 5;
const TABLE_WIDTH: usize = 5;

const CELL_WIDTH: usize = 32;
const CELL_HEIGHT: usize = 32;

struct Table<'a> {
    inner: [u8; (TABLE_HEIGHT * TABLE_WIDTH) as usize],
    _mark: std::marker::PhantomData<&'a u8>,
}

struct TableIter<'a> {
    table: &'a Table<'a>,
    row: usize,
}

impl<'a> Table<'a> {
    fn new(inner: [u8; TABLE_HEIGHT * TABLE_WIDTH]) -> Self {
        Table {
            inner,
            _mark: std::marker::PhantomData,
        }
    }
    fn iter(&'a self) -> TableIter<'a> {
        TableIter {
            table: self,
            row: 0,
        }
    }
}

impl<'a> std::ops::Deref for Table<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.inner
    }
}

impl<'a> std::ops::DerefMut for Table<'a> {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= TABLE_HEIGHT {
            return None;
        }
        self.row += 1;
        Some(
            &self.table.inner
                [(self.row - 1) * TABLE_WIDTH..self.row * TABLE_WIDTH],
        )
    }
}

fn main() -> ocl::Result<()> {
    let pro_que = ProQue::builder()
        .device(
            Device::list(Platform::default(), Some(DeviceType::new().gpu()))
                .unwrap()
                .get(0)
                .unwrap(),
        )
        .prog_bldr({
            let mut bldr = Program::builder();
            bldr.src_file("src/kernel.cl")
                .cmplr_def("WIDTH", TABLE_WIDTH as i32)
                .cmplr_def("HEIGHT", TABLE_HEIGHT as i32);
            bldr
        })
        .dims((TABLE_HEIGHT, TABLE_WIDTH))
        .build()?;
    let prev = Table::new([
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0,
    ]);
    let prev_buffer = pro_que
        .buffer_builder()
        .copy_host_slice(&prev)
        .len(TABLE_HEIGHT * TABLE_WIDTH)
        .build()?;
    let next_buffer = pro_que
        .buffer_builder::<u8>()
        .len(TABLE_HEIGHT * TABLE_WIDTH)
        .build()?;
    let kernel = pro_que
        .kernel_builder("next_iteration")
        .arg(&prev_buffer)
        .arg(&next_buffer)
        .build()?;
    unsafe {
        kernel.enq()?;
    }
    let mut next = Table::new([0u8; TABLE_WIDTH * TABLE_HEIGHT]);
    next_buffer.read(&mut next as &mut [u8]).enq()?;
    for row in next.iter() {
        println!("{:?}", row);
    }
    Ok(())
}
