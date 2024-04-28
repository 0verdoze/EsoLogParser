
mod guarded_reader;
mod unguarded_reader;

pub use guarded_reader::*;
pub use unguarded_reader::*;

pub trait EsoReaderTrait<'a> {
    type LineReader: EsoLogLineReaderTrait<'a>;
    type SplitReader: EsoLogReaderTrait<'a>;

    fn read_line(s: &'a str) -> Self::LineReader;

    fn new_split(s: &'a str, c: char) -> Self::SplitReader;

    #[inline(always)]
    fn read_vec(s: &'a str) -> Self::LineReader {
        assert!(s.starts_with("[") && s.ends_with("]"), "{}", s);

        Self::read_line(&s[1..s.len() - 1])
    }
}

pub trait EsoLogReaderTrait<'a>: Clone + Iterator<Item = &'a str> {
    fn is_depleted(&self) -> bool;
    fn inner(&self) -> &'a str;
}

pub trait EsoLogLineReaderTrait<'a>: EsoLogReaderTrait<'a> {
    /// advance internal buffer by `x` bytes 
    /// 
    /// # SAFETY
    /// 
    /// caller must ensure that `i <= this.inner().as_bytes().len()`
    /// and that resulting slice will remain a valid UTF-8 string
    unsafe fn advance(&mut self, i: usize);
}
