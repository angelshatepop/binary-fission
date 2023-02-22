use rand::{distributions::{Distribution, Uniform}};

pub fn mutation(child: &mut Vec<u8>) -> &mut Vec<u8>{
    const MUTATION_PROB: f64 = 0.1111111111111111; // 1|length of bitstring(9)
    for i in 0..child.len() {
        let between  = Uniform::from(0.0..=1.0);
        let mut rng = rand::thread_rng();
        if between.sample(&mut rng) <= MUTATION_PROB {
            if child[i] == 0 { child[i] = 1} else {child[i] = 0};
        }
    }
    return child;
}

pub fn calc_fitness_sequence(child: &mut Vec<u8>) -> u32{
    let fitness: u32;
    let count = child.iter().filter(|&n| *n == 1).count();
    match count {
        1 => fitness = 11,
        2 => fitness = 11*2,
        3 => fitness = 11*3,
        4 => fitness = 11*5,
        5 => fitness = 11*6,
        6 => fitness = 11*7,
        7 => fitness = 11*8,
        8 => fitness = 11*8,
        9 => fitness = 99,
        _ => todo!()
    }
    return fitness;
}

pub fn crossover(parent1: &mut (Vec<u8>, u32), parent2: &mut (Vec<u8>, u32)) -> (Vec<u8>, u32) {
    if parent1.0.len() != parent2.0.len() { 
        panic!("chromosome length not equal! parent1:{} => parent2:{} ?", parent1.0.len(), parent2.0.len())
    };
    let crossoverpoint = Uniform::from(1..=parent1.0.len()-1).sample(&mut rand::thread_rng());
    let sex: u8 = Uniform::from(0..=1).sample(&mut rand::thread_rng());

    if sex == 0 {
        let mut child = parent1.0.clone();
        let _childxy: Vec<u8> = child.splice(0..crossoverpoint, parent2.0[0..crossoverpoint].to_vec()).collect();
        mutation(&mut child);
        let fitness = calc_fitness_sequence(&mut child);

        return (child, fitness);
    }
    else {
        let mut child = parent2.0.clone();
        let _childxx: Vec<u8> = child.splice(0..crossoverpoint, parent1.0[0..crossoverpoint].to_vec()).collect();
        mutation(&mut child);
        let fitness = calc_fitness_sequence(&mut child);

        return (child, fitness);
    };
}