use std;
use url;
use Entry;

/// A reader that reads the `M3U` format from the underlying reader.
///
/// A `Reader` is a streaming reader. It reads data from the underlying reader on demand and reads
/// no more than strictly necessary.
///
/// The inner `reader` `R` must be some buffered reader as the "#EXTM3U" header, "#EXTINF:" tags
/// and entries are each read from a single line of plain text.
///
/// A `Reader` will only attempt to read entries of type `E`.
#[derive(Debug, Clone)]
pub struct Reader<R, E>
where
    R: std::io::BufRead,
{
    /// The reader from which the `M3U` format is read.
    reader: R,
    /// String used for buffering read lines.
    line_buffer: String,
    /// The entry type that the `reader` will read.
    entry: std::marker::PhantomData<E>,
}
mod entry;
mod entry_ext;
pub use self::entry::{Entries, EntryReader};
pub use self::entry_ext::{
    EntryExtReader, EntryExtReaderConstructionError, EntryExts, ReadEntryExtError,
};

impl<R, E> Reader<R, E>
where
    R: std::io::BufRead,
{
    fn new_inner(reader: R, line_buffer: String) -> Self {
        Reader {
            reader,
            line_buffer,
            entry: std::marker::PhantomData,
        }
    }

    /// Produce the inner `reader`.  
    pub fn into_inner(self) -> R {
        self.reader
    }
}

/// Attempt to read the next `Entry` from the inner reader.
fn read_next_entry<R>(
    reader: &mut R,
    line_buffer: &mut String,
) -> Result<Option<Entry>, std::io::Error>
where
    R: std::io::BufRead,
{
    loop {
        // Read the next line or return `None` if we're done.
        line_buffer.clear();
        if reader.read_line(line_buffer)? == 0 {
            return Ok(None);
        }

        let line = line_buffer.trim_start();
        match line.chars().next() {
            // Skip empty lines.
            None => continue,
            // Skip comments.
            Some('#') => continue,
            // Break when we have a non-empty, non-comment line.
            _ => return Ok(Some(read_entry(line.trim_end()))),
        }
    }
}

/// Read an `Entry` from the given line.
///
/// First attempts to read a URL entry. A URL is only returned if `Some` `host_str` is parsed.
///
/// If a URL cannot be parsed, we assume the entry is a `Path`.
fn read_entry(line: &str) -> Entry {
    if let Ok(url) = url::Url::parse(line) {
        if url.host_str().is_some() {
            return Entry::Url(url);
        }
    }
    Entry::Path(line.into())
}

#[cfg(feature = "iptv")]
pub mod iptv;
