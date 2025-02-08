mod individual;
mod chromosome;
mod crossover;
mod mutation;
mod selection;
mod statistics;
mod genetic_algorithm;
mod genetic_flex_algorithm;

pub use self::individual::*;
pub use self::chromosome::*;
pub use self::crossover::*;
pub use self::mutation::*;
pub use self::selection::*;
pub use self::statistics::*;
pub use self::genetic_algorithm::*;
pub use self::genetic_flex_algorithm::*;
use rand::seq::SliceRandom;
use rand::{Rng, RngCore, seq::IteratorRandom};
use std::collections::{HashMap, HashSet};
use rand::thread_rng;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Index;
use std::ops::Add;



#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[(f32, usize, usize, usize)]) -> TestIndividual {
        TestIndividual::create(genes.iter().cloned().collect())
    }
    fn flex_individual(genes: &[(f32, usize, usize, usize)],
                       life_time: usize,
                       fitness: f32,
                       mut_force: usize,
    ) -> FlexIndividual {
        let mut ind =
            <FlexIndividual as IndividualFlex>::create(genes.iter().cloned().collect(),
                                                       life_time,
                                                       false,
                                                       mut_force);
        ind.fitness = fitness;
        ind
    }

    #[allow(clippy::excessive_precision)] // formatting the numbers differently would make the test less readable
    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            2500,
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[(0.0,0,0,0), (0.0,0,0,0), (0.0,0,0,0)]),
            individual(&[(1.0,0,0,0), (1.0,0,0,0), (1.0,0,0,0)]),
            individual(&[(1.0,0,0,0), (2.0,0,0,0), (1.0,0,0,0)]),
            individual(&[(1.0,0,0,0), (2.0,0,0,0), (4.0,0,0,0)]),
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population).0;
        }

        let expected_population = vec![
            individual(&[(4.4993515,0,0,0),  (4.564677,0,0,0),  (4.1209025,0,0,0)]),
            individual(&[(3.773639,0,0,0),   (3.4663687,0,0,0), (4.71867,0,0,0)]),
            individual(&[(4.053298,0,0,0),   (4.2940416,0,0,0), (4.2940416,0,0,0)]),
            individual(&[(4.2320604,0,0,0),  (3.8735359,0,0,0), (4.2940416,0,0,0)]),
        ];

        assert_eq!(population, expected_population);
    }

    #[test]
    fn test_flex1() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga =
            GeneticFlexAlgorithm::new(
            2500,
            RouletteWheelSelection,
            UniformCrossover,
            Flex1Mutation::new(0.5, 0.5, 9),
        );

        // vec![//(вес,слой,нейрон,вх.связь)
        //      //входной слой
        //      (0.0,1,1,0), (1.0,1,1,1), (0.0,1,2,0), (1.0,1,2,2),//1,2
        //      //первый слой - 1 нейрон. 2 входа, 1 выход
        //      (0.1,2,3,0), (0.2,2,3,1),(0.3,2,3,2),//3
        //      //второй слой - 1 нейрон. 1 вход, 1 выход
        //      (0.4,3,4,0), (0.5,3,4,3)//4
        // ];

        let mut population = vec![
            flex_individual(&[(0.0,1,1,0), (1.0,1,1,1),  (0.0,1,2,0), (1.0,1,2,2),//1,2
                (0.1,2,3,0), (0.2,2,3,1),(0.3,2,3,2),//3
                (0.4,3,4,0), (0.5,3,4,3)],//4
                            5, 10.0f32, 1),//остается
            flex_individual(&[(0.0,1,1,0), (1.0,1,1,1),  (0.0,1,2,0), (1.0,1,2,2),//1,2
                (0.1,2,3,0), (0.2,2,3,1),(0.3,2,3,2),//3
                (0.4,3,4,0), (0.5,3,4,3)],//4
                            1, 9.0f32, 2),//под замену
            flex_individual(&[(0.0,1,1,0), (1.0,1,1,1),  (0.0,1,2,0), (1.0,1,2,2),//1,2
                (0.1,2,3,0), (0.2,2,3,1),(0.3,2,3,2),//3
                (0.4,3,4,0), (0.5,3,4,3)],//4
                            1, 8.0f32, 3),//под замену
            // flex_individual(&[(0.0,1,1,0), (1.0,1,1,1),  (0.0,1,2,0), (1.0,1,2,2),//1,2
            //     (0.1,2,3,0), (0.2,2,3,1),(0.3,2,3,2),//3
            //     (0.4,3,4,0), (0.5,3,4,3)],//4
            // 5, 7.0f32),
        ];
        population = ga.evolve(&mut rng, &population).0;

        Some(population.len());
    }
}