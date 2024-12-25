use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    let numeric_possible_paths = build_full(HashMap::from([
        ('7', HashMap::from([('>', '8'), ('v', '4')])),
        ('8', HashMap::from([('<', '7'), ('>', '9'), ('v', '5')])),
        ('9', HashMap::from([('<', '8'), ('v', '6')])),
        ('4', HashMap::from([('>', '5'), ('^', '7'), ('v', '1')])),
        (
            '5',
            HashMap::from([('<', '4'), ('>', '6'), ('^', '8'), ('v', '2')]),
        ),
        ('6', HashMap::from([('<', '5'), ('^', '9'), ('v', '3')])),
        ('1', HashMap::from([('>', '2'), ('^', '4')])),
        (
            '2',
            HashMap::from([('<', '1'), ('>', '3'), ('^', '5'), ('v', '0')]),
        ),
        ('3', HashMap::from([('<', '2'), ('^', '6'), ('v', 'A')])),
        ('0', HashMap::from([('>', 'A'), ('^', '2')])),
        ('A', HashMap::from([('<', '0'), ('^', '3')])),
    ]));

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+

    let directional_possible_paths = build_full(HashMap::from([
        ('^', HashMap::from([('>', 'A'), ('v', 'v')])),
        ('A', HashMap::from([('<', '^'), ('v', '>')])),
        ('<', HashMap::from([('>', 'v')])),
        ('v', HashMap::from([('<', '<'), ('>', '>'), ('^', '^')])),
        ('>', HashMap::from([('<', 'v'), ('^', 'A')])),
    ]));

    let mut directional_path_lengths = directional_possible_paths
        .iter()
        .map(|(&key, sequences)| (key, sequences[0].len()))
        .collect::<HashMap<_, _>>();

    directional_path_lengths = directional_possible_paths
        .iter()
        .map(|(&key, sequences)| {
            let min_sum = sequences
                .iter()
                .map(|sequence| sequence_sum(sequence, &directional_path_lengths))
                .min()
                .unwrap();
            (key, min_sum)
        })
        .collect();

    let numeric_path_lengths = numeric_possible_paths
        .iter()
        .map(|(&key, sequences)| {
            let min_sum = sequences
                .iter()
                .map(|sequence| sequence_sum(sequence, &directional_path_lengths))
                .min()
                .unwrap();
            (key, min_sum)
        })
        .collect::<HashMap<_, _>>();

    let result = input
        .iter()
        .map(|main_sequence| {
            let numeric_value = main_sequence[..main_sequence.len() - 1]
                .parse::<usize>()
                .unwrap();
            let sequence_len = sequence_sum(main_sequence, &numeric_path_lengths);

            numeric_value * sequence_len
        })
        .sum::<usize>();

    println!("{result}");
}

fn build_full(basic_map: HashMap<char, HashMap<char, char>>) -> HashMap<(char, char), Vec<String>> {
    let mut map = HashMap::new();
    let mut queue = vec![];
    let mut queue_index = 0;

    for key in basic_map.keys() {
        map.insert((*key, *key), vec!["".to_string()]);
        queue.push((*key, *key, "".to_string()));
    }

    while queue_index < queue.len() {
        let (start, current, sequence) = queue[queue_index].clone();
        for (key, next) in basic_map.get(&current).unwrap() {
            let next_sequence = [sequence.clone(), key.to_string()].concat();
            let sequences = map.entry((start, *next)).or_insert(vec![]);
            if sequences.len() == 0 || sequences[0].len() == next_sequence.len() {
                sequences.push(next_sequence.clone());
                queue.push((start, *next, next_sequence));
            }
        }
        queue_index += 1;
    }

    for (_, sequences) in map.iter_mut() {
        for sequence in sequences {
            sequence.push('A');
        }
    }

    map
}

fn sequence_sum(sequence: &str, path_lengths: &HashMap<(char, char), usize>) -> usize {
    let mut sum = 0;
    let mut prev = 'A';
    for current in sequence.chars() {
        sum += path_lengths.get(&(prev, current)).unwrap();
        prev = current;
    }
    sum
}
