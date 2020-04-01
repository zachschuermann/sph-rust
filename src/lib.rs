use std::f32::consts::PI;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::nalgebra::core::Vector2;
use ggez::{Context, GameResult};
use lazy_static::lazy_static;
use rand::random;

use rayon::prelude::*;

const RADIUS: f32 = H/2.;

const REST_DENS: f32 = 1000.; // rest density
const GAS_CONST: f32 = 2000.; // const for equation of state
const H: f32 = 16.; // kernel radius
const HSQ: f32 = H * H; // radius^2 for optimization
const MASS: f32 = 65.; // assume all particles have the same mass
const VISC: f32 = 250.; // viscosity constant
const DT: f32 = 0.0008; // integration timestep

// simulation parameters
const EPS: f32 = H; // boundary epsilon
const BOUND_DAMPING: f32 = -0.5;

//const MAX_PARTICLES: usize = 2500;
//const DAM_PARTICLES: usize = 500;
//const BLOCK_PARTICLES: usize = 250;

// re projection parameters
const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 600;
const VIEW_WIDTH: f32 = 1.5 * WINDOW_WIDTH as f32;
const VIEW_HEIGHT: f32 = 1.5 * WINDOW_HEIGHT as f32;

lazy_static! {
    // external (gravitational) forces
    static ref G: Vector2<f32> = Vector2::new(0., 12000.*-9.8);
    // smoothing kernels defined in Müller and their gradients
    static ref POLY6: f32 = 315./(65.*PI*H.powf(9.));
    static ref SPIKY_GRAD: f32 = -45./(PI*H.powf(6.));
    static ref VISC_LAP: f32 = 45./(PI*H.powf(6.));
}

#[derive(Clone, PartialEq, Debug)]
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
        Particle {
            x: Vector2::new(x, y),
            v: Vector2::new(0., 0.),
            f: Vector2::new(0., 0.),
            r: 0.,
            p: 0.,
        }
    }
}

impl State {
    pub fn new() -> GameResult<State> {
        let mut particles = vec![];
        // x iter
        for i in 10..50 {
            // y iter
            for j in 10..40 {
                let jitter = random::<f32>();
                particles.push(Particle::new(i as f32 * H + jitter, j as f32 * H));
            }
        }
        let s = State { particles };
        Ok(s)
    }
    pub fn update(&mut self) {
        self.density_pressure();
        self.forces();
        self.integrate();
    }
    fn density_pressure(&mut self) {
        let particles_clone = self.particles.clone();
        //for p_i in &mut self.particles {
        self.particles = self.particles.par_iter().map(|p_i| {
            let mut r = 0.;
            for p_j in &particles_clone {
                let rij: Vector2<f32> = p_j.x - p_i.x;
                let r2 = rij.norm_squared();
                if r2 < HSQ {
                    r += MASS * (*POLY6) * (HSQ - r2).powf(3.);
                }
            }
            let p = GAS_CONST * (r - REST_DENS);
            Particle {x: p_i.x, v: p_i.v, f: p_i.f, r, p}
        }).collect();
    } 
    fn forces(&mut self) {
        let particles_clone = self.particles.clone();
        self.particles = self.particles.par_iter().map(|p_i| {
            let mut fpress = Vector2::new(0., 0.);
            let mut fvisc = Vector2::new(0., 0.);
            for p_j in &particles_clone {
                if p_i == p_j {
                    continue;
                }
                let rij = p_j.x - p_i.x;
                let r = rij.norm();
                if r < H {
                    // compute pressure force contribution
                    fpress += -rij.normalize() * MASS * (p_i.p + p_j.p) / (2. * p_j.r)
                        * (*SPIKY_GRAD)
                        * (H - r).powf(2.);
                    // compute viscosity force contribution
                    fvisc += VISC * MASS * (p_j.v - p_i.v) / p_j.r * (*VISC_LAP) * (H - r);
                }
            }
            let fgrav = (*G) * p_i.r;
            //p_i.f = fpress + fvisc + fgrav;
            let x = p_i.x;
            let v = p_i.v;
            let r = p_i.r;
            let p = p_i.p;
            Particle {x, v, r, p, f: fpress + fvisc + fgrav}
        }).collect();
    }
    fn integrate(&mut self) {
        for p in self.particles.iter_mut() {
            // forward Euler integration
            p.v += DT * p.f / p.r;
            p.x += DT * p.v;

            // enforce boundary conditions
            if p.x[0] - EPS < 0. {
                p.v[0] *= BOUND_DAMPING;
                p.x[0] = EPS;
            }
            if p.x[0] + EPS > VIEW_WIDTH {
                p.v[0] *= BOUND_DAMPING;
                p.x[0] = VIEW_WIDTH - EPS;
            }
            if p.x[1] - EPS < 0. {
                p.v[1] *= BOUND_DAMPING;
                p.x[1] = EPS;
            }
            if p.x[1] + EPS > VIEW_HEIGHT {
                p.v[1] *= BOUND_DAMPING;
                p.x[1] = VIEW_HEIGHT - EPS;
            }
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
