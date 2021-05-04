use bytesize::ByteSize;
use std::time::{Instant, Duration};

/// Metric on a io operation
#[derive(Clone, Copy, Debug)]
pub struct IoMetric {
    pub start: Instant,
    pub duration: Duration,
    pub size: usize
}

impl IoMetric {
    /// Start measuring a new Io operation.
    fn begin(io_size: usize) -> IoMetric {
        IoMetric {
            start: Instant::now(),
            duration: Duration::from_secs(0),
            size: io_size
        }
    }

    /// Indicate that the Io operation being measured has stopped.
    fn end(&mut self) -> Duration {
        if self.duration == Duration::from_secs(0) {
            self.duration = self.start.elapsed();
        }
        self.duration
    }

    /// Calculate the bytes-per-second speed of this Io operation.
    /// This measurement is only approximate, as it implicitly includes the
    /// overhead of the io call itself which may be relatively fixed compared
    // to the actual io time taken per byte.
    pub fn bytes_per_second(&self) -> usize {
        if self.duration == Duration::from_secs(0) { return 0; }

        ((self.size as u128 * Duration::from_secs(1).as_nanos())
                / self.duration.as_nanos()) as usize
    }
}

impl std::fmt::Display for IoMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/s", ByteSize(self.bytes_per_second() as u64)
                    .to_string_as(true))
    }
}

/// Statistics gathered across multiple io ops
#[derive(Clone, Debug)]
pub struct IoStats {
    pub io_metrics: Vec<IoMetric>,  // TODO: make this circular, since
                                    //       otherwise this vec will expand
                                    //       until we run out of memory.
}

impl IoStats {
    pub fn new() -> IoStats {
        IoStats {
            io_metrics: Vec::new()
        }
    }

    /// Begin measuring a new Io operation.
    pub fn begin(&self, io_size: usize) -> IoMetric {
        IoMetric::begin(io_size)
    }

    /// Indicate the end of an Io operation, recording the results.
    pub fn end(&mut self, io_metric: &mut IoMetric) -> IoMetric {
        io_metric.end();
        let copy = io_metric.clone();  // we effectively delay until now the
                                       // copy operation that would take place
                                       // if we didn't pass the metric by
                                       // reference
        self.io_metrics.push(copy);

        copy
    }

    pub fn avg_bytes_per_second(&self) -> usize {
        if self.io_metrics.len() == 0 { return 0; }

        self.io_metrics.iter().fold(0,
                |sum, metric| sum + metric.bytes_per_second())
            / self.io_metrics.len()
    }

    // TODO: create function that tries to calculate io op overhead
    // TODO: create function that tries to predict io time for a given region size
    // TODO: create function that tries to determine most efficient io plan for a set of projected io operations
}

impl std::fmt::Display for IoStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bytes_per_second = self.avg_bytes_per_second() as u64;
        write!(f, "~{}/s", ByteSize(bytes_per_second).to_string_as(true))
    }
}

impl std::ops::Add for IoStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut aggregate = IoStats::new();

        // TODO: when we make the internal buffer circular, merge in the latest
        //       metrics from both IoStats objects first
        aggregate.io_metrics = [&self.io_metrics[..], &other.io_metrics[..]].concat();

        aggregate
    }
}

// TODO: create RegionMetadatStats for tracking time taken to dump region metadata based on region count