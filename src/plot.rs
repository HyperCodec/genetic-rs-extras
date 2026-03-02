use genetic_rs::prelude::*;
use plotters::{coord::{Shift, types::{RangedCoordf32, RangedCoordusize}}, prelude::*};

/// A struct implementing [`FitnessObserver`] that collects fitness values across generations and can plot them using the [`plotters`] crate.
#[derive(Default, Debug, Clone)]
pub struct FitnessPlotter {
    /// The highest fitness value observed in each generation.
    pub highest: Vec<f32>,
    
    /// The median fitness value observed in each generation.
    pub median: Vec<f32>,

    /// The lowest fitness value observed in each generation.
    pub lowest: Vec<f32>,
}

impl FitnessPlotter {
    /// Creates a new [`FitnessPlotter`] with an empty fitness history. Identical to [`default`][Self::default].
    pub fn new() -> Self {
        Self {
            highest: Vec::new(),
            median: Vec::new(),
            lowest: Vec::new(),
        }
    }

    /// Plots the fitness history using the provided [`DrawingBackend`].
    /// Plots the fitness history using the provided [`DrawingBackend`].
    /// This will plot the highest, median, and lowest fitness values for each generation.
    pub fn plot<'a, DB: DrawingBackend>(&self, root: &'a DrawingArea<DB, Shift>) ->
        Result<
            ChartContext<'a, DB, Cartesian2d<RangedCoordusize, RangedCoordf32>>,
            DrawingAreaErrorKind<DB::ErrorType>
        >
    {
        root.fill(&WHITE)?;

        let max = self.highest.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        let mut chart = ChartBuilder::on(root)
            .caption("fitness values per generation", ("sans-serif", 50).into_font())
            .margin(20)
            .x_label_area_size(50)
            .y_label_area_size(30)
            .build_cartesian_2d(0usize..self.highest.len(), 0f32..max)?;
        
        chart.configure_mesh()
            .x_desc("generation")
            .y_desc("fitness")
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                self.highest.iter().cloned().enumerate(),
                GREEN,
            ))?
            .label("highest")
            .legend(|(x,y)| Rectangle::new([(x - 5, y + 1), (x + 10, y)], GREEN));

        chart
            .draw_series(LineSeries::new(
                self.median.iter().cloned().enumerate(),
                YELLOW,
            ))?
            .label("median")
            .legend(|(x,y)| Rectangle::new([(x - 5, y + 1), (x + 10, y)], YELLOW));

        chart
            .draw_series(LineSeries::new(
                self.lowest.iter().cloned().enumerate(),
                RED,
            ))?
            .label("lowest")
            .legend(|(x,y)| Rectangle::new([(x - 5, y + 1), (x + 10, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(chart)
    }
}

impl<G> FitnessObserver<G> for FitnessPlotter {
    fn observe(&mut self, fitnesses: &[(G, f32)]) {
        // these fitness values are already sorted by the caller, so we can just take the first, middle, and last values
        let highest = fitnesses.first().map(|(_, f)| *f).unwrap_or(0.0);
        let lowest = fitnesses.last().map(|(_, f)| *f).unwrap_or(0.0);
        let median = fitnesses.get(fitnesses.len() / 2).map(|(_, f)| *f).unwrap_or(0.0);

        self.highest.push(highest);
        self.median.push(median);
        self.lowest.push(lowest);
    }
}