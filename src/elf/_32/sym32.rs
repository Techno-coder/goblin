pub use elf::sym::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "endian_fd", derive(Pread, Pwrite))]
pub struct Sym {
    /// Symbol name (string tbl index)
    pub st_name: u32,
    /// Symbol value
    pub st_value: u32,
    /// Symbol size
    pub st_size: u32,
    /// Symbol type and binding
    pub st_info: u8,
    /// Symbol visibility
    pub st_other: u8,
    /// Section index
    pub st_shndx: u16,
}

pub const SIZEOF_SYM: usize = 4 + 1 + 1 + 2 + 4 + 4;

elf_sym_impure_impl!(
    pub fn parse<S: scroll::Gread>(fd: &S, offset: usize, count: usize, little_endian: scroll::Endian) -> Result<Vec<Sym>> {
        let mut syms = Vec::with_capacity(count);
        let mut offset = offset;
        let mut offset = &mut offset;
        for _ in 0..count {
            let mut sym = Sym::default();
            sym.st_name =  fd.gread(offset, little_endian)?;
            sym.st_value = fd.gread(offset, little_endian)?;
            sym.st_size =  fd.gread(offset, little_endian)?;
            sym.st_info =  fd.gread_into(offset)?;
            sym.st_other = fd.gread_into(offset)?;
            sym.st_shndx = fd.gread(offset, little_endian)?;
            syms.push(sym);
        }
        Ok(syms)
    });
