extern crate num_rust;
use num_rust::ext::traits::ToMatrix2d;
use num_rust::ext::impls::*;
use num_rust::Matrix2d;
use num_rust::utils::*;

extern crate image;
use image::GenericImage;

use std::path::Path;
use std::fs::File;

fn get_matrix_window(matrix: &Matrix2d, n: usize, m: usize, i: usize, j: usize) -> Option<Matrix2d> {
    if i + n > matrix.get_cols() || j + m > matrix.get_rows() {
        return None;
    }
    let mut res: Vec<f64> = Vec::new();
    for y in j..j+m {
        for x in i..i+n {
            res.push(matrix.get_row(y).unwrap()[x]);
        }
    }
    Some(res.reshape(n, m).unwrap())
}

struct Kernel {
    matrix: Matrix2d
}

impl Kernel {
    pub fn new(convolution: Matrix2d) -> Kernel {
        let size = convolution.get_rows() * convolution.get_cols();
        let norm_m = vec![1./(size as f64); size].reshape(convolution.get_rows(), convolution.get_cols()).unwrap();
        Kernel {
            matrix: convolution
        }
    }

    pub fn apply(&self, image: &Matrix2d) -> Option<Matrix2d> {
        let (r, c) = (self.matrix.get_rows(), self.matrix.get_cols());
        let (n, m) = (image.get_cols() - (c - 1), image.get_rows() - (r - 1));
        let mut data: Vec<f64> = Vec::new();
        for y in 0..m {
            for x in 0..n {
                let w = match get_matrix_window(image, c, r, y, x) {
                    Some(v) => v,
                    None => {
                        println!("error at {} {}", y, x);
                        return None;
                    }
                };
                let p = w.mult(&self.matrix).unwrap();
                data.push(sum_vec(p.get_matrix()).abs());
            }
        }
        Some(data.reshape(n, m).unwrap())
    }
}

fn main() {
    let mut image = (0..28*28).map(|i: usize| i as f64).collect::<Vec<f64>>().reshape(28, 28).unwrap();
    // println!("{:?}", image);
    // println!("{:?}", get_matrix_window(&image, 3, 3, 26, 26));
    let conv_m = vec![
        vec![-1., -1., -1.],
        vec![-1., 8., -1.],
        vec![-1., -1., -1.],
    ].reshape(3, 3).unwrap();
    let conv = Kernel::new(conv_m);
    // println!("{:?}", conv.apply(&image).unwrap());

    let img = image::open(&Path::new("test.png")).ok().expect("Opening image failed");
    let kernel = [-1.0f32, -1.0, -1.0,
              -1.0, 8.0, -1.0,
              -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    // let out = File::create(&Path::new("out.png")).unwrap();
    // let _ = filtered.save(out, image::PNG).ok().expect("Saving image failed");
}
