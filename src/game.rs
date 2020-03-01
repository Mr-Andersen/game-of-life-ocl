use ocl::{Buffer, Device, DeviceType, Kernel, Platform, ProQue, Program};

use crate::{consts::*, Table};

pub struct Game {
    buffers: [Buffer<u32>; 2],
    cur_buf: u32,
    _pro_que: ProQue,
    next_it: Kernel,
}

impl Game {
    pub fn new(
        init_table: impl IntoIterator<Item = impl IntoIterator<Item = u32>>,
    ) -> ocl::Result<Self> {
        let pro_que = ProQue::builder()
            .device(
                Device::list(
                    Platform::default(),
                    Some(DeviceType::new().gpu()),
                )?
                .get(0)
                .expect("Get at least 1 GPU"),
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
        let prev = Table::new(init_table);
        let prev_buffer = pro_que
            .buffer_builder()
            .copy_host_slice(&*prev)
            .len(TABLE_HEIGHT * TABLE_WIDTH)
            .build()?;
        let next_buffer = pro_que
            .buffer_builder::<u32>()
            .len(TABLE_HEIGHT * TABLE_WIDTH)
            .build()?;
        let kernel = pro_que
            .kernel_builder("next_iteration")
            .arg_named("prev_buffer", None::<&Buffer<u32>>)
            .arg_named("next_buffer", None::<&Buffer<u32>>)
            .build()?;
        Ok(Game {
            buffers: [prev_buffer, next_buffer],
            cur_buf: 0,
            _pro_que: pro_que,
            next_it: kernel,
        })
    }

    pub fn next<'a>(&'a mut self) -> ocl::Result<&'a Buffer<u32>> {
        // index of buffer holding current game state
        let cur_idx = self.cur_buf & 1;
        self.cur_buf = !self.cur_buf;
        // index of buffer where next state will be written
        let nxt_idx = self.cur_buf & 1;
        self.next_it
            .set_arg("prev_buffer", &self.buffers[cur_idx as usize])?;
        self.next_it
            .set_arg("next_buffer", &self.buffers[nxt_idx as usize])?;
        unsafe {
            self.next_it.enq()?;
        }
        Ok(&self.buffers[nxt_idx as usize])
    }

    pub fn buffer<'a>(&'a mut self) -> &'a mut Buffer<u32> {
        &mut self.buffers[self.cur_buf as usize & 1]
    }
}
