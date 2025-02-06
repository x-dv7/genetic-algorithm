use crate::*;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn chromosome_mut(&mut self) -> &mut Chromosome;
    fn fitness(&self) -> f32;
}

pub trait IndividualFlex: Individual {
    fn create(chromosome: Chromosome,//набор хромосом особи
              life_time: usize,//время жизни особи
              changed: bool,//особь была заменена
              mut_force: usize//сила мутации
    ) -> Self;
    fn life_time(&self) ->  usize;//life_time сколько осталось жить птичке
    fn changed(&self) -> bool;//замененная птичка
    fn mut_force(&self) -> usize;//сила мутации
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
        }
    }

    fn chromosome_mut(&mut self) -> &mut Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } =>
                chromosome.iter()
                    .map(|(value, _, _, _)| value).sum(),
            Self::WithFitness { fitness } => *fitness,
        }
    }
    //
    // fn life_time(&self) -> usize {
    //     match self {
    //         Self::WithChromosome { .. } =>
    //             panic!("not supported for TestIndividual::WithChromosome"),
    //         Self::WithFitness { .. } =>
    //             panic!("not supported for TestIndividual::WithFitness"),
    //         // Self::WithAge { age } => *age,
    //     }
    // }
    // fn changed(&self) -> bool { true }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub struct FlexIndividual {
    pub fitness: f32,//насыщенность птички едой
    chromosome: Chromosome,
    life_time: usize,//life_time сколько осталось жить птичке
    changed: bool,//замененная птичка
    mut_force: usize,//сила мутации
}

#[cfg(test)]
impl Individual for FlexIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
            life_time: 0,
            changed: false,
            mut_force: 1,
        }
    }
    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }
    fn chromosome_mut(&mut self) -> &mut Chromosome {
        &mut self.chromosome
    }
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
impl IndividualFlex for FlexIndividual {
    fn create(chromosome: Chromosome, life_time: usize, changed: bool, mut_force: usize) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
            life_time,
            changed,
            mut_force,
        }
    }
    //life_time сколько осталось жить птичке
    fn life_time(&self) ->  usize {
        self.life_time
    }
    //замененная птичка
    fn changed(&self) -> bool {
        self.changed
    }
    //сила мутации
    fn mut_force(&self) -> usize {
        self.mut_force
    }
}