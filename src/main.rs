use rayon::prelude::* ;

fn main() {
    let res = (0..1_000_000)
                            .into_par_iter()
                            .map(|x| x * x)
                            .sum::<u64>() ;

    println!("{}", res) ;   // Out: 333332833333500000

    let v_res = [0, 1, 3, 4, 5]
                            .par_iter()
                            .map(|x| x * x)
                            .collect::<Vec<_>>() ;

    println!("{:?}", v_res) ; // [0, 1, 9, 16, 25]
}
