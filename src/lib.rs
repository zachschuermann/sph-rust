use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::nalgebra::core::Vector2;
use ggez::{Context, GameResult};
use lazy_static::lazy_static;

const RADIUS: f32 = 5.;

const REST_DENS: f32 = 1000.;   // rest density
const GAS_CONST: f32 = 2000.;   // const for equation of state
const H: f32 = 16.;             // kernel radius
const HSQ: f32 = H*H;           // radius^2 for optimization
const MASS: f32 = 65.;          // assume all particles have the same mass
const VISC: f32 = 250.;         // viscosity constant
const DT: f32 = 0.0008;         // integration timestep

lazy_static! {
    // external (gravitational) forces
    static ref G: Vector2<f32> = Vector2::new(0., 12000.*-9.8);
    // smoothing kernels defined in Müller and their gradients
    static ref POLY6: f32 = 315./(65.*std::f32::consts::PI*H.powf(9.));
    static ref SPIKY_GRAD: f32 = -45./(std::f32::consts::PI*H.powf(6.));
    static ref VISC_LAP: f32 = 45./(std::f32::consts::PI*H.powf(6.));
}

// simulation parameters
const EPS: f32 = H; // boundary epsilon
const BOUND_DAMPING: f32 = -0.5;

#[derive(Clone)]
struct Particle {
    x: Vector2<f32>,
    v: Vector2<f32>,
    f: Vector2<f32>,
    r: f32,
    p: f32,
}

pub struct State {
    particles: Vec<Particle>,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {x: Vector2::new(x, y), v: Vector2::new(0., 0.), f: Vector2::new(0., 0.), r: 0., p: 0.}
    } 
}

impl State {
    pub fn new() -> GameResult<State> {
        let particles = vec![Particle::new(1., 1.), Particle::new(20., 20.)];
        let s = State { particles };
        Ok(s)
    }
    fn update(&mut self) {
        self.density_pressure();
 	self.forces();
 	self.integrate();
    }
    fn density_pressure(&mut self) {
        let length = self.particles.len();
        for p_i in self.particles.iter_mut() {
            p_i.r = 0.;
            for j in 0..length - 1 {
                let p_j = self.particles[j].clone();
                let rij: Vector2<f32> = p_j.x - p_i.x;
                let r2 = rij.norm_squared();
                if r2 < HSQ {
                    p_i.r += 1.;//MASS*POLY6*HSQ-r2.powf(3.);
                }
            }
            p_i.p = GAS_CONST*(p_i.r - REST_DENS);
        }
    }
    fn forces(&mut self) {
        for p in self.particles.iter_mut() {
        }
    }
    fn integrate(&mut self) {
        for p in self.particles.iter_mut() {
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            RADIUS,
            1.0,
            graphics::WHITE,
        )?;

        for p in self.particles.iter() {
            graphics::draw(ctx, &circle, (na::Point2::from(p.x),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

