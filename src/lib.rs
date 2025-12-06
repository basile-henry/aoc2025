use std::iter::FromIterator;
use std::ops::Range;
use std::ops::RangeInclusive;

/// Loads input for a specific Advent of Code day.
///
/// Arguments:
/// * `day`: The day number (e.g., 1, 2, 25).
///
/// Priority:
/// 1. Command-line argument (e.g., `cargo run -- inputs/custom.txt`)
/// 2. Default pattern: `inputs/dayXX.txt` (where XX is zero-padded, e.g., "01")
pub fn get_input(day: u8) -> std::io::Result<String> {
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        // {:02} pads the number with a leading zero if it's less than 10.
        // 1 -> "inputs/day01.txt"
        // 15 -> "inputs/day15.txt"
        format!("inputs/day{:02}.txt", day)
    });

    std::fs::read_to_string(file_path)
}

#[derive(Debug, Clone)]
pub struct IntervalSet<T> {
    // The Vec always has an even number of elements
    // The elements alternate between an included and excluded part of ranges
    // For example [2, 5, 7, 9] means that the Interval set contains 2..5 and 7..9
    inner: Vec<T>,
}

impl<T> IntervalSet<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: Ord> IntervalSet<T> {
    /// Checks if x is contained within any interval.
    pub fn contains(&self, x: &T) -> bool {
        match self.inner.binary_search(x) {
            // Found exact match:
            // If index is Even, it's a Start (Included).
            // If index is Odd, it's an End (Excluded).
            Ok(index) => index % 2 == 0,

            // Not found, would be inserted at `index`:
            // If index is Odd, we are strictly inside an interval (Included).
            // If index is Even, we are in a gap (Excluded).
            Err(index) => index % 2 != 0,
        }
    }

    /// Inserts a range, merging overlapping or adjacent intervals.
    pub fn insert(&mut self, range: Range<T>) {
        assert!(range.start < range.end);

        // 1. Determine where the new range starts affecting the vector
        // and if we need to insert the new start point.
        let (start_idx, push_start) = match self.inner.binary_search(&range.start) {
            Ok(i) if i % 2 == 0 => (i + 1, false), // Hit existing Start: Keep it (merge).
            Ok(i) => (i, false),                   // Hit existing End: Remove it (merge).
            Err(i) if i % 2 == 0 => (i, true),     // In a gap: Insert new Start.
            Err(i) => (i, false),                  // Inside interval: Ignore new Start.
        };

        // 2. Determine where the new range stops affecting the vector
        // and if we need to insert the new end point.
        let (end_idx, push_end) = match self.inner.binary_search(&range.end) {
            Ok(i) if i % 2 == 0 => (i + 1, false), // Hit existing Start: Remove it (merge).
            Ok(i) => (i, false),                   // Hit existing End: Keep it (end of merge).
            Err(i) if i % 2 == 0 => (i, true),     // In a gap: Insert new End.
            Err(i) => (i, false),                  // Inside interval: Ignore new End.
        };

        // 3. Construct the new boundary elements to insert.
        let mut new_items = Vec::with_capacity(2);
        if push_start {
            new_items.push(range.start);
        }
        if push_end {
            new_items.push(range.end);
        }

        // 4. Replace the affected range in the vector.
        // We remove everything between the conceptual new start and new end.
        self.inner.splice(start_idx..end_idx, new_items);
    }
}

impl IntervalSet<usize> {
    pub fn count(&self) -> usize {
        self.inner
            .chunks_exact(2)
            .map(|chunk| chunk[1] - chunk[0])
            .sum()
    }

    /// Inserts an inclusive range, converting it to an exclusive range for storage.
    pub fn insert_inclusive(&mut self, range: RangeInclusive<usize>) {
        if range.is_empty() {
            return;
        }

        let start = *range.start();
        let end_inclusive = *range.end();

        // The exclusive end is (end_inclusive + 1).
        // This must be checked for overflow if end_inclusive is usize::MAX.
        if end_inclusive == usize::MAX {
            // Since usize::MAX + 1 overflows, the exclusive end must be MAX.
            self.insert(start..end_inclusive);
        } else {
            self.insert(start..(end_inclusive + 1));
        }
    }
}

impl<T: Ord + Copy> FromIterator<Range<T>> for IntervalSet<T> {
    fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
        let mut set = IntervalSet::new();
        for range in iter {
            set.insert(range);
        }
        set
    }
}

impl FromIterator<RangeInclusive<usize>> for IntervalSet<usize> {
    fn from_iter<I: IntoIterator<Item = RangeInclusive<usize>>>(iter: I) -> Self {
        let mut set = IntervalSet::new();
        for range in iter {
            set.insert_inclusive(range);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insertion_and_contains() {
        let mut set = IntervalSet::new();
        set.insert(10..20);

        assert!(set.contains(&10));
        assert!(set.contains(&15));
        assert!(!set.contains(&20)); // Exclusive end
        assert!(!set.contains(&5));
    }

    #[test]
    fn test_count_elements() {
        let mut set = IntervalSet::new();
        set.insert(1..5); // Size 4
        set.insert(10..15); // Size 5
        assert_eq!(set.count(), 9);
    }

    #[test]
    fn test_merge_overlapping() {
        let mut set = IntervalSet::new();
        set.insert(1..5);
        set.insert(4..8); // Overlaps

        // Should become 1..8
        assert_eq!(set.inner, vec![1, 8]);
        assert_eq!(set.count(), 7);
    }

    #[test]
    fn test_merge_subset_superset() {
        let mut set = IntervalSet::new();
        set.insert(1..10);

        // Insert subset (should do nothing)
        set.insert(2..5);
        assert_eq!(set.inner, vec![1, 10]);

        // Insert superset (should overwrite)
        set.insert(0..15);
        assert_eq!(set.inner, vec![0, 15]);
    }

    #[test]
    fn test_bridge_gap() {
        // This is a critical edge case
        let mut set = IntervalSet::new();
        set.insert(1..3); // [1, 3]
        set.insert(5..7); // [1, 3, 5, 7]

        // Insert a range that bridges the two: 2..6
        set.insert(2..6);

        // 1..3 + 2..6 + 5..7 -> 1..7
        assert_eq!(set.inner, vec![1, 7]);
        assert_eq!(set.count(), 6);
    }

    #[test]
    fn test_adjacent_merge() {
        let mut set = IntervalSet::new();
        set.insert(1..5);
        set.insert(5..10); // Starts exactly where previous ended

        assert_eq!(set.inner, vec![1, 10]);
        assert_eq!(set.count(), 9);
    }

    #[test]
    fn test_collect() {
        let ranges = vec![
            10..20,
            1..5, // Unsorted order
            4..8, // Overlaps with 1..5
        ];

        let set: IntervalSet<usize> = ranges.into_iter().collect();

        // 1..5 and 4..8 merge into 1..8
        // 10..20 is separate
        // Result: [1, 8, 10, 20]
        assert_eq!(set.inner, vec![1, 8, 10, 20]);
        assert_eq!(set.count(), 17); // (8-1) + (20-10) = 7 + 10 = 17
    }
}
