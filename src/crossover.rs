//Реализованные модули алгоритмов кроссовера
mod uniform;
//Экспорт алгоритмов
pub use self::uniform::*;

use crate::*;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
