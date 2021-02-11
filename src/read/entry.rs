use super::{read_next_entry, Reader};
use Entry;
/// A `Reader` that specifically reads `Entry`s.
pub type EntryReader<R> = Reader<R, Entry>;

/// An iterator that yields `Entry`s.
///
/// All `Entry`s are lazily read from the inner buffered reader.
pub struct Entries<'r, R>
where
    R: 'r + std::io::BufRead,
{
    reader: &'r mut EntryReader<R>,
}

impl<R> EntryReader<R>
where
    R: std::io::BufRead,
{
    /// Create a reader that reads the original, non-extended M3U `Entry` type.
    pub fn new(reader: R) -> Self {
        Self::new_inner(reader, String::new())
    }

    /// Attempt to read the next `Entry` from the inner reader.
    ///
    /// Returns `Ok(None)` when there are no more lines.
    ///
    /// Returns an `Err(std::io::Error)` if an error occurs when calling the inner `reader`'s
    /// `BufRead::read_line` method.
    fn read_next_entry(&mut self) -> Result<Option<Entry>, std::io::Error> {
        let Reader {
            ref mut reader,
            ref mut line_buffer,
            ..
        } = *self;
        read_next_entry(reader, line_buffer)
    }

    /// Produce an iterator that yields `Entry`s.
    ///
    /// All `Entry`s are lazily read from the inner buffered reader.
    pub fn entries(&mut self) -> Entries<R> {
        Entries { reader: self }
    }
}

impl<'r, R> Iterator for Entries<'r, R>
where
    R: std::io::BufRead,
{
    type Item = Result<Entry, std::io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_next_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
