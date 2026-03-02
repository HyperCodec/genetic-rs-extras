use genetic_rs::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// This trait extends [`GeneticSim`][genetic_rs::GeneticSim] with variants of [`perform_generations`][genetic_rs::GeneticSim::perform_generations] that track the progress with a progress bar.
/// Note that this implementation is generic, so it will not have access to things like fitness. Use [`ProgressObserver`] if you want to track progress with access to the population and fitness values.
pub trait ProgressExt {
    /// Performs the specified number of generations while tracking the progress
    /// with a progress bar.
    fn perform_generations_tracked(&mut self, generations: u64, style: ProgressStyle) -> ProgressBar;

    /// Performs the specified number of generations while tracking the progress
    /// with a progress bar that is part of a [`MultiProgress`].
    fn perform_generations_multipb(&mut self, generations: u64, style: ProgressStyle, multi: MultiProgress) -> ProgressBar;
}

impl<G, E, R> ProgressExt for GeneticSim<G, E, R>
where 
    G: Sized,
    E: Eliminator<G>,
    R: Repopulator<G>,
{
    fn perform_generations_tracked(&mut self, generations: u64, style: ProgressStyle) -> ProgressBar {
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
            .template("{bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .tick_chars("#=>")
    }
}

impl<G> FitnessObserver<G> for ProgressObserver {
    fn observe(&self, genomes: &[(G, f32)]) {
        self.0.inc(1);

        assert!(!genomes.is_empty(), "genomes should not be empty");

        let highest = genomes.first().unwrap().1;
        let median = genomes[genomes.len() / 2].1;
        let lowest = genomes.last().unwrap().1;

        self.0.set_message(format!("Fitness: [hi: {highest:.2} med: {median:.2} lo: {lowest:.2}]"));
    }
}