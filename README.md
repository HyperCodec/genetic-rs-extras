# genetic-rs-extras
![fitness graph](https://raw.githubusercontent.com/HyperCodec/genetic-rs-extras/main/assets/fitness.svg)

This crate contains features that I thought were a bit too intrusive to include in the main `genetic-rs` crate, but are still useful for a lot of common projects. The current feature list is as follows:
- `plotters` - Adds a `plot` module with the struct `FitnessPlotter`, which implements `FitnessObserver` and exposes a method to plot a chart on the given plotting backend.
- `indicatif` - Adds a `pb` module, which adds both a `ProgressExt` trait to extend `GeneticSim` with a simple non-fitness-tracking progress bar, and a struct called `ProgressObserver` that displays the fitness values on a progress bar.

### How to Use
In this guide, I'll assume you have at least basic knowledge of the `genetic-rs` framework. If not, you can find the docs [here](https://docs.rs/genetic-rs). This crate mostly just plugs into the existing `genetic-rs` ecosystem, so it can be used the same as any other crate in the ecosystem.

For the observers (i.e. `ProgressObserver` and `FitnessPlotter`), you can add these to your `FitnessEliminator`:
```rust,ignore
let cool_observer = ProgressObserver::new_with_default_style(100).layer(FitnessPlotter::default());
let eliminator = FitnessEliminator::builder()
    .observer(cool_observer)
    .fitness(my_fitness_func)
    .build_or_panic();
```

With things like `ProgressExt`, you can just import them and use the method added to `GeneticSim`:
```rust,ignore
use genetic_rs_extras::pb::ProgressExt;

let mut sim = GeneticSim::new(...);
let pb = sim.perform_generations_pb(100, ProgressStyle::default());
```

### Feature Requests and Contributions
Feature requests and PRs are welcome. The goal of this crate is to make using `genetic-rs` more convenient via features that aren't quite suitable for the main `genetic-rs` repo. For super complex features, I ask that you create your own separate crate that integrates with `genetic-rs`, but anything that is relatively simple to implement/maintain and offers meaningful convenience/eliminates boilerplate for the `genetic-rs` ecosystem can be added to this crate.

### License
This crate falls under the `MIT` license.