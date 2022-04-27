use rand::prelude::*;

/// a sampler trait for a 2D square area
/// this is an iterator-like API, where `sample` for the `next` method
pub trait AreaSampler {
    fn sample(&mut self) -> Option<(f32, f32)>;
    fn sample_in_disk(&mut self) -> Option<(f32, f32)>;
    fn get_range(&self) -> f32;
}

/// the random sampler on a uniform distribution
pub struct WhiteNoiseSampler {
    rng: rand::prelude::ThreadRng,
    range: f32,
    rate: i32,
}

impl WhiteNoiseSampler {
    pub fn new(range: f32, rate: i32) -> Self {
        WhiteNoiseSampler {
            rng: rand::thread_rng(),
            range: range,
            rate: rate,
        }
    }

    fn gen_point(&mut self) -> (f32, f32) {
        (
            self.rng.gen::<f32>() * self.range,
            self.rng.gen::<f32>() * self.range,
        )
    }
}

impl AreaSampler for WhiteNoiseSampler {
    fn sample(&mut self) -> Option<(f32, f32)> {
        self.rate -= 1;
        if self.rate >= 0 {
            Some(self.gen_point())
        } else {
            None
        }
    }

    fn sample_in_disk(&mut self) -> Option<(f32, f32)> {
        self.rate -= 1;
        if self.rate >= 0 {
            let ret = loop {
                let p = self.gen_point();
                if _in_disk(self, &p) {
                    break p;
                }
            };
            Some(ret)
        } else {
            None
        }
    }

    fn get_range(&self) -> f32 {
        self.range
    }
}

/// Uniform sampling
pub struct UniformSampler {
    interval: f32,
    range: f32,
    rate: i32,
    i: f32,
    j: f32,
}

impl UniformSampler {
    /// the second parameter is a reference sample rate,
    /// which will be shortened to the nearest square number
    pub fn new(range: f32, ref_rate: i32) -> Self {
        let edge_rate = (ref_rate as f64).sqrt() as i32;
        let rate = edge_rate * edge_rate;
        let interval = range / edge_rate as f32;
        UniformSampler {
            interval: interval,
            range: range,
            rate: rate,
            i: interval / 2.0,
            j: interval / 2.0,
        }
    }
}

impl AreaSampler for UniformSampler {
    fn sample(&mut self) -> Option<(f32, f32)> {
        let ret = if self.rate > 0 {
            Some((self.i, self.j))
        } else {
            None
        };
        self.rate -= 1;
        self.i += self.interval;
        if self.i > self.range {
            self.i = self.interval / 2.0;
            self.j += self.interval;
        }
        ret
    }

    fn sample_in_disk(&mut self) -> Option<(f32, f32)> {
        _sample_in_disk(self)
    }

    fn get_range(&self) -> f32 {
        self.range
    }
}

/// Stratified Sampling
pub struct JitteredSampler {
    uniform_sampler: UniformSampler,
    rng: rand::prelude::ThreadRng,
    jitter_rate: f32,
}

impl JitteredSampler {
    pub fn new(range: f32, ref_rate: i32) -> Self {
        let uniform = UniformSampler::new(range, ref_rate);
        let jitter_rate = uniform.interval / 2.0;
        JitteredSampler {
            uniform_sampler: uniform,
            rng: rand::thread_rng(),
            jitter_rate: jitter_rate,
        }
    }
}

impl AreaSampler for JitteredSampler {
    fn sample(&mut self) -> Option<(f32, f32)> {
        self.uniform_sampler.sample().map(|(a, b)| {
            (
                a + self.rng.gen::<f32>() * self.jitter_rate,
                b + self.rng.gen::<f32>() * self.jitter_rate,
            )
        })
    }

    fn sample_in_disk(&mut self) -> Option<(f32, f32)> {
        _sample_in_disk(&mut self.uniform_sampler)
    }

    fn get_range(&self) -> f32 {
        self.uniform_sampler.get_range()
    }
}

pub struct BlueNoiseSampler {
    range: f32,
    rate: i32,
    radius: f32,
    rng: rand::prelude::ThreadRng,
    points: Vec<(f32, f32)>,
}

impl BlueNoiseSampler {
    pub fn new(range: f32, rate: i32) -> Self {
        let radius = (range / (rate as f32).sqrt()) * 0.8;
        BlueNoiseSampler {
            range: range,
            rate: rate,
            radius: radius,
            rng: rand::thread_rng(),
            points: Vec::new(),
        }
    }

    fn conflict(&self, p: &(f32, f32)) -> bool {
        for i in self.points.iter() {
            if sq_dist(p, &i) < self.radius.powi(2) {
                return true;
            }
        }
        false
    }
}

impl AreaSampler for BlueNoiseSampler {
    fn sample(&mut self) -> Option<(f32, f32)> {
        self.rate -= 1;
        if self.rate >= 0 {
            let ret = loop {
                let p = (
                    self.rng.gen::<f32>() * self.range,
                    self.rng.gen::<f32>() * self.range,
                );
                if !self.conflict(&p) {
                    self.points.push(p);
                    break p;
                }
            };
            Some(ret)
        } else {
            None
        }
    }

    fn sample_in_disk(&mut self) -> Option<(f32, f32)> {
        _sample_in_disk(self)
    }

    fn get_range(&self) -> f32 {
        self.range
    }
}

// helper functions

fn _in_disk<T: AreaSampler>(sampler: &mut T, p: &(f32, f32)) -> bool {
    let radius = sampler.get_range() / 2.0;
    let cen = (radius, radius);
    sq_dist(p, &cen) < radius.powi(2)
}

fn _sample_in_disk<T: AreaSampler>(sampler: &mut T) -> Option<(f32, f32)> {
    loop {
        match sampler.sample() {
            Some(p) => {
                if _in_disk(sampler, &p) {
                    return Some(p);
                }
            }
            None => {
                return None;
            }
        }
    }
}

fn sq_dist(a: &(f32, f32), b: &(f32, f32)) -> f32 {
    (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)
}