#![cfg_attr(not(test), no_std)]

use sys::MagnetoCalibration;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Magneto(sys::MagnetoCalibration);

impl Magneto {
    pub const fn new() -> Magneto {
        Magneto(MagnetoCalibration {
            ata: [0.0; 100],
            norm_sum: 0.0,
            sample_count: 0.0,
        })
    }

    pub fn sample(&mut self, x: f64, y: f64, z: f64) {
        // SAFETY: perfectly safe, bindgen is just paranoid
        unsafe { self.0.sample(x, y, z) }
    }

    pub fn current_calibration(&self) -> [[f32; 3]; 4] {
        let mut buf = [[0.0; 3]; 4];
        // SAFETY: current_calibration doesn't write to the class.
        // a *mut is also a fancy usize and I don't need to guarantee anything.
        unsafe {
            sys::MagnetoCalibration_current_calibration(
                &self.0 as *const MagnetoCalibration as usize as *mut MagnetoCalibration,
                buf.as_mut_ptr(),
            );
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(lhs: [[f32; 3]; 4], rhs: [[f32; 3]; 4]) {
        for y in 0..4 {
            for x in 0..3 {
                assert!(
                    (lhs[y][x] - rhs[y][x]).abs() < 0.001,
                    "lhs[{y}][{x}] != rhs[{y}][{x}]"
                );
            }
        }
    }

    #[test]
    fn it_works() {
        let mut magneto = Magneto::new();
        let samples = include_str!("mag.txt");
        for line in samples.lines() {
            let sample = line
                .split('\t')
                .map(str::parse::<f64>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            magneto.sample(sample[0], sample[1], sample[2]);
        }

        assert_approx_eq(
            magneto.current_calibration(),
            [
                [-0.021659242, 0.013249581, -0.026166687],
                [0.9779701, -0.018199349, 0.006708033],
                [-0.018199349, 0.98569584, 0.003376677],
                [0.006708033, 0.003376677, 1.0588791],
            ],
        );
    }
}
