use crate::*;
//Равномерный кроссовер.
//При это crossover учитывает возможность скрещивания 2-х родителей с разной структурой.
//Выбираем случайно и оставляем структуру 1-го родителя, а 2-го накладываем "сверху"
//это означает: "чего нет в 1-м родителе, то не добавляется из 2-го".
#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        // assert_eq!(parent_a.len(), parent_b.len()); теперь не обязательно
        let mut childs: Vec<(f32, usize, usize, usize)> = Vec::new();
        //выбираем, структуру какого родителя оставляем как базовую
        let parents =
            if rng.gen_bool(0.5) {
                (parent_a, parent_b)
            } else {
                (parent_b, parent_a)
            };
        // Создаем HashMap для быстрого поиска по layer, neuron_out, neuron_in 2-го родителя
        let parent_map: HashMap<(usize, usize, usize), usize> = parents.1
            .iter()
            .enumerate()
            .map(|(index, (_, l, o, i))| ((l, o, i), index))
            .collect();
        //при этом мы оставляем структуру 1-го родителя, а 2-го накладываем "сверху"
        //это означает: чего нет в 1-м родителе, то не добавляется из 2-го
        for (w1, l1, o1, i1) in parents.0.iter() {
            let value: (f32, usize, usize, usize);
            // Используем HashMap для поиска по l1, o1, i1
            if let Some(index) = parent_map.get(&(l1, o1, i1)) {
                // Найден элемент в parent_b
                if rng.gen_bool(0.5) {
                    value = parents.1[*index];//берем parent_b
                }
                else {
                    value = (w1, l1, o1, i1)//берем parent_a
                };
            } else {
                // Элемент не найден в parent_b, берем parent_a
                value = (w1, l1, o1, i1);
            }
            childs.push(value)
        }
        // parent_a
        //     .zip(parent_b)
        //     .map(|(a, b)| if rng.gen_bool(0.5) { a } else { b })
        //     .collect()
        Chromosome::new(childs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100)
            .map(|n| (n as f32, 0, 0, 0))
            .collect();
        let parent_b: Chromosome = (1..=100)
            .map(|n| (n as f32, 0, 0, 0))
            .collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        // Number of genes different between `child` and `parent_a`
        let diff_a = child
            .iter()
            .zip(parent_a.iter())
            .filter(|(c, p)| c != p)
            .count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child
            .iter()
            .zip(parent_b.iter())
            .filter(|(c, p)| c != p)
            .count();

        // Roughly looks like 50%, which proves that chance for picking either
        // gene is 50%
        assert_eq!(diff_a, 51);
        assert_eq!(diff_b, 51);
    }
}
