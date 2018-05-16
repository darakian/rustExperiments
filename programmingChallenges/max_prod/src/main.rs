extern crate quickcheck;
use quickcheck::{TestResult, quickcheck};


fn main() {
    let the_vec = vec![5,7,1,22,10,100, 0, 0, 0, -999];
    println!("{}", max_product_naive(the_vec.clone()));
    println!("{}", max_product_better(the_vec.clone()));

        fn prop(xs: Vec<i64>) -> TestResult {
            if xs.len() == 0 {
              return TestResult::discard()
              }
            TestResult::from_bool(max_product_naive(xs.clone()) == max_product_better(xs.clone()))
        }
        quickcheck(prop as fn(Vec<i64>) -> TestResult);
}

fn max_product_naive(input: Vec<i64>) -> i64{
    if input.len()<3{return -1}
    let mut products: Vec<(i64, i64, i64, i64)> = Vec::new();

    for (i_index, i_element) in input.iter().enumerate(){
        for (j_index, j_element) in input.iter().enumerate(){
            for (k_index, k_element) in input.iter().enumerate(){
                if i_index!=j_index && i_index!=k_index && j_index!=k_index {
                    products.push((i_element.clone(), j_element.clone(), k_element.clone(), i_element*j_element*k_element));
                }
            }
        }
    }
    return products.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3
}

fn max_product_better(mut input: Vec<i64>) -> i64{
    if input.len()<3{return -1}
    input.sort();
    //println!("{:?}", input);
    return std::cmp::max(input[0]*input[1]*input[input.len()-1], input[input.len()-1]*input[input.len()-2]*input[input.len()-3])
}
