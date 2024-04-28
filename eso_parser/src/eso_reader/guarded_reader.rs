use std::simd::cmp::SimdPartialEq;
use super::{EsoLogReaderTrait, EsoLogLineReaderTrait, EsoReaderTrait};

pub struct EsoLogReader;

impl<'a> EsoReaderTrait<'a> for EsoLogReader {
    type LineReader = EsoLogReaderLine<'a>;
    type SplitReader = EsoLogReaderSplitChar<'a>;

    #[inline(always)]
    fn read_line(s: &'a str) -> Self::LineReader {
        EsoLogReaderLine(s.as_bytes())
    }

    #[inline(always)]
    fn new_split(s: &'a str, c: char) -> Self::SplitReader {
        let mut buf = [0; 1];
        c.encode_utf8(&mut buf);

        EsoLogReaderSplitChar(s.as_bytes(), buf[0])
    }
}

#[derive(Clone)]
pub struct EsoLogReaderSplitChar<'a>(&'a [u8], u8);

impl<'a> EsoLogReaderTrait<'a> for EsoLogReaderSplitChar<'a> {
    #[inline]
    fn is_depleted(&self) -> bool {
        self.0.is_empty()
    }

    fn inner(&self) -> &'a str {
        unsafe {
            std::str::from_utf8_unchecked(self.0)
        }
    }
}

impl<'a> Iterator for EsoLogReaderSplitChar<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // SAFETY: head (0) is always <= len()
        let end = unsafe { FastFind::find(0, self.0, self.1) };

        // cast pointers to slice
        // SAFETY: FastFind::find will always return value from range [0..=self.0.len()]
        let result = unsafe {
            std::slice::from_raw_parts(self.0.as_ptr(), end)
        };

        // override self.0 with data we just found skipped +1 (to consume character we just found)
        // SAFETY: `end`` is in range of [0..=self.0.len()], and we add 1 to it only if `end != self.0.len()`
        // this is faster than [(end + 1).min(self.0.len())..] as we skip unnecessary bound check
        self.0 = unsafe {
            let mut range = self.0.as_ptr_range();
            range.start = range.start.add((end + 1).min(self.0.len()));

            std::slice::from_ptr_range(range)
        };

        // if result is empty that means that self.0 is also empty, return None in that case
        // as iterator is exhausted
        if std::intrinsics::likely(!result.is_empty()) {
            // SAFETY: `result` is a valid UTF-8 string as above reasons
            unsafe {
                let slice = std::str::from_utf8_unchecked(result);
    
                Some(slice)
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct EsoLogReaderLine<'a>(&'a [u8]);

impl<'a> EsoLogReaderTrait<'a> for EsoLogReaderLine<'a> {
    #[inline]
    fn is_depleted(&self) -> bool {
        self.0.is_empty()
    }

    fn inner(&self) -> &'a str {
        unsafe {
            std::str::from_utf8_unchecked(self.0)
        }
    }
}

impl<'a> EsoLogLineReaderTrait<'a> for EsoLogReaderLine<'a> {
    /// advance internal buffer by `x` bytes 
    /// 
    /// # SAFETY
    /// 
    /// caller must ensure that `i <= this.inner().as_bytes().len()`
    /// and that resulting slice will remain a valid UTF-8 string
    unsafe fn advance(&mut self, i: usize) {
        let mut range = self.0.as_ptr_range();
        range.start = range.start.add(i);

        self.0 = std::slice::from_ptr_range(range);
    }
}

impl<'a> Iterator for EsoLogReaderLine<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.0;

        let end = match s.get(0)? {
            b'"' => unsafe {
                // SAFETY: head (1) is always <= s.len()
                SimdFind::find(1, s, b'"') + 1
            },
            b'[' => {
                let mut i = 0;
                let mut count = 0usize;

                // parse nested arrays
                let end = loop {
                    if std::intrinsics::unlikely(i == s.len()) { break i; }
                    // SAFETY: we just checked if `i` is < s.len()
                    let c = unsafe { *self.0.get_unchecked(i) };

                    // suggests compiler to calculate both values and use `CMOV`
                    count = if c == b'[' { count + 1 } else { count };
                    count = if c == b']' { count - 1 } else { count };
                    
                    i += 1;
                    if count == 0 { break i; }
                };

                end
            },
            _ => unsafe {
                // SAFETY: head (0) is always <= s.len()
                SimdFind::find(0, s, b',')
            }
        };

        // &s[..end];
        // SAFETY: SimdFind::find will return value <= s.len()
        let r = unsafe {
            let mut range = s.as_ptr_range();
            range.end = range.start.add(end);

            std::slice::from_ptr_range(range)
        };
        
        // `end` points at `,`, self.0 slice pointing one byte past it
        // satturating at buffers end
        // SAFETY: we are satturating at buffers end
        unsafe {
            let mut range = s.as_ptr_range();
            range.start = range.start.add((end + 1).min(s.len()));

            self.0 = std::slice::from_ptr_range(range);
        }
        
        if std::intrinsics::likely(!r.is_empty()) {
            // SAFETY: buffer is guranteed to be a valid UTF-8
            let slice = unsafe { std::str::from_utf8_unchecked(r) };

            Some(slice)
        } else {
            None
        }
    }
}

struct SimdFind;

impl SimdFind {
    // SAFETY: head <= s.len()
    unsafe fn find(mut head: usize, s: &[u8], f: u8) -> usize {
        type Simd = std::simd::Simd<u8, 8>;
    
        let mask = Simd::from_array([f; Simd::LEN]);
        while s.len() - head >= Simd::LEN {
            let mut range = s.as_ptr_range();
            // SAFETY: there is atleast `Simd::LEN` bytes past head that are valid to read
            unsafe {
                range.start = range.start.add(head);
                range.end = range.start.add(Simd::LEN);
            }
    
            // SAFETY: range length is equal to Simd::LEN
            let values = unsafe {
                Simd::from_array(std::slice::from_ptr_range(range).try_into().unwrap_unchecked())
            };

            // this type must be casted to type with same amount of bits as Simd::LEN
            // if its not we will skip some data (if its too big), or repeat our selves (if too small)
            let result = values.simd_eq(mask).to_bitmask() as u8;
    
            head += result.trailing_zeros() as usize;
    
            if result != 0 {
                return head;
            }
        }

        unsafe {
            // SAFETY: we have just checked for required conditions
            FastFind::find_rev_start(head, s, f)
        }
    }
}

struct FastFind; 

impl FastFind {
    /// # SAFETY
    /// Caller must ensure that `head <= s.len()`
    #[inline]
    unsafe fn find_rev_start(mut head: usize, s: &[u8], f: u8) -> usize {
        let old_head = head;
        head = s.len();
    
        for i in (old_head..s.len()).rev() {
            if *s.get_unchecked(i) == f {
                head = i;
            }
        }

        head
    }

    /// # SAFETY
    /// Caller must ensure that `head <= s.len()`
    #[inline]
    pub unsafe fn find(head: usize, s: &[u8], f: u8) -> usize {
        for i in head..s.len() {
            if *s.get_unchecked(i) == f {
                return i;
            }
        }

        s.len()
    }
}
