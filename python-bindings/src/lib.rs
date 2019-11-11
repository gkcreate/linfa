use linfa_k_means as linfa_impl;
use ndarray_rand::rand::SeedableRng;
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand_isaac::Isaac64Rng;

#[pyfunction]
fn k_means(
    n_clusters: usize,
    // (n_observations, n_features)
    observations: &PyArray2<f64>,
    tolerance: f64,
    max_n_iterations: usize,
) -> Py<PyArray1<usize>> {
    // Prepare input
    let observations_array = observations.as_array();

    // TODO: maybe receive the seed as optinal argument?
    let mut rng = Isaac64Rng::seed_from_u64(42);

    // Execute K-means
    let centroids = linfa_impl::k_means(
        n_clusters,
        &observations_array,
        &mut rng,
        tolerance,
        max_n_iterations,
    );
    let cluster_memberships =
        linfa_impl::compute_cluster_memberships(&centroids, &observations_array);

    // Prepare output
    let gil = pyo3::Python::acquire_gil();
    cluster_memberships.to_pyarray(gil.python()).to_owned()
}

#[pymodule]
fn linfa_k_means(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(k_means))?;

    Ok(())
}
