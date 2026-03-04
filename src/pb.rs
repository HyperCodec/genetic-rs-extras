use genetic_rs::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// This trait extends [`GeneticSim`] with variants of [`perform_generations`][genetic_rs::GeneticSim::perform_generations] that track the progress with a progress bar.
/// Note that this implementation is generic, so it will not have access to things like fitness. Use [`ProgressObserver`] if you want to track progress with access to the population and fitness values.
pub trait ProgressExt {
    /// Performs the specified number of generations while tracking the progress
    /// with a progress bar.
    fn perform_generations_pb(&mut self, generations: u64, style: ProgressStyle) -> ProgressBar;

    /// Performs the specified number of generations while tracking the progress
    /// with a progress bar that is part of a [`MultiProgress`].
    fn perform_generations_multipb(&mut self, generations: u64, style: ProgressStyle, multi: MultiProgress) -> ProgressBar;
}

impl<G, E, R> ProgressExt for GeneticSim<G, E, R>
where 
    G: Sized,
    E: FeatureBoundedEliminator<G>,
    R: FeatureBoundedRepopulator<G>,
{
    fn perform_generations_pb(&mut self, generations: u64, style: ProgressStyle) -> ProgressBar {
        let pb = ProgressBar::new(generations);
        pb.set_style(style);

        for _ in 0..generations {
            self.next_generation();
            pb.inc(1);
        }

        pb.finish();

        pb
    }

    fn perform_generations_multipb(&mut self, generations: u64, style: ProgressStyle, multi: MultiProgress) -> ProgressBar {
        let pb = multi.add(ProgressBar::new(generations));
        pb.set_style(style);

        for _ in 0..generations {
            self.next_generation();
            pb.inc(1);
        }

        pb.finish();

        pb
    }
}

/// A simple wrapper around `ProgressBar` that can be used as an observer to track the progress of a genetic algorithm with access to the population and fitness values.
pub struct ProgressObserver(pub ProgressBar);

impl ProgressObserver {
    /// Creates a new `ProgressObserver` with the specified number of generations and progress bar style.
    pub fn new(generations: u64, style: ProgressStyle) -> Self {
        let pb = ProgressBar::new(generations);
        pb.set_style(style);
        Self(pb)
    }

    /// Creates a new [`ProgressObserver`] with the specified number of generations and progress bar style, and adds it to a [`MultiProgress`].
    pub fn new_multi(generations: u64, style: ProgressStyle, multi: MultiProgress) -> Self {
        let pb = multi.add(ProgressBar::new(generations));
        pb.set_style(style);
        Self(pb)
    }

    /// Returns a default progress bar style that can be used with [`ProgressObserver`].
    pub fn default_style() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("Elapsed: {elapsed} Gen: {pos}/{len} {msg} {bar:40.cyan/blue} (eta: {eta})")
            .unwrap()
            .tick_chars("#=>")
    }

    /// Finishes the progress bar. This should be called after the genetic algorithm has completed all generations to ensure that the progress bar is properly finalized.
    pub fn finish(&self) {
        self.0.finish();
    }

    /// Finishes the progress bar and clears it from the terminal. This can be useful if you want to clean up the output after the genetic algorithm has completed.
    pub fn finish_and_clear(&self) {
        self.0.finish_and_clear();
    }

    /// Creates a new [`ProgressObserver`] with the specified number of generations and the default progress bar style.
    pub fn new_with_default_style(generations: u64) -> Self {
        Self::new(generations, Self::default_style())
    }

    /// Creates a new [`ProgressObserver`] with the specified number of generations and the default progress bar style, and adds it to a [`MultiProgress`].
    pub fn new_multi_with_default_style(generations: u64, multi: MultiProgress) -> Self {
        Self::new_multi(generations, Self::default_style(), multi)
    }
}

impl<G> FitnessObserver<G> for ProgressObserver {
    fn observe(&mut self, genomes: &[(G, f32)]) {
        self.0.inc(1);

        assert!(!genomes.is_empty(), "genomes should not be empty");

        let highest = genomes.first().unwrap().1;
        let median = genomes[genomes.len() / 2].1;
        let lowest = genomes.last().unwrap().1;

        self.0.set_message(format!("Fitness: [hi: {highest:.2} med: {median:.2} lo: {lowest:.2}]"));
    }
}