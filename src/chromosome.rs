use crate::*;
#[derive(Clone, Debug)]
pub struct Chromosome {
    ///состав структуры (bias or weight, layer_num, neuron_out, neuron_in)
    genes: Vec<(f32, usize, usize, usize)>,
}

impl Chromosome {
    // Конструктор
    pub fn new(genes: Vec<(f32, usize, usize, usize)>) -> Self {
        Self {genes}
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (f32, usize, usize, usize)> + '_ {
        self.genes.iter().copied()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (f32, usize, usize, usize)> {
        self.genes.iter_mut()
    }
    /// Создание представления сети из весов (в них указана топология сети),
    /// без списка функций активации
    pub fn from_weights_to_flex_net_view(
        weights: impl IntoIterator<Item = (f32, usize, usize, usize)>
    ) -> (HashMap<usize, Vec<(usize, f32)>>, Vec<Vec<usize>>) {
        //Список выходных нейронов (как ключи) со списками (номеров входных нейронов, весов) или
        //(0, смещение)
        let mut inp_links: HashMap<usize, Vec<(usize, f32)>> = HashMap::new();
        //Список номеров нейронов по слоям
        let mut neurons: Vec<Vec<usize>> = Vec::new();

        for (weight, layer_num, neuron_out, neuron_in) in weights {
            inp_links.entry(neuron_out).or_insert_with(Vec::new).push((neuron_in, weight));

            // Добавляем нейрон в соответствующий слой, если его еще нет
            if neurons.len() < layer_num {
                neurons.resize_with(layer_num, Vec::new);
            }
            let layer = &mut neurons[layer_num - 1];
            if !layer.contains(&neuron_out) {
                layer.push(neuron_out);
            }
        }
        //номера нейронов в слое д.б. по возрастанию
        neurons.iter_mut().for_each(|inner_vec| inner_vec.sort());
        (inp_links, neurons)
    }
    /// Обновление хромосомы из представления сети
    pub fn update_genes(&mut self,
                        inp_links: HashMap<usize, Vec<(usize, f32)>>,
                        neurons: Vec<Vec<usize>>) {
        //из представления сети в хромосомы
        let mut weights: Vec<(f32, usize, usize, usize)> = Vec::new();
        for (l_num, layer) in neurons.iter().enumerate() {//обход послойно
            for neuron_out in layer {//обход нейронов слоя
                if let Some(neurons_in) = inp_links.get(&(*neuron_out)) {
                    //обход входный связей и смещения нейрона
                    for (neuron_in, wt) in neurons_in {
                        weights.push((*wt, l_num+1, *neuron_out, *neuron_in));
                    }
                }
            }
        }
        //замена хромосом на новые
        self.genes.clear();
        self.genes.extend(&weights);
    }
}

impl Index<usize> for Chromosome {
    type Output = (f32, usize, usize, usize);

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<(f32, usize, usize, usize)> for Chromosome {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (f32, usize, usize, usize)>,
    {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(
            self.genes
            .iter()
            .map(|&(value, _, _, _)| value)
            .collect::<Vec<_>>()
            .as_slice(),
            other.genes
            .iter()
            .map(|&(value, _, _, _)| value)
            .collect::<Vec<_>>()
            .as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![(3.0, 0,0,0), (1.0, 0,0,0), (2.0, 0,0,0)],
        }
    }

    #[test]
    fn len() {
        assert_eq!(chromosome().len(), 3);
    }

    #[test]
    fn iter() {
        let chromosome = chromosome();
        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0], (3.0, 0,0,0));
        assert_eq!(genes[1], (1.0, 0,0,0));
        assert_eq!(genes[2], (2.0, 0,0,0));
    }

    #[test]
    fn iter_mut() {
        let mut chromosome = chromosome();

        chromosome.iter_mut().for_each(|(gene,_,_,_)| {
            *gene *= 10.0;
        });

        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0], (30.0, 0,0,0));
        assert_eq!(genes[1], (10.0, 0,0,0));
        assert_eq!(genes[2], (20.0, 0,0,0));
    }

    #[test]
    fn index() {
        let chromosome = chromosome();

        assert_eq!(chromosome[0], (3.0, 0,0,0));
        assert_eq!(chromosome[1], (1.0, 0,0,0));
        assert_eq!(chromosome[2], (2.0, 0,0,0));
    }

    #[test]
    fn from_iterator() {
        let chromosome: Chromosome = chromosome().iter().collect();

        assert_eq!(chromosome[0], (3.0, 0,0,0));
        assert_eq!(chromosome[1], (1.0, 0,0,0));
        assert_eq!(chromosome[2], (2.0, 0,0,0));
    }
}
