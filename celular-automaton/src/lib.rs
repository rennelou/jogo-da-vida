use ndarray::{Array2};

pub fn tick(get_neighborhood: fn (&Array2<u8>, usize, (usize, usize)) -> Array2<u8>, size: usize, states: &Array2<u8>) -> Array2<u8>{
    
    let mut new_states = Array2::<u8>::zeros((size, size));
    
    for (i, row) in states.outer_iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let value = states[[i, j]];
            let neighborhood = get_neighborhood(&states, size, (i, j));
            new_states[[i, j]] = transition(&neighborhood, value);
        }
    }

    return new_states;
}

fn transition(mat: &Array2<u8>, value: u8) -> u8 {
    let mut counter = mat.iter().filter(|&item| *item == 1).count(); 
    if value == 1 {
        counter -= 1;
    }

    if counter >= 3 {
        return 1;
    } else {
        return 0;
    }
}

pub fn zero_neighborhood(mat: &Array2<u8>, rad: usize, point: (usize, usize)) -> Array2<u8> {
    let mut neighborhood = Array2::<u8>::zeros((rad, rad));
    let mask = [-1, 0, 1];
    
    for (i, i_val) in mask.iter().enumerate() {
        
        let i_tranformed = (point.0 as i32) + *i_val;
        for (j, j_val) in mask.iter().enumerate() {
            
            let j_tranformed = (point.1 as i32) + *j_val;

            if i_tranformed < 0 || i_tranformed >= rad as i32 || j_tranformed < 0 || j_tranformed >= rad as i32 {
                neighborhood[[i, j]] = 0;
            } else {
                neighborhood[[i, j]] = mat[[i_tranformed as usize, j_tranformed as usize]];
            }
        }    
    }
    
    return neighborhood.to_owned();
} 

pub fn circular_boundary(mat: &Array2<u8>, rad: usize, point: (usize, usize)) -> Array2<u8> {
    let mut neighborhood = Array2::<u8>::zeros((rad, rad));
    let mask = [-1, 0, 1];

    let tranform = |p: usize, val: i32| { return (((p as i32) + (rad as i32) + val) % (rad as i32)) as usize; };

    for (i, i_val) in mask.iter().enumerate() {
        
        let i_circle = tranform(point.0, *i_val);
        for (j, j_val) in mask.iter().enumerate() {
            
            let j_circle = tranform(point.1, *j_val);
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
    const SIZE: usize = 3;

    #[test]
    fn validate_range_test() {
        let mat = arr2(&[[1, 1, 1],
                         [1, 0, 1],
                         [1, 1, 1]]);

        let result = arr2(&[[0, 0, 0],
                            [0, 1, 1],
                            [0, 1, 0]]);
        assert_eq!(result, zero_neighborhood(&mat, SIZE, (0, 0)));
        
        let result = arr2(&[[0, 1, 0],
                            [1, 1, 0],
                            [0, 0, 0]]);
        assert_eq!(result, zero_neighborhood(&mat, SIZE, (2, 2)));

        let result = arr2(&[[1, 1, 0],
                            [0, 1, 0],
                            [1, 1, 0]]);
        assert_eq!(result, zero_neighborhood(&mat, SIZE, (1, 2)));

        assert_eq!(mat, zero_neighborhood(&mat, SIZE, (1, 1)));
    }

    #[test]
    fn transition_test() {
        let value = 0; //foco em (0, 1)
        let mat = arr2(&[[1, 0],
                         [1, 1]]);

        let expected = 1;

        assert_eq!(expected, transition(&mat, value));
    }

    #[test]
    fn tick_test() {
        let mat = arr2(&[[1, 1, 1],
                         [1, 0, 1],
                         [1, 1, 1]]);

        let expected = arr2(&[[0, 1, 0],
                              [1, 1, 1],
                              [0, 1, 0]]);

        assert_eq!(expected, tick(zero_neighborhood, SIZE, &mat));
    }
}