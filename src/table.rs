use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use crate::consts::*;

pub struct Table {
    inner: TableLin,
}

pub struct TableIter<'a> {
    table: &'a Table,
    row: usize,
}

impl Table {
    pub fn new(
        val: impl IntoIterator<Item = impl IntoIterator<Item = u32>>,
    ) -> Self {
        let mut mat: TableMat = [[0u32; TABLE_WIDTH]; TABLE_HEIGHT];
        for (i, row) in val.into_iter().take(TABLE_HEIGHT).enumerate() {
            for (j, elem) in row.into_iter().take(TABLE_WIDTH).enumerate() {
                mat[i][j] = elem;
            }
        }
        let inner: &TableLin =
            unsafe { (&mat as *const TableMat as *const TableLin).as_ref() }
                .unwrap();
        Table { inner: *inner }
    }
    pub fn iter<'a>(&'a self) -> TableIter<'a> {
        TableIter {
            table: self,
            row: 0,
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        use std::iter::once;

        Table::new(once(once(0x0).cycle()).cycle())
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.iter() {
            for elem in row {
                match *elem {
                    0x00ffffff => write!(f, "#")?,
                    0 => write!(f, ".")?,
                    x => eprintln!("{}", x),
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Deref for Table {
    type Target = TableLin;

    fn deref<'a>(&'a self) -> &'a TableLin {
        &self.inner
    }
}

impl DerefMut for Table {
    fn deref_mut<'a>(&'a mut self) -> &'a mut TableLin {
        &mut self.inner
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = &'a [u32];

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
