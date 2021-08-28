//! Barebones implementation of the [COCOMO](https://en.wikipedia.org/wiki/COCOMO) algorithm; rewrote from the [`scc`](https://github.com/boyter/scc/) project

/// Basic COCOMO Params from Boehm
///
/// - Organic: A software project is said to be an organic type if the team size required is adequately small, the
/// problem is well understood and has been solved in the past and also the team members have a nominal experience
/// regarding the problem.
/// - Semi-detached:  A software project is said to be a Semi-detached type if the vital characteristics such as team-size,
/// experience, knowledge of the various programming environment lie in between that of organic and Embedded.
/// The projects classified as Semi-Detached are comparatively less familiar and difficult to develop compared to
/// the organic ones and require more experience and better guidance and creativity. Eg: Compilers or
/// different Embedded Systems can be considered of Semi-Detached type.
/// - Embedded: A software project with requiring the highest level of complexity, creativity, and experience
/// requirement fall under this category. Such software requires a larger team size than the other two models
/// and also the developers need to be sufficiently experienced and creative to develop such complex models.
const PROJECT_TYPE: [(f64, f64, f64, f64); 3] = [
    (2.4, 1.05, 2.5, 0.38), // organic
    (3.0, 1.12, 2.5, 0.35), // semi-detached
    (3.6, 1.20, 2.5, 0.32), // embedded
];

/// Average known american wage
const AVG_WAGE: usize = 60_000;

/// Default project type used for debugging
const DPT: usize = 1;

/// Returns the estimated cost in dollars, the estimated months of effort and the amount of humans required to complete
pub fn calc(sloc: usize) -> (usize, f64, f64) {
    let effort = estimate_effort(sloc);
    let schedule_months = estimate_schedule_months(effort);

    (
        estimate_cost(effort) as usize,
        schedule_months,
        effort / schedule_months,
    )
}

/// Calculates the cost in dollars applied using generic COCOMO weighted values based on the average american yearly wage
fn estimate_cost(effort_applied: f64) -> f64 {
    effort_applied * (AVG_WAGE / 12) as f64 * 2.4
}

/// Calculates the effort applied using generic COCOMO weighted values
fn estimate_effort(sloc: usize) -> f64 {
    PROJECT_TYPE[DPT].0 * (sloc as f64).powf(PROJECT_TYPE[DPT].1) * 1.0
}

/// Estimates the effort in months based on the result from the [estimate_effort] function
fn estimate_schedule_months(effort_applied: f64) -> f64 {
    PROJECT_TYPE[DPT].2 * effort_applied.powf(PROJECT_TYPE[DPT].3)
}
