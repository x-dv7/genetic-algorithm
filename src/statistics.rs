use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
    median_fitness: f32,
    changed_count: usize,//сколько I поменялось
    neurons_by_layer_all: HashSet<String>,//сколько слоев и нейронов в каждом слое
    max_neuron_num: usize,//номер максимального нейрона в популяции
}

impl Statistics {
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        //min-max-avg
        let len = population.len();

        let fitnesses = {
            let mut fitnesses: Vec<_> = population.iter().map(|i| i.fitness()).collect();
            fitnesses.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            fitnesses
        };

        let min_fitness = fitnesses[0];
        let max_fitness = fitnesses[len - 1];
        let avg_fitness = fitnesses.iter().sum::<f32>() / (len as f32);

        let median_fitness = if len % 2 == 0 {
            (fitnesses[len / 2 - 1] + fitnesses[len / 2]) / 2.0
        } else {
            fitnesses[len / 2]
        };

        //структура сети
        let mut neurons_by_layer_all: HashSet<String> = HashSet::new();
        for child in population {
            let mut neurons_by_layer: Vec<Vec<usize>> = Vec::new();//нейроны послойно
            for (w1,layer_num,n_out,n_in) in child.chromosome().iter() {
                //добавляем новый слой
                if neurons_by_layer.len() < layer_num {
                    neurons_by_layer.resize_with(layer_num, Vec::new);
                }
                //ссылка на слой
                let layer = &mut neurons_by_layer[layer_num - 1];
                if layer_num == 1 {
                    //для 1 слоя смотрим только n_in != 0 (не смещение),
                    //добавляем вход только если вес > 0, т.е. не удален
                    if n_in != 0 && w1 > 0.0 && !layer.contains(&n_out) {
                        layer.push(n_out);
                    }
                } else {
                    //для последующих слоев просто добавляем нейроны в слой (даже смещение)
                    if !layer.contains(&n_out) {
                        layer.push(n_out);
                    }
                }
            }
            let mut str1: String = "".to_string();
            for nl in neurons_by_layer {
                str1 = str1.add(nl.len().to_string().as_str());
                str1 = str1.add(".");
            }
            neurons_by_layer_all.insert(str1);
        }

        //максимальный номер нейрона по популяции
        let mut max_n_out: usize = 0;
        for child in population {
            let max_n: usize = child
                .chromosome()
                .iter()
                .max_by_key(|(_, _, n_out, _)| *n_out) // Находим элемент с макс. n_out
                .map(|(_, _, n_out, _)| n_out) // Возвращаем только n_out
                .unwrap_or(0);
            if max_n > max_n_out { max_n_out = max_n };
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness,
            median_fitness,
            changed_count: 0,
            neurons_by_layer_all,
            max_neuron_num: max_n_out,
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }

    pub fn median_fitness(&self) -> f32 {
        self.median_fitness
    }

    pub fn changed_count(&self) -> usize {
        self.changed_count
    }
    pub fn set_changed_count(&mut self, ch: usize) {
        self.changed_count = ch;
    }

    pub fn neurons_by_layer(&self) -> HashSet<String> { self.neurons_by_layer_all.clone() }

    pub fn max_neuron_num(&self) -> usize { self.max_neuron_num }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(10.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 10.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (10.0 + 20.0 + 30.0 + 40.0) / 4.0);
        approx::assert_relative_eq!(stats.median_fitness(), (20.0 + 30.0) / 2.0);
    }

    #[test]
    fn test_odd() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 20.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (20.0 + 30.0 + 40.0) / 3.0);
        approx::assert_relative_eq!(stats.median_fitness(), 30.0);
    }
}
