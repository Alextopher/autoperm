/// The assumed model of computation
pub trait Model {
    /// The final program output type
    type Output;

    /// Sets the top of the stack to a given index
    /// 
    /// This function should generate a no-op. It is used just to communicate the starting state
    fn start(&mut self, cell: isize);
    /// Clears a cell to 0
    fn clear(&mut self, cell: isize);
    /// Clears the data in a cell and copies it to a list of other cells
    fn mov(&mut self, cell: isize, to: Vec<isize>);
    /// Changes the top of the stack to a given index
    fn top(&mut self, cell: isize);
    /// Returns the final program output.
    fn finish(self) -> Self::Output;
}
