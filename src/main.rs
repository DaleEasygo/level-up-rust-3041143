fn median(mut a: Vec<f32>) -> Option<f32> {
    // Empty
    if a.is_empty() {
        return None;
    }

    // Sort
    a.sort_by(|x, y| { x.partial_cmp(y).unwrap() });

    // Odd
    if a.len() % 2 == 1 {
        let middle_index = a.len() / 2;
        return Some(a[middle_index]);
    }

    // Even
    let middle_right = a.len() / 2;
    let middle_left = (a.len() / 2) - 1;
    let avg: f32 = (a[middle_right] + a[middle_left]) / 2.0;

    Some(avg)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
