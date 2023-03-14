/// The assumed model of computation
pub trait Model {
    /// The final program output type
    type Output;

    /// Clears a cell to 0
    fn clear(&mut self, cell: isize);
    /// Clears the data in a cell and copies it to a list of other cells
    fn mov(&mut self, cell: isize, to: Vec<isize>);
    /// Returns the final program output
    fn finish(self) -> Self::Output;
}
