use genetic_rs::prelude::*;
#[cfg(feature = "indicatif")]
use genetic_rs_extras::pb::ProgressObserver;
#[cfg(feature = "plotters")]
use genetic_rs_extras::plot::FitnessPlotter;

const GENERATIONS: usize = 1000;
const POPULATION_SIZE: usize = 500;
const MUTATION_RATE: f32 = 0.1;
const MUTATION_AMOUNT: f32 = 0.05;

#[cfg(feature = "plotters")]
const PLOT_PATH: &str = "./fitness.svg";

#[derive(Debug, Clone, Mitosis)]
#[mitosis(use_randmut = true)]
struct MyGenome {
    x: f32,
    y: f32,
    z: f32,
}

impl GenerateRandom for MyGenome {
    fn gen_random(rng: &mut impl rand::Rng) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }
}

impl RandomlyMutable for MyGenome {
    type Context = f32;

    fn mutate(&mut self, mutation_amount: &Self::Context, rate: f32, rng: &mut impl rand::Rng) {
        if rng.random::<f32>() < rate {
            self.x += rng.random::<f32>() * mutation_amount;
        }
        if rng.random::<f32>() < rate {
            self.y += rng.random::<f32>() * mutation_amount;
        }
        if rng.random::<f32>() < rate {
            self.z += rng.random::<f32>() * mutation_amount;
        }
    }
}

impl Crossover for MyGenome {
    type Context = f32;

    fn crossover(
        &self,
        other: &Self,
        ctx: &Self::Context,
        rate: f32,
        rng: &mut impl rand::Rng,
    ) -> Self {
        let mut child = self.clone();
        child.x = (self.x + other.x) / 2.0;
        child.y = (self.y + other.y) / 2.0;
        child.z = (self.z + other.z) / 2.0;

        child.mutate(ctx, rate, rng);

        child
    }
}

fn fitness(genome: &MyGenome) -> f32 {
    // A simple fitness function that tries to maximize the product of x and y while minimizing z
    genome.x * genome.y - genome.z
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    let observer = ();
    
    #[cfg(feature = "indicatif")]
    let observer = observer.layer(ProgressObserver::new_with_default_style(GENERATIONS as u64));

    #[cfg(feature = "plotters")]
    let observer = observer.layer(FitnessPlotter::new());

    let eliminator = FitnessEliminator::builder()
        .fitness_fn(fitness)
        .observer(observer)
        .build_or_panic();

    let mut sim = GeneticSim::new(
        Vec::gen_random(&mut rng, POPULATION_SIZE),
        eliminator,
        CrossoverRepopulator::new(MUTATION_RATE, MUTATION_AMOUNT),
    );

    sim.perform_generations(GENERATIONS);

    #[cfg(feature = "indicatif")]
    {
        #[cfg(feature = "plotters")]
        sim.eliminator.observer.0.1.finish();

        #[cfg(not(feature = "plotters"))]
        sim.eliminator.observer.1.finish();
    }

    #[cfg(feature = "plotters")]
    {
        use plotters::prelude::{IntoDrawingArea, SVGBackend};

        let backend = SVGBackend::new(PLOT_PATH, (800, 600));
        let drawing_area = backend.into_drawing_area();
        sim.eliminator.observer.1.plot(&drawing_area)?;
        drawing_area.present()?;
        println!("Fitness plot saved to {}", PLOT_PATH);
    }

    Ok(())
}