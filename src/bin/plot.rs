extern crate proc_macro;
use boids::{Boid, Vec2d, World};
use inline_python::{python, Context};

fn plot_sim(time: Vec<usize>, pos: (Vec<f32>, Vec<f32>)) {
    let t1 = time.clone();
    let t2 = time;
    let x = pos.0;
    let y = pos.1;

    // Create Python context to plot using matplotlib
    let ctx = Context::new();

    ctx.run(python! {
        import matplotlib.pyplot as plt

        // Create figure/subplots
        fig, axs = plt.subplots(3, 1)

        // Plot Time vs X-Position
        axs[0].scatter('t1, 'x)
        axs[0].set_title("X-Position vs Time")

        // Plot Time vs Y-Position
        axs[1].scatter('t2, 'y)
        axs[1].set_title("Y-Position vs Time")

        // Plot Trajectory (X vs Y Positions)
        axs[2].scatter('x, 'y)
        axs[2].set_title("Trajectory")

        plt.show()
    });
}

fn run_sim() -> (Vec<usize>, Vec<f32>, Vec<f32>) {
    let boid = Boid::new(1., Vec2d::new(0., 0.), Vec2d::new(0., 5.));
    let mut world = World::new(500., 500., vec![boid]);

    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut time = Vec::new();

    // Run sim for 30 seconds
    for i in 0..30 {
        x.push(world.boids[0].position.x);
        y.push(world.boids[0].position.y);
        time.push(i);
        world.step(10.);
        world.warp_world_borders();
    }

    (time, x, y)
}

fn main() -> std::io::Result<()> {
    let (time, x, y) = run_sim();
    plot_sim(time, (x, y));
    Ok(())
}
