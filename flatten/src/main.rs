// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0

use clap::{Arg, Parser};
use cubic32::{Cubic, Point};
use euler32::CubicToEulerIter;
use flatten::{espc_integral_inv, n_subdiv_analytic};
use flatten32::flatten_offset;

use crate::euler::EulerParams;

mod arc_segment;
mod cubic32;
mod cubic_err_plot;
mod euler;
mod euler32;
mod euler_arc;
mod evolute;
mod flatten;
mod flatten32;
mod perf_graph;
#[cfg(feature = "skia-safe")]
mod skia;
mod stroke;
mod svg;

#[derive(Parser)]
enum Args {
    Arc,
    Cubic,
    CubicErr(cubic_err_plot::CubicErrPlot),
    Evolute,
    Espc,
    EstFlattenErr,
    PrimCountGraph(perf_graph::PrimCountArgs),
    Stroke,
    Svg(svg::SvgArgs),
}

fn main_est_flatten_err() {
    let th0 = 0.101;
    let th1 = 0.1;
    let offset = 0.5;
    let ep = EulerParams::from_angles(th0, th1);
    println!("{ep:?}");
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        println!(
            "{t}: {} {:?}",
            ep.eval_th(t),
            ep.eval_with_offset(t, offset)
        );
    }
    let th = 0.05;
    println!("{:?}", ep.inv_th(th));
    for i in -10..=10 {
        let dist = i as f64 / 10.0;
        let exact_err = ep.exact_flatten_err_seg(dist, 0.0, 1.0);
        println!(
            "dist={dist:.2}: est {:.6} numeric {:.6} exact {:.6}",
            ep.est_flatten_err(dist),
            ep.numeric_flatten_err(dist),
            exact_err
        );
    }
}

#[allow(unused)]
fn main_invert_espc_int() {
    for i in 0..100 {
        let x = i as f64 / 50.0;
        let _ = espc_integral_inv(x);
    }
}

#[allow(unused)]
/// Main entry point for ESPC experiments.
fn main_espc() {
    let k0 = 0.01;
    let dist = 1e-4;
    let scale = 1.0;
    let tol = 1.0;
    for i in 0..10 {
        let k1 = if i == 0 { 0.0 } else { 0.1f64.powi(i) };
        // Note: this isn't necessarily accurate when k1 and dist are tiny
        let analytic = n_subdiv_analytic(k0, k1, scale, dist, tol);
        let approx_f32 =
            flatten32::n_subdiv_robust(k0 as f32, k1 as f32, scale as f32, dist as f32, tol as f32);
        println!("{k0} {k1:.1e} {dist}: {} {}", analytic, approx_f32);
    }
}

fn main() {
    let args = Args::parse();
    match args {
        Args::Arc => euler_arc::arc_main(),
        Args::Cubic => {
            let c = Cubic::new(
                Point::new(0.0, 0.0),
                Point::new(0.0, 0.0),
                Point::new(100.0, 0.0),
                Point::new(100.0, 0.0),
            );
            let iter = CubicToEulerIter::new(c, 0.1);
            let path = flatten_offset(iter, 0.0, 0.1);
            println!("{}", path.to_svg());
        }
        Args::CubicErr(ce) => cubic_err_plot::cubic_err_plot(ce),
        Args::Espc => main_espc(),
        Args::EstFlattenErr => main_est_flatten_err(),
        Args::Evolute => evolute::euler_evolute_main(),
        Args::PrimCountGraph(args) => perf_graph::perf_graph(args),
        Args::Stroke => stroke::stroke_main(),
        Args::Svg(args) => svg::svg_main(args),
    }
}

// results with .01 accuracy
// k0 = 0.1:
//   dist = 1e-7: no soln
//   dist = 3e-7: k1 = 1e-3
//   dist = 1e-6: k1 = 1e-4
//   dist = 1e-5: k1 = 1e-5
//   dist = 1e-4: k1 = 1e-6
//   dist = 1e-3: k1 = 1e-8
//   dist = 1e-2: k1 = 1e-9

// k0 = 0.01:
//   dist = 1e-7: no soln
//   dist = 3e-7: k1 = 1e-3
//   dist = 1e-6: k1 = 1e-4
//   dist = 1e-5: k1 = 1e-5
//   dist = 1e-4: k1 = 1e-6
//   dist = 1e-3: k1 = 1e-8
//   dist = 1e-2: k1 = 1e-9

// approx rule: dist * k1 = 1e-10
