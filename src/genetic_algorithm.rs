use crate::*;

pub struct GeneticAlgorithm<S> {
    sim_generation_length: usize,//длительность 1-го цикла перед обучением
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}


impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        sim_generation_length: usize,
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            sim_generation_length,//для генерирования life_time
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let new_population = (0..population.len())
            .map(|_| {
                let parent = self.selection_method.select(rng, population);
                let parent_a = parent.0.chromosome();
                let parent_b = parent.1.chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();
        let mut statistic = Statistics::new(population);
        statistic.set_changed_count(population.len());
        (new_population, statistic)
    }

    // pub fn evolve_1<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    // where
    //     I: IndividualFlex + Clone,
    // {
    //     assert!(!population.is_empty());
    //     let mut statistic = Statistics::new(population);
    //     let range = statistic.max_fitness() - statistic.min_fitness();
    //     let q1 = statistic.min_fitness() + range / 3.0f32;    // ~1/3 от диапазона
    //     let q2 = statistic.min_fitness() + 2.0f32 * range / 3.0f32;// ~2/3 от диапазона
    //
    //     //время жизни в поколениях уменьшаем для "плохих" птичек
    //     //при этом "хорошие" птички сохраняют свою жизнь дольше
    //     let mut new_population: Vec<I> = Vec::new();
    //     let mut ch_count: usize = 0;
    //     for (j, parent) in population.iter().enumerate() {
    //         let fitness = parent.fitness();
    //         let mut life_time = parent.life_time();//life_time сколько осталось жить птичке
    //         if fitness < q1 {        // ~первая 1/3 от диапазона
    //             life_time = 0;
    //         } else if fitness < q2 { // ~1/3 - 2/3 от диапазона
    //             life_time -= 1;
    //         }
    //         if life_time == 0 { //под замену
    //             let parents = self.selection_method.select(rng, population);
    //             let parent_a = parents.0.chromosome();
    //             let parent_b = parents.1.chromosome();
    //
    //             let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
    //
    //             self.mutation_method.mutate(rng, &mut child);
    //
    //             life_time = rng.gen_range(1..=self.sim_generation_length/500);
    //             let new_child = <I as IndividualFlex>::create(child,
    //                                                           life_time,
    //                                                           true,
    //                                                           1);
    //             new_population.push(new_child);
    //             ch_count += 1;
    //         } else {
    //             let child_chromosome = population[j].chromosome().clone();
    //             let new_child = <I as IndividualFlex>::create(child_chromosome,
    //                                                           life_time,
    //                                                           false,
    //                                                           1);
    //             new_population.push(new_child);
    //         }
    //     };
    //     statistic.set_changed_count(ch_count);
    //     (new_population, statistic)
    // }
}