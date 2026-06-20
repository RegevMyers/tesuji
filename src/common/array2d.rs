use crate::common::prolog::*;

use array2d::Array2D;

pub fn map<Ta: Clone, Tb: Clone>(array: Array2D<Ta>, function: impl Fn(&Ta) -> Tb) -> Result<Array2D<Tb>, Error> {
    let rows = array.as_row_major().iter().map(function).collect::<Vec<Tb>>();
    Ok(Array2D::from_row_major(&rows, array.num_rows(), array.num_columns())?)
}
