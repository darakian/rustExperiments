fn main() {
    println!("{}", max_product_naive(vec![5,7,-1,2,10]));
    println!("{}", max_product_better(vec![5,7,-1,2,10]));
}

fn max_product_naive(input: Vec<i64>) -> i64{
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

fn max_product_better(input: Vec<i64>) -> i64{
    let mut products: Vec<(i64, i64, i64, i64)> = Vec::new();


    return products.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3
}
