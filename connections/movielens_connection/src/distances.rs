// use std::collections::HashMap;

// fn manhattan_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
//     minowski_distance(first, second, 1)
// }

// fn euclidean_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
//     minowski_distance(first, second, 2)
// }


// fn minowski_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>, r: i32) -> f64{
//     let mut distance = 0.0;
//     let mut has_intersection = false;
//     for (idx, first_value) in first{
//         if let Some(second_value) = second.get(idx){
//             distance += (first_value - second_value).abs().powi(r);
//             has_intersection = true;
//         }
//     }
//     if !has_intersection{
//         return NAN;
//     }
//     return distance.powf(1./(r as f64));
// }
