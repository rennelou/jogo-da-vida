use ndarray::{Array2};

#[derive(Copy, Clone)]
pub enum Boundary {
    Limited,
    Circular
}

#[derive(Copy, Clone)]
pub struct Grid {
    dimensions: (usize, usize),
    boundary_method: fn (&Array2<u8>, (usize, usize), (usize, usize)) -> Array2<u8>
}

//desativa warning do _ do match boundary
#[allow(unreachable_patterns)]
impl Grid {
    pub fn new(mat: &Array2<u8>, boundary: Boundary) -> Result<Grid, &'static str> {
        let dim = mat.dim();
        if dim.0 < 3 || dim.1 < 3 {
            return Err("Calculo inválido para matrizes com linha e/ou coluna menor que 3 elementos.");
        }

        let method = match boundary {
            Boundary::Limited => limited_boundary,
            Boundary::Circular => circular_boundary,
            _ => return Err("Método para cálculo de vizinhança não definido."),
        };

        Ok(
            Grid {
                dimensions: dim,
                boundary_method: method
            }
        )
    }

    pub fn tick(self, states: &Array2<u8>) -> Array2<u8> {
        let (heigth, length) = self.dimensions;
        let get_neighborhood = self.boundary_method;
        let mut new_states = Array2::<u8>::zeros((heigth, length));
        
        for (i, row) in states.outer_iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let value = states[[i, j]];
                let neighborhood = get_neighborhood(&states, self.dimensions, (i, j));
                new_states[[i, j]] = transition(&neighborhood, value);
            }
        }
    
        return new_states;
    } 
}

// Uma celula morta com exatamente 3 vizinho nasce
// Uma celula viva com 2 ou 3 vizinho permanece viva
// De resto a celula morre ou permanece morta
fn transition(mat: &Array2<u8>, value: u8) -> u8 {

    let mut counter = mat.iter().filter(|&item| *item == 1).count(); 
    
    //retira da contagem caso a celula tenha valor 1 
    if value == 1 {
        counter -= 1;
    }

    if value == 0 && counter == 3 {
        return 1;
    }

    if value == 1 && (counter == 2 || counter == 3) {
        return 1;
    }
    
    return 0;
}

pub fn limited_boundary(mat: &Array2<u8>, dim: (usize, usize), point: (usize, usize)) -> Array2<u8> {
    let (heigth, length) = dim;
    let mut neighborhood = Array2::<u8>::zeros((heigth, length));
    let mask = [-1, 0, 1];
    
    for (i, i_val) in mask.iter().enumerate() {

        let i_tranformed = (point.0 as i32) + i_val;

        for (j, j_val) in mask.iter().enumerate() {
            
            let j_tranformed = (point.1 as i32) + j_val;

            if i_tranformed < 0 || i_tranformed >= heigth as i32 || j_tranformed < 0 || j_tranformed >= length as i32 {
                neighborhood[[i, j]] = 0;
            } else {
                neighborhood[[i, j]] = mat[[i_tranformed as usize, j_tranformed as usize]];
            }
        }    
    }
    
    return neighborhood.to_owned();
} 

pub fn circular_boundary(mat: &Array2<u8>, dim: (usize, usize), point: (usize, usize)) -> Array2<u8> {
    let (heigth, length) = dim;
    let mut neighborhood = Array2::<u8>::zeros((heigth, length));
    let mask = [-1, 0, 1];

    let tranform = |p: usize, val: i32| { return (((p as i32) + (heigth as i32) + val) % (length as i32)) as usize; };

    for (i, &i_val) in mask.iter().enumerate() {
        
        let i_circle = tranform(point.0, i_val);

        for (j, &j_val) in mask.iter().enumerate() {
            
            let j_circle = tranform(point.1, j_val);
            neighborhood[[i, j]] = mat[[i_circle, j_circle]]
        }    
    }
    
    return neighborhood.to_owned();
}

//TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn limited_boundary_test() {
        let mat = arr2(&[[1, 1, 1],
                         [1, 0, 1],
                         [1, 1, 1]]);
        let size = mat.dim();

        let result = arr2(&[[0, 0, 0],
                            [0, 1, 1],
                            [0, 1, 0]]);
        assert_eq!(result, limited_boundary(&mat, size, (0, 0)));
        
        let result = arr2(&[[0, 1, 0],
                            [1, 1, 0],
                            [0, 0, 0]]);
        assert_eq!(result, limited_boundary(&mat, size, (2, 2)));

        let result = arr2(&[[1, 1, 0],
                            [0, 1, 0],
                            [1, 1, 0]]);
        let size = mat.dim();
        assert_eq!(result, limited_boundary(&mat, size, (1, 2)));

        assert_eq!(mat, limited_boundary(&mat, size, (1, 1)));
    }

    #[test]
    fn circular_boundary_test() {
        let mat = arr2(&[[1, 1, 1],
                         [1, 0, 1],
                         [1, 1, 1]]);
        let size = mat.dim(); 
        
        let result = arr2(&[[1, 1, 1],
                            [1, 1, 1],
                            [1, 1, 0]]);
                                               
        assert_eq!(result, circular_boundary(&mat, size, (0, 0)));
        
        let result = arr2(&[[0, 1, 1],
                            [1, 1, 1],
                            [1, 1, 1]]);
        assert_eq!(result, circular_boundary(&mat, size, (2, 2)));

        let result = arr2(&[[1, 1, 1],
                            [0, 1, 1],
                            [1, 1, 1]]);
        assert_eq!(result, circular_boundary(&mat, size, (1, 2)));

        assert_eq!(mat, circular_boundary(&mat, size, (1, 1)));
    }

    #[test]
    fn transition_test() {
        let value = 0; //foco em (0, 1)
        let mat = arr2(&[[1, 0],
                         [1, 1]]);

        let expected = 1;
        assert_eq!(expected, transition(&mat, value));

        let value = 1;
        let expected = 1;
        assert_eq!(expected, transition(&mat, value));
    }

    #[test]
    fn tick_test() {
        let mat = arr2(&[[1, 1, 1],
                         [1, 0, 1],
                         [1, 1, 1]]);

        let expected = arr2(&[[1, 0, 1],
                              [0, 0, 0],
                              [1, 0, 1]]);
        let grid = Grid::new(&mat, Boundary::Limited).unwrap();
        assert_eq!(expected, grid.tick(&mat));
    }
}