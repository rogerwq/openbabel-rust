pub fn cxx_vector_into_vector<T: cxx::vector::VectorElement + Clone>(vec_cxx: &cxx::Vector<T>) -> Vec<T> {
    vec_cxx.iter().cloned().collect()
}