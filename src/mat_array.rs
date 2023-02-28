
/// Matlab N-dimension array
pub struct MatArray<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) dims: Vec<u64>,
}

impl<'a, T> MatArray<'a, T> {
    /// Creates a new Matlab N-dimension array
    /// 
    /// The data is aligned according to and in the order of the dimension vector `dims`
    pub fn new(data: &'a [T], dims: Vec<u64>) -> Self {
        let n: u64 = dims.iter().product();
        assert_eq!(
            n,
            data.len() as u64,
            "expect {} elements, found {}",
            n,
            data.len() as u64
        );
        Self { data, dims }
    }
}
