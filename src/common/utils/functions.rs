use crate::common::log;

pub fn alternative<I, O, E, F, const LEN: usize>(functions: [F; LEN]) -> impl Fn(I) -> Result<O, E>
where
    I: Copy,
    F: Fn(I) -> Result<O, E>,
{
    if LEN == 0 {
        log::fatal("alternative called with no functions");
        panic!("alternative called with no functions");
    }

    move |input: I| {
        let mut last_error: Option<E> = None;

        for function in &functions {
            match function(input) {
                Ok(value) => return Ok(value),
                Err(error) => last_error = Some(error),
            }
        }
        
        Err(last_error.unwrap())
    }
}

