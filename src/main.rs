use std::{io, vec};

fn main() {

    let std_in  = io::stdin();
    let mut input_str = String::new();
    std_in.read_line(&mut input_str).unwrap();

    let num_cases = input_str.trim().parse::<usize>().unwrap();


    for case in 1..=num_cases {
        let (_num_modules, fun_factors, graph) = read_case(&std_in);
        let (ans, _root_path) = dfs(0,&graph, &fun_factors);
        println!("Case #{}: {}", case, ans);
    }
}

fn dfs(n: usize, graph:&Vec<Vec<usize>>, fun_factors: &Vec<usize>) -> (usize, usize) {
    if graph[n].len() == 0 
    {
        return (fun_factors[n], fun_factors[n])
    } else {
        let mut ans = 0;
        let mut worst_path = usize::MAX;
        for c in &graph[n] {
            let (total, path) = dfs(*c,graph,fun_factors);
            ans += total;
            worst_path = std::cmp::min(worst_path, path);
        }
        if worst_path < fun_factors[n] {
            ans += fun_factors[n] - worst_path;
            worst_path = fun_factors[n];
        }
        return (ans, worst_path)
    }
}


fn read_case(std_in: &io::Stdin) -> (usize, Vec<usize>,Vec<Vec<usize>>) {
    let mut input_str = String::new();
    std_in.read_line(&mut input_str).unwrap();
    let num_modules = input_str.trim().parse::<usize>().unwrap();

    input_str = String::new();
    std_in.read_line(&mut input_str).unwrap();
    let mut fun_factors: Vec<usize> = vec![0;num_modules+1];
    for (i,v) in input_str.trim().split_whitespace().into_iter().enumerate() {
        fun_factors[i+1] = v.parse::<usize>().unwrap();
    }
  

    input_str = String::new();
    std_in.read_line(&mut input_str).unwrap();
    let mut pointers: Vec<usize> = vec![0; num_modules+1];
    for (i,v) in input_str.trim().split_whitespace().into_iter().enumerate() {
        pointers[i+1] = v.parse::<usize>().unwrap();
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); num_modules+1];
    for i in 1..=num_modules {
        graph[pointers[i]].push(i);
    }
    (num_modules, fun_factors, graph)
}