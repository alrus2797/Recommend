use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const NAN: f64 = -f64::INFINITY;

fn get_union(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> Vec::<i32>{
    let mut intersections = Vec::new();
    for (key, first_value) in first{
        if let Some(second_value) = second.get(key){
            intersections.push(*key);
        }
        else{
            intersections.push(*key);
        }
    }
    intersections
}


pub fn manhattan_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
    minowski_distance(first, second, 1)
}

pub fn euclidean_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
    minowski_distance(first, second, 2)
}


pub fn minowski_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>, r: i32) -> f64{
    let mut distance = 0.0;
    let mut has_intersection = false;
    for (idx, first_value) in first{
        if let Some(second_value) = second.get(idx){
            distance += (first_value - second_value).abs().powi(r);
            has_intersection = true;
        }
    }
    if !has_intersection{
        return NAN;
    }
    return distance.powf(1./(r as f64));
}

pub fn pearson_aproximation(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
    let mut x_times_y =  0.;
    let mut sum_x = 0.;
    let mut sum_y = 0.;
    let mut sum_pow_x = 0.;
    let mut sum_pow_y = 0.;
    let mut n = 0;

    for (key, val1) in first{
        if let Some(val2) = second.get(key){
            x_times_y   += val1 * val2;
            sum_x       += val1; 
            sum_y       += val2;
            sum_pow_x   += val1.powi(2);
            sum_pow_y   += val2.powi(2);
            n+=1;
        }
    }
    let first_factor = x_times_y - (sum_x * sum_y / n as f64);
    let first_square = (sum_pow_x - sum_x.powi(2) / n as f64).sqrt();
    let second_square = (sum_pow_y - sum_y.powi(2) / n as f64).sqrt();
    let second_factor = first_square * second_square;
    if n == 0 || second_factor == 0.0{
        return NAN;
    }

    return first_factor / second_factor;
}

pub fn cosine_simmilarity(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
    let mut norm_x = 0.;
    let mut norm_y = 0.;
    let mut p_mult = 0.;

    for (key, val1) in first{
        if let Some(val2) = second.get(key){
            p_mult += val1 * val2;
            norm_x += val1.powi(2);
            norm_y += val2.powi(2);
        }
    }
    norm_x = norm_x.sqrt();
    norm_y = norm_y.sqrt();

    return p_mult / (norm_x * norm_y);
}

pub fn jaccard_index(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64 {
    let mut I = 0;   
    for key in first.keys(){
        if second.contains_key(key){
            I += 1;
        }
    }
    let U = (first.keys().len() - I) + (second.keys().len() - I) + I;
    if U == 0{
        return NAN;
    }
    return (I as f64) / (U as f64);
}

pub fn jaccard_distance(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>) -> f64{
    let jacc_index = jaccard_index(first, second);
    if jacc_index == NAN{
        return NAN;
    }
    else{
        return 1. - jacc_index;
    }
}

pub fn adjusted_cosine_simmilarity(first: &HashMap<i32, f64>, second: &HashMap<i32, f64>, averages:  &HashMap<i32, f64>) -> f64{
    let mut numerator   = 0.0;
    let mut first_factor    = 0.0;
    let mut second_factor   = 0.0;
    // print!("Numerador: ");
    for (key, val1) in first{
        if let Some(val2) = second.get(key){
            numerator   += (val1 - averages[key]) * (val2 - averages[key]);
            // print!("({:.4} - {:.4}) * ({:.4} - {:.4}) + ", val1, averages[key], val2, averages[key]);
            first_factor += (val1 - averages[key]).powi(2);
            second_factor+= (val2 - averages[key]).powi(2);
        }
    }
    // println!("\n");
    first_factor    = first_factor.sqrt();
    second_factor   = second_factor.sqrt();
    return numerator/(first_factor * second_factor)
}
