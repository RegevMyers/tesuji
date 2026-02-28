pub fn alternative<I, O, E, F, const LEN: usize>(functions: [F; LEN])
where
    F: Fn(I) -> Result<O, E>,
    [(); LEN - 1]: Sized,
{
    |input: I| {
        let mut last_error;

        for function in &functions {
            match function(input) {
                Ok(value) => return Ok(value),
                Err(error) => last_error = error,
            }
        }
        
        Err(last_error)
    }
}
