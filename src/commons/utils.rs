pub mod progress_bars {
    pub use indicatif::{ProgressBar, ProgressStyle};
    pub use indicatif::ProgressIterator;
    pub use indicatif::ParallelProgressIterator;

    pub fn default(len: usize) -> ProgressBar {
        ProgressBar::new(len as u64)
    }


    pub fn file_writer(expected_size: usize) -> ProgressBar {
        let progress_style =
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-");

        let progress_bar = ProgressBar::new(
            expected_size as u64,
        );

        progress_bar.with_style(progress_style)
    }

    pub fn hidden() -> ProgressBar {
        ProgressBar::hidden()
    }
}
