use std::collections::BTreeMap;
mod genetic_algs;
mod queue;
//statistically, swearing in code returns better code quality compared to not swearing in code (Jan's Thesis)
//so, i do not apologize for the language before hand.
//how the fuck do i make a recursive function to generate an entire population?
//no clue!

fn main() {
    let (mut alpha_xya, mut alpha_xxa) = 
    ((vec![0,1,0,1,1,0,1,1,0],77), (vec![1,0,1,1,1,0,1,1,1],88));
    let (mut alpha_xyb, mut alpha_xxb) = 
    ((vec![1,1,1,0,1,1,0,1,0], 77), (vec![0,0,1,1,1,0,1,0,1],55));
    let mut population_queue = BTreeMap::new();
    
    let alphachild_a = genetic_algs::crossover(&mut alpha_xya, &mut alpha_xxa);
    let alphachild_b = genetic_algs::crossover(&mut alpha_xyb, &mut alpha_xxb);

    queue::add_alphachildren(alphachild_a, alphachild_b, &mut population_queue);
    queue::add_parents(alpha_xya, alpha_xxa, &mut population_queue);
    queue::add_parents(alpha_xyb, alpha_xxb, &mut population_queue);

    for (chromosome, fitness) in &population_queue {
        println!("{chromosome:?}: {fitness}");
    }
}
