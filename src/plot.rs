use std::cell::RefCell;

use genetic_rs::prelude::*;
use plotters::{coord::{Shift, types::{RangedCoordf32, RangedCoordusize}}, prelude::*};

/// A struct to hold the fitness values across each generation, which can be used for plotting.
/// Each field should have the same length, corresponding to the number of generations observed.
/// The index represents the generation number.
#[derive(Default, Debug, Clone)]
pub struct FitnessHistory {
    /// The highest fitness value observed in each generation.
    pub highest: Vec<f32>,
    
    /// The median fitness value observed in each generation.
    pub median: Vec<f32>,

    /// The lowest fitness value observed in each generation.
    pub lowest: Vec<f32>,
}

impl FitnessHistory {
    /// Plots the fitness history using the provided [`DrawingBackend`].
    /// This will plot the highest, median, and lowest fitness values for each generation.
    pub fn plot<'a, DB: DrawingBackend>(&self, root: &'a DrawingArea<DB, Shift>) -> Result<ChartContext<'a, DB, Cartesian2d<RangedCoordusize, RangedCoordf32>>, DrawingAreaErrorKind<DB::ErrorType>> {
        let max_highest = self.highest.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        let mut chart = ChartBuilder::on(root)
            .caption("fitness values per generation", ("sans-serif", 50).into_font())
            .margin(20)
            .x_label_area_size(50)
            .y_label_area_size(30)
            .build_cartesian_2d(0usize..self.highest.len(), 0f32..max_highest)?;
        
        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                self.highest.iter().cloned().enumerate(),
                GREEN,
            ))?
            .label("highest");

        chart
            .draw_series(LineSeries::new(
                self.median.iter().cloned().enumerate(),
                YELLOW,
            ))?
            .label("median");

        chart
            .draw_series(LineSeries::new(
                self.lowest.iter().cloned().enumerate(),
                RED,
            ))?
            .label("lowest");

        Ok(chart)
    }
}

/// A struct to hold the fitness history for all generations, which can be used for plotting.
#[derive(Default, Debug, Clone)]
pub struct FitnessPlotter {
    /// The fitness history for all generations, which gets updated in [`observe`][Self::observe].
    pub history: RefCell<FitnessHistory>,
}

impl FitnessPlotter {
    /// Creates a new [`FitnessPlotter`] with an empty fitness history. Identical to [`default`][Self::default].
    pub fn new() -> Self {
        Self {
            history: RefCell::new(FitnessHistory::default()),
        }
    }

    /// Plots the fitness history using the provided [`DrawingBackend`].
    pub fn plot<'a, DB: DrawingBackend>(&self, root: &'a DrawingArea<DB, Shift>) -> Result<ChartContext<'a, DB, Cartesian2d<RangedCoordusize, RangedCoordf32>>, DrawingAreaErrorKind<DB::ErrorType>> {
        self.history.borrow().plot(root)
    }
}

impl<G> FitnessObserver<G> for FitnessPlotter {
    fn observe(&self, fitnesses: &[(G, f32)]) {
        // these fitness values are already sorted by the caller, so we can just take the first, middle, and last values
        let highest = fitnesses.first().map(|(_, f)| *f).unwrap_or(0.0);
        let lowest = fitnesses.last().map(|(_, f)| *f).unwrap_or(0.0);
        let median = fitnesses.get(fitnesses.len() / 2).map(|(_, f)| *f).unwrap_or(0.0);

        let mut history = self.history.borrow_mut();
        history.highest.push(highest);
        history.median.push(median);
        history.lowest.push(lowest);
    }
}