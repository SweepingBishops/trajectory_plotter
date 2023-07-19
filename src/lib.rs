#![allow(unused)]
use std::time::Instant;
use std::thread;
use std::result::Result;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyFloat};

#[pyfunction]
fn solve_multiple<'a>(py: Python<'a>, inits: &PyTuple, stop: &PyFloat, step: &PyFloat, params: &PyTuple) -> PyResult<&'a PyTuple> {
    let start = Instant::now();
    let stop: f64 = stop.extract().unwrap();
    let step: f64 = step.extract().unwrap();
    let params: (f64, f64, f64) = params.extract().unwrap();

    let mut trajectories = Vec::new();
    let mut procs = Vec::new();

    for init in inits {
        let init: (f64, f64, f64) = init.extract().unwrap();
        let proc = thread::spawn(move || {runge_kutta_solve(init, stop, step, params)});
        procs.push(proc);
    }

    for proc in procs {
        let trajectory = proc.join().unwrap();
        trajectories.push(PyTuple::new(py, trajectory));
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(PyTuple::new(py, trajectories))
}

fn runge_kutta_solve(init: (f64,f64,f64), stop: f64, step: f64, params: (f64, f64, f64)) -> [Vec<f64>; 4]  {
    let mut t: f64 = 0.0;
    let mut point: (f64, f64, f64) = init;
    let mut x_points: Vec<f64> = vec![point.0];
    let mut y_points: Vec<f64> = vec![point.1];
    let mut z_points: Vec<f64> = vec![point.2];
    let mut t_points: Vec<f64> = vec![t];

    let mut k1: (f64, f64, f64);
    let mut k2: (f64, f64, f64);
    let mut k3: (f64, f64, f64);
    let mut k4: (f64, f64, f64);

    while t < stop {
        t += step;
        k1 = scalar_mult(point_dot(point, params), step);
        k2 = scalar_mult(point_dot(add(point, scalar_mult(k1, 1./2.)), params), step);
        k3 = scalar_mult(point_dot(add(point, scalar_mult(k2, 1./2.)), params), step);
        k4 = scalar_mult(point_dot(add(point, k3), params), step);

        point = add(point,
            scalar_mult(add(add(k1, k4), scalar_mult(add(k2,k3), 1./2.)),
                1./6.));
        x_points.push(point.0);
        y_points.push(point.1);
        z_points.push(point.2);
        t_points.push(t);
    }
    let index = x_points.len()-1;
    println!("Final point for {:?}: ({},{},{})\nat t={}",init, x_points[index], y_points[index], z_points[index], t_points[index]);
    //Ok(PyTuple::new(py, vec![x_points, y_points, z_points, t_points]))
    [x_points, y_points, z_points, t_points]
}


fn point_dot(point: (f64, f64, f64), params: (f64, f64, f64)) -> (f64, f64, f64) {
    let x = point.0;
    let y = point.1;
    let z = point.2;

    let σ = params.0;
    let b = params.1;
    let r = params.2;

    let x_dot = σ*(y-x);
    let y_dot = r*x - y - x*z;
    let z_dot = x*y - b*z;
    (x_dot, y_dot, z_dot)
}

fn scalar_mult(point: (f64, f64, f64,), scalar: f64) -> (f64, f64, f64,) {
    (point.0*scalar, point.1*scalar, point.2*scalar)
}

fn add(point1: (f64, f64, f64), point2: (f64, f64, f64)) -> (f64, f64, f64) {
    (point1.0+point2.0,
     point1.1+point2.1,
     point1.2+point2.2)
}

// A Python module implemented in Rust.
#[pymodule]
fn lorenz_equations_plotter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_multiple, m)?)?;
    Ok(())
}
