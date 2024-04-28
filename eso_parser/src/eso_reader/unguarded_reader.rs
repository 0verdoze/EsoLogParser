use std::simd::cmp::SimdPartialEq;
use super::{EsoLogReaderTrait, EsoLogLineReaderTrait, EsoReaderTrait};

// unguarded version of EsoLogReader, where reading past provided str slice is allowed
pub struct UnguardedEsoLogReader;

impl<'a> EsoReaderTrait<'a> for UnguardedEsoLogReader {
    type LineReader = UnguardedEsoLogReaderLine<'a>;
    type SplitReader = UnguardedEsoLogReaderSplitChar<'a>;

    #[inline(always)]
    fn read_line(s: &'a str) -> Self::LineReader {
        UnguardedEsoLogReaderLine(s.as_bytes())
    }

    #[inline(always)]
    fn new_split(s: &'a str, c: char) -> Self::SplitReader {
        let mut buf = [0; 1];
        // in reality only ASCII characters are allowed
        // this will panic if passed characted is not ASCII
        // 
        // we still do this instead of changing parameter to u8 (or straight up casting it to it)
        // to avoid worrying about invaliding utf-8 guarantee
        c.encode_utf8(&mut buf);

        UnguardedEsoLogReaderSplitChar(s.as_bytes(), buf[0])
    }
}

/// helper for iterating over values splitted by character
#[derive(Clone)]
pub struct UnguardedEsoLogReaderSplitChar<'a>(&'a [u8], u8);

impl<'a> EsoLogReaderTrait<'a> for UnguardedEsoLogReaderSplitChar<'a> {
    #[inline]
    fn is_depleted(&self) -> bool {
        self.0.is_empty()
    }

    fn inner(&self) -> &'a str {
        // SAFETY: self.0 can be only set from within this source file
        // and it is guranteed to be a valid utf-8 string
        unsafe {
            std::str::from_utf8_unchecked(self.0)
        }
    }
}

impl<'a> Iterator for UnguardedEsoLogReaderSplitChar<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // find index of provided character
        // end will be equal to `self.0.len()` if there is no such character
        // SAFETY: `UnguardedEsoLogReader` requires that reading past provided string
        // will not cause UB
        let end = unsafe { SimdFind::find(0, self.0, self.1) };

        // cast pointers to slice
        // SAFETY: SimdFind::find will always return value from range [0..=self.0.len()]
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
        if !result.is_empty() {
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
pub struct UnguardedEsoLogReaderLine<'a>(&'a [u8]);

impl<'a> EsoLogReaderTrait<'a> for UnguardedEsoLogReaderLine<'a> {
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

impl<'a> EsoLogLineReaderTrait<'a> for UnguardedEsoLogReaderLine<'a> {
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

impl<'a> Iterator for UnguardedEsoLogReaderLine<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.0;

        let end = match s.get(0)? {
            b'"' => unsafe {
                // SAFETY: caller guaranteed that reading past this string's buffer is safe
                // when they created this object instance
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

                    // suggests compiler to calculate both values and use `CMOV` for 3% performance boost
                    count = if c == b'[' { count + 1 } else { count };
                    count = if c == b']' { count - 1 } else { count };
                    
                    i += 1;
                    if count == 0 { break i; }
                };

                end
            },
            _ => unsafe {
                // SAFETY: caller guaranteed that reading past this string's buffer is safe
                // when they created this object instance
                SimdFind::find(0, s, b',')
            }
        };

        // &s[..end] without bound checking
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
    /// # SAFETY
    /// Caller must ensure that reading up to `31` past the passed string is safe
    unsafe fn find(mut head: usize, s: &[u8], f: u8) -> usize {
        type Simd = std::simd::Simd<u8, 32>;

        let mask = Simd::from_array([f; Simd::LEN]);
        while head < s.len() {
            let mut range = s.as_ptr_range();
            range.start = range.start.add(head);
            range.end = range.start.add(Simd::LEN);
    
            let values = Simd::from_array(std::slice::from_ptr_range(range).try_into().unwrap_unchecked());
            // SAFETY: this type must be casted to type with same amount of bits as Simd::LEN
            let result = values.simd_eq(mask).to_bitmask() as u32;
    
            head += result.trailing_zeros() as usize;
    
            if result != 0 {
                return head.min(s.len());
            }
        }
    
        s.len()
    }
}
