use ndarray::{Array2, s};

const SIZE: usize = 3;

pub fn tick(states: &Array2<u8>) -> Array2<u8>{
    
    let mut new_states = Array2::<u8>::zeros((SIZE, SIZE));
    
    for (i, row) in states.outer_iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let (i_start, i_end) = validate_range(i);
            let (j_start, j_end) = validate_range(j);

            let subarray = states.slice(s![i_start..i_end, j_start..j_end]);
            let value = states[[i, j]];
            new_states[[i, j]] = transition(&subarray.to_owned(), value);
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
        return 1
    } else {
        return 0
    }
}

fn validate_range(index : usize) -> (usize, usize) {
    const LIMIT_END: usize = SIZE - 1;

    let start = match index{
        0 => 0,
        other => other - 1,
    };

    let end = match index {
        LIMIT_END => SIZE,
        other => other + 2,
    };

    (start, end)
} 

//TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn validate_range_test() {
        //size = 3
        let i = 0;
        let j = 0;
        assert_eq!((0, 2), validate_range(i));
        assert_eq!((0, 2), validate_range(j));

        let i = 1;
        let j = 1;
        assert_eq!((0, 3), validate_range(i));
        assert_eq!((0, 3), validate_range(j));

        let i = 1;
        let j = 2;
        assert_eq!((0, 3), validate_range(i));
        assert_eq!((1, 3), validate_range(j));
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

        assert_eq!(expected, tick(&mat));
    }
}