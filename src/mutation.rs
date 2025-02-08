//Реализованные модули алгоритмов мутации
mod gaussian;
mod flex1;
//Экспорт алгоритмов
pub use self::gaussian::*;
pub use self::flex1::*;

use crate::*;

pub trait MutationMethod {//мутация весом одной хромосомы без изменения структуры
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub trait MutationMethodFlex {//мутация весов и структуры хромосом в популяции
    fn mutate<I>(&self, rng: &mut dyn RngCore, population: &mut [I])// -> Vec<I>
    where
        I: IndividualFlex;
}