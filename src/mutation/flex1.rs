use crate::*;
//метод мутации, при котором мутируют веса и структура линейной НС
#[derive(Clone, Debug)]
pub struct Flex1Mutation {
    chance: f32,
    coeff: f32,
    eye_cells: usize,// кол. сегментов зрения
}

impl Flex1Mutation {
    pub fn new(chance: f32,
               coeff: f32,
               eye_cells: usize,
    ) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self {
            chance,
            coeff,
            eye_cells,//для мутации разного кол. входов
        }
    }
}

impl MutationMethodFlex for Flex1Mutation {
    fn mutate<I>(&self, rng: &mut dyn RngCore, population: &mut [I])// -> Vec<I>
    where
        I: IndividualFlex {
        // let mut new_population: Vec<I> = Vec::new();
        let mut max_n_out: usize = 0;//максимальный номер нейрона по популяции
        //Список слоев сетей популяции с максимальным номером нейрона, присвоенным этому слою.
        //Имеет вид: <(слой,кол.нейронов слоя),макс.номер>. При составлении списка слоев макс.
        //значение берется для порядкового номера нейрона в слое. При добавлении нейрона в слой
        //ему присваивается макс.номер для того кол.нейронов, которое будет с ним, но у других
        //сетей при добавлении нейрона в тот же слой при том же кол.в слое присваивается тот же
        //самый макс.номер. У разных слоев для каждого кол.нейронов в слое свой макс. номер.
        let mut layer_nums: HashMap<(usize,usize), usize> = HashMap::new();
        //мутация значения гена-------------------------------------------------------
        for child in &mut *population {
            //составляем послойный список макс. нейронов,
            //для операции нужно представление сети
            let (_, neurons) =
                Chromosome::from_weights_to_flex_net_view(child.chromosome().iter());
            for (layer_num, layer) in neurons.iter().enumerate() {//послойно
                if layer_num > 0 {//не учитываем входной слой
                    for (i,n_out) in layer.iter().enumerate() {
                        //у нейрона, его порядковый номер+1, будем считать
                        //как количество нейронов в слое, если бы он был последним,
                        //пишем его номер n_out как макс. нейрон
                        layer_nums.insert((layer_num+1,i+1),*n_out);
                        //макс. нейрон по популяции
                        if *n_out > max_n_out { max_n_out = *n_out };
                    }
                }
            }
            //Мутация весов
            //(вес или bias, слой, нейрон, вх.связь или 0)
            for (mut gene,layer_num,n_out,n_in)
            in child.chromosome().iter() {
                if layer_num > 1 && //веса и смещения первого слоя не мутируются!
                    child.mut_force() > 0
                {
                    let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                    if rng.gen_bool(self.chance as _) {
                        gene += sign * self.coeff * rng.gen::<f32>();//мутация
                    };
                };
            };
        };
        //Шансы (от вероятности мутации веса):
        let chance_n = 1.0f32; //self.chance; //добавление/удаление нейрона.
        let chance_i = 0.0f32; // self.chance //добавление/удаление входа.
        let chance_l = 1.0f32; // self.chance //добавление/удаление слоя.

        //добавление/удаление входа.------------------------------------------------
        //обходим всю популяцию
        for child in &mut *population {
            if child.mut_force() != 2 { continue; };//только для силы мутации = 2
            if child.changed() == false { continue; };//только заменяемые
            if !rng.gen_bool(chance_i as _) { continue; };//не будем ничего менять
            let eye_num: usize = rng.gen_range(1..=self.eye_cells * 2);//какой вход менять
            //(вес или bias, слой, нейрон, вх.связь или 0)
            for (mut gene,layer_num,n_out,n_in)
            in child.chromosome().iter() {
                if layer_num != 1 { continue }//входы только на 1-м слое
                if n_out != eye_num { continue }//только выбранный вход

                if rng.gen_bool(0.5) {
                    gene = 1.0;//добавление входа
                } else {
                    gene = 0.0;//удаление входа
                };
            }
        }
        //добавление/удаление нейрона.----------------------------------------------
        //обходим всю популяцию
        for child in &mut *population {
            if child.mut_force() != 2 { continue; };//только для силы мутации = 2
            if child.changed() == false { continue; };//только заменяемые
            if !rng.gen_bool(chance_n as _) { continue; };//не будем ничего менять
            //для операции нужно представление сети
            let (mut inp_links, mut neurons) =
                Chromosome::from_weights_to_flex_net_view(child.chromosome().iter());
            //изменяемый номер слоя, не учитываем входной и выходной слой
            let l_num = rng.gen_range(2..neurons.len());
            // let l_num = layer_nums.iter().choose(rng).copied().unwrap();
            if rng.gen_bool(0.7) {//добавление нейрона 0.5
                //изменяемый слой
                let layer_0 = &mut neurons[l_num - 1];
                //определим добавляемый номер нейрона в слое layer_0 по количеству нейронов в нём
                let mut max_n: usize = layer_0.len() + 1;//следующий нейрон в слое
                //новый нейрон в слое уже забит в список макс. нейронов?
                if let Some(n) = layer_nums.get(&(l_num, max_n)) {
                    max_n = *n;//возьмем тот номер, который есть
                } else {//новый нейрон для популяции
                    max_n_out += 1;
                    max_n = max_n_out;
                }
                if !layer_0.contains(&max_n) {
                    //добавляем нейрон, при этом надо него добавить все входные связи этого
                    //слоя и в следующий слой добавить связи к этому нейрону
                    layer_0.push(max_n);//добавляем нейрон в слой
                    layer_nums.insert((l_num,layer_0.len()), max_n);
                    //вх. связи этого нейрона
                    let num_0 = layer_0[0];//первый нейрон этого слоя, возьмем его связи
                    let links_0 = inp_links.get(&num_0).unwrap();
                    inp_links.insert(max_n, links_0.clone());//добавим вх.связи нового нейрона
                    //вых. связи от этого нейрона
                    let layer_1 = &mut neurons[l_num];//следующий слой
                    for num_1 in layer_1 {
                        //добавим связи на новый нейрон из следующего слоя
                        let links_1 = inp_links.get_mut(num_1).unwrap();
                        links_1.push((max_n, rng.gen_range(-1.0..=1.0)));
                    }
                }
            } else {//удаление нейрона
                //найдем нейрон, который надо удалить. если он последний - не трогаем
                //или удаляем слой? (если он не последний)
                //изменяемый слой
                let layer_0 = &mut neurons[l_num - 1];
                //определим удаляемый номер нейрона в слое layer_0 по количеству нейронов в нём
                let mut max_n: usize = layer_0.len();//последний нейрон в слое
                if max_n < 2 {  //надо удалять слой, т.к. это последний нейрон в слое
                    //ничего не будем делать, т.к. при удалении слоя выберется тот, где меньше
                    //нейронов
                } else {  //просто удаляем нейрон
                    max_n = layer_0.pop().unwrap();//номер последнего нейрона
                    //вх. связи этого нейрона
                    inp_links.remove(&max_n);//удалим вх.связи удаленного нейрона
                    //вых. связи от этого нейрона
                    let layer_1 = &mut neurons[l_num];//следующий слой
                    for num_1 in layer_1 {
                        //удалим связи на удаленный нейрон из следующего слоя
                        let links_1 = inp_links.get_mut(num_1).unwrap();
                        if let Some(index) =
                            links_1.iter().position(|(x,_)| *x == max_n) {
                            links_1.remove(index);
                        }
                    }
                }
            };
            //номера нейронов в слое д.б. по возрастанию
            neurons.iter_mut().for_each(|inner_vec| inner_vec.sort());
            //обновим хромосомы child
            child.chromosome_mut().update_genes(inp_links, neurons);
        };
        //добавление/удаление слоя.-------------------------------------------------
        //обходим всю популяцию
        for child in &mut *population {
            if child.mut_force() != 3 { continue; }; //только для силы мутации = 3
            if child.changed() == false { continue; };//только заменяемые
            if !rng.gen_bool(chance_l as _) { continue; }; //не будем ничего менять
            //для операции нужно представление сети
            let (mut inp_links, mut neurons) =
                Chromosome::from_weights_to_flex_net_view(child.chromosome().iter());
            if rng.gen_bool(0.7) {//добавление слоя 0.5 (удлинение хвоста сети)
                //целевой номер слоя - последний, т.е. его мы копируем
                let l_num = neurons.len();
                let layer_1 = neurons.last().unwrap();//последний слой
                //новый слой
                let mut layer_new: Vec<usize> = Vec::new();
                //обходим нейроны последнего слоя и создаем новый слой по аналогии с ним
                for (i, _) in layer_1.iter().enumerate() {
                    let mut max_n: usize = 0;//следующий нейрон в слое
                    //новый нейрон в новом слое уже забит в список макс. нейронов?
                    if let Some(n) = layer_nums.get(&(l_num+1, i+1)) {
                        max_n = *n;//возьмем тот номер, который есть
                    } else {//новый нейрон для популяции
                        max_n_out += 1;
                        max_n = max_n_out;
                    }
                    //добавляем нейрон, при этом надо него добавить все входные связи этого
                    //слоя
                    layer_new.push(max_n);//добавляем нейрон в слой
                    layer_nums.insert((l_num+1,layer_new.len()), max_n);
                    //добавим связи на последний слой из нового последнего слоя
                    let mut links_new: Vec<(usize, f32)> = Vec::new();
                    links_new.push((0, rng.gen_range(-1.0..=1.0)));//bias
                    for num_1 in layer_1 {
                        //добавим связи на новый нейрон из предыдущего слоя
                        links_new.push((*num_1, rng.gen_range(-1.0..=1.0)));
                    }
                    inp_links.insert(max_n, links_new);//добавим вх.связи нового нейрона
                }
                //за целевым (последний) слоем добавляем новый последний слой
                neurons.push(layer_new);
            } else { //удаление слоя
                //мы переносим нейроны из последующего слоя в удаляемый, при этом перестраивая
                //связи каждого нейрона и заменяя его номер.
                if neurons.len() <= 3 { continue; }
                //изменяемый номер слоя, не учитываем входной и выходной слой
                // let mut l_num = rng.gen_range(2..neurons.len());
                let mut l_num = 2;
                let mut layer_min = &neurons[1];
                for j in 2..neurons.len() {//послойно
                    let layer = &neurons[j];
                    if layer_min.len() < layer.len() {
                        l_num = j+1;//слой с мин. кол. нейронов
                        layer_min = layer;
                    }
                }
                //обход всех слоев начиная с удаляемого
                for d in l_num-1 .. neurons.len() {
                    let layer_m = &neurons[d-1];//предыдущий слой
                    let mut layer_0: Vec<usize> = Vec::new();//удаляемый слой
                    if let Some(layer_1) = neurons.get(d+1) {//последующий слой
                        //слой есть, значит есть откуда копировать.
                        //копируем в него предыдущий, при этом пересобирая его вх. связи
                        for j in 0 .. layer_1.len() {
                            //определим добавляемый номер нейрона в слое layer_0 по количеству нейронов в нём
                            let mut max_n: usize = 0;
                            //новый нейрон в слое уже забит в список макс. нейронов?
                            if let Some(n) = layer_nums.get(&(d+1, j+1)) {
                                max_n = *n;//возьмем тот номер, который есть
                            } else {//новый нейрон для популяции
                                max_n_out += 1;
                                max_n = max_n_out;
                            }
                            //добавляем нейрон, при этом надо него добавить все входные связи
                            //этого слоя, но не скопировать, а пересобрать заново
                            layer_0.push(max_n);//добавляем нейрон в слой
                            layer_nums.insert((d+1,layer_0.len()), max_n);
                            //вх. связи этого нейрона, добавим связи из предыдущего слоя
                            let mut links_0: Vec<(usize, f32)> = Vec::new();
                            links_0.push((0, rng.gen_range(-1.0..=1.0)));//bias
                            for num_m in layer_m {
                                //добавим связи на новый нейрон из предыдущего слоя
                                links_0.push((*num_m, rng.gen_range(-1.0..=1.0)));
                            }
                            inp_links.insert(max_n, links_0);//перезапишем вх.связи нейрона
                        }
                        neurons[d] = layer_0;
                    } else {
                        //последующего слоя нет, значит нечего копировать.
                        //удалим последний слой
                        neurons.remove(d);
                    }
                }
            }
            //номера нейронов в слое д.б. по возрастанию
            neurons.iter_mut().for_each(|inner_vec| inner_vec.sort());
            //обновим хромосомы child
            child.chromosome_mut().update_genes(inp_links, neurons);
        }
    }
}
