use crate::*;
//Набор алгоритмов для 1 шага репродуктивного плана Flex.
//В этом плане изменяются веса и структура НС.
//В основном это происходит в методе mutation.
//При это crossover учитывает возможность скрещивания 2-х родителей с разной структурой.
//Selection выбирает с большей вероятностью те особи, которые имеют большую пригодность (fitness())
pub struct GeneticFlexAlgorithm<S,M> {
    sim_generation_length: usize,//длительность 1-го цикла перед обучением, влияет на длину жизни
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: M,
}

impl<S,M> GeneticFlexAlgorithm<S,M>
where
    S: SelectionMethod,
    M: MutationMethodFlex,
{
    pub fn new(
        sim_generation_length: usize,
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: M,
    ) -> Self {
        Self {
            sim_generation_length,//для генерирования life_time
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method,
        }
    }
    //1 шаг репродуктивного плана
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: IndividualFlex + Clone,
    {
        assert!(!population.is_empty());
        let mut statistic = Statistics::new(population);
        let range = statistic.max_fitness() - statistic.min_fitness();
        // let q1 = statistic.min_fitness() + range / 3.0f32;    // ~1/3 от диапазона
        // let q2 = statistic.min_fitness() + 2.0f32 * range / 3.0f32;// ~2/3 от диапазона
        let q1 = statistic.min_fitness() + range / 4.0f32;         //~1/4 от диапазона
        let q2 = statistic.min_fitness() + 2.0f32 * range / 4.0f32;//~1/2 от диапазона
        let q3 = statistic.min_fitness() + 3.0f32 * range / 4.0f32;//~3/4 от диапазона

        //время жизни в поколениях уменьшаем для "плохих" птичек
        //при этом "хорошие" птички сохраняют свою жизнь дольше
        let mut new_population: Vec<I> = Vec::new();
        let mut ch_count: usize = 0;
        for (j, parent) in population.iter().enumerate() {
            let fitness = parent.fitness();
            let mut life_time = parent.life_time();//life_time сколько осталось жить птичке
            let mut mut_force: usize = 0;//сила мутации
            if fitness < q1 {        // ~первая ~1/4 от диапазона
                life_time -= 1;//под замену как только станет 0
                mut_force = 3;//сильная мутация - веса + удаление/добавление слоя
            } else if fitness < q2 { // ~вторая 1/4 от диапазона
                life_time -= 1;//под замену как только станет 0
                mut_force = 2;//средняя мутация - веса + удаление/добавление нейронов в слой
            } else if fitness < q3 { // ~третья 1/4 от диапазона
                life_time -= 1;//под замену как только станет 0
                mut_force = 1;//слабая мутация - только веса
            } else {                 // ~четвертая 1/4 от диапазона
                //Время жизни не меняется, т.к. Individual хорошо приспособлена
                mut_force = 0;//нет мутации
            }
            if life_time == 0 { //под замену
                let parents = self.selection_method.select(rng, population);
                let parent_a = parents.0.chromosome();
                let parent_b = parents.1.chromosome();
                // //сам оцениваемый родитель
                // let parent_a = parent.chromosome();

                let child_chromosome =
                    self.crossover_method.crossover(rng, parent_a, parent_b);

                life_time = rng.gen_range(1..=self.sim_generation_length/500);
                let new_individual = <I as IndividualFlex>::create(child_chromosome,
                                                                    life_time,
                                                                    true,
                                                                    mut_force);
                new_population.push(new_individual);
                ch_count += 1;
            } else {
                let child_chromosome = population[j].chromosome().clone();
                let new_individual = <I as IndividualFlex>::create(child_chromosome,
                                                                    life_time,
                                                                    false,
                                                                    mut_force);
                new_population.push(new_individual);
            }
        };
        //мутация структуры (2,3 и changed) и весов (1,2,3) у всей новой популяции
        self.mutation_method.mutate(rng, &mut new_population);
        statistic.set_changed_count(ch_count);
        (new_population, statistic)
    }
}