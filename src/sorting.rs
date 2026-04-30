use crate::config::{Algorithm, Config, DataMode};

#[derive(Clone, Debug)]
pub struct SortFrame {
    pub values: Vec<usize>,
    pub active: Vec<usize>,
    pub message: String,
}

#[derive(Default)]
struct Recorder {
    frames: Vec<SortFrame>,
}

impl Recorder {
    fn push(&mut self, values: &[usize], active: &[usize], message: impl Into<String>) {
        self.frames.push(SortFrame {
            values: values.to_vec(),
            active: active.to_vec(),
            message: message.into(),
        });
    }
}

pub fn build_frames(config: &Config) -> Vec<SortFrame> {
    let mut values = make_values(config.size, config.data_mode);
    let mut recorder = Recorder::default();

    recorder.push(&values, &[], "Start");
    match config.algorithm {
        Algorithm::Bubble => bubble_sort(&mut values, &mut recorder),
        Algorithm::Selection => selection_sort(&mut values, &mut recorder),
        Algorithm::Insertion => insertion_sort(&mut values, &mut recorder),
        Algorithm::Quick => quick_sort(&mut values, &mut recorder),
    }
    recorder.push(&values, &[], "Fertig sortiert");

    recorder.frames
}

fn make_values(size: usize, mode: DataMode) -> Vec<usize> {
    let mut values: Vec<usize> = (1..=size).collect();

    match mode {
        DataMode::Random => shuffle(&mut values),
        DataMode::Reversed => values.reverse(),
        DataMode::NearlySorted => {
            if size > 6 {
                values.swap(1, size - 2);
                values.swap(size / 3, size / 2);
            }
        }
    }

    values
}

fn shuffle(values: &mut [usize]) {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64 ^ values.len() as u64;

    for index in (1..values.len()).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let other = (seed as usize) % (index + 1);
        values.swap(index, other);
    }
}

fn bubble_sort(values: &mut [usize], recorder: &mut Recorder) {
    let len = values.len();

    for sorted_tail in 0..len {
        for index in 0..len - sorted_tail - 1 {
            recorder.push(values, &[index, index + 1], "Vergleiche Nachbarn");
            if values[index] > values[index + 1] {
                values.swap(index, index + 1);
                recorder.push(values, &[index, index + 1], "Tausche");
            }
        }
    }
}

fn selection_sort(values: &mut [usize], recorder: &mut Recorder) {
    for start in 0..values.len() {
        let mut min_index = start;

        for index in start + 1..values.len() {
            recorder.push(values, &[min_index, index], "Suche kleinstes Element");
            if values[index] < values[min_index] {
                min_index = index;
                recorder.push(values, &[min_index], "Neues Minimum");
            }
        }

        if min_index != start {
            values.swap(start, min_index);
            recorder.push(values, &[start, min_index], "Setze Minimum nach vorne");
        }
    }
}

fn insertion_sort(values: &mut [usize], recorder: &mut Recorder) {
    for index in 1..values.len() {
        let mut current = index;

        while current > 0 && values[current - 1] > values[current] {
            recorder.push(
                values,
                &[current - 1, current],
                "Schiebe Element nach links",
            );
            values.swap(current - 1, current);
            recorder.push(values, &[current - 1, current], "Tausche");
            current -= 1;
        }
    }
}

fn quick_sort(values: &mut [usize], recorder: &mut Recorder) {
    if values.len() > 1 {
        quick_sort_range(values, 0, values.len() - 1, recorder);
    }
}

fn quick_sort_range(values: &mut [usize], low: usize, high: usize, recorder: &mut Recorder) {
    if low >= high {
        return;
    }

    let pivot = partition(values, low, high, recorder);

    if pivot > 0 {
        quick_sort_range(values, low, pivot - 1, recorder);
    }
    quick_sort_range(values, pivot + 1, high, recorder);
}

fn partition(values: &mut [usize], low: usize, high: usize, recorder: &mut Recorder) -> usize {
    let pivot_value = values[high];
    let mut smaller = low;

    recorder.push(values, &[high], format!("Pivot: {pivot_value}"));

    for index in low..high {
        recorder.push(values, &[index, high], "Vergleiche mit Pivot");
        if values[index] <= pivot_value {
            values.swap(smaller, index);
            recorder.push(values, &[smaller, index], "Links vom Pivot einsortieren");
            smaller += 1;
        }
    }

    values.swap(smaller, high);
    recorder.push(values, &[smaller, high], "Pivot platzieren");
    smaller
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn all_algorithms_sort_values() {
        for algorithm in Algorithm::all() {
            let config = Config {
                algorithm: *algorithm,
                data_mode: DataMode::Reversed,
                size: 8,
                delay: Duration::from_millis(0),
            };

            let frames = build_frames(&config);
            let final_values = &frames.last().expect("frames should exist").values;

            assert_eq!(final_values, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
            assert!(!frames.is_empty());
        }
    }

    #[test]
    fn reversed_mode_starts_with_descending_values() {
        let config = Config {
            algorithm: Algorithm::Bubble,
            data_mode: DataMode::Reversed,
            size: 6,
            delay: Duration::from_millis(0),
        };

        let frames = build_frames(&config);

        assert_eq!(
            frames.first().expect("frames should exist").values,
            vec![6, 5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn nearly_sorted_mode_only_swaps_a_few_values() {
        let config = Config {
            algorithm: Algorithm::Bubble,
            data_mode: DataMode::NearlySorted,
            size: 9,
            delay: Duration::from_millis(0),
        };

        let frames = build_frames(&config);

        assert_eq!(
            frames.first().expect("frames should exist").values,
            vec![1, 8, 3, 5, 4, 6, 7, 2, 9]
        );
    }

    #[test]
    fn random_mode_is_deterministic_for_same_size() {
        let first = make_values(10, DataMode::Random);
        let second = make_values(10, DataMode::Random);

        assert_eq!(first, second);
        assert_ne!(first, (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn frames_keep_expected_size_and_active_indices_in_range() {
        let config = Config {
            algorithm: Algorithm::Quick,
            data_mode: DataMode::Random,
            size: 10,
            delay: Duration::from_millis(0),
        };

        let frames = build_frames(&config);

        for frame in frames {
            assert_eq!(frame.values.len(), config.size);
            assert!(frame.active.iter().all(|index| *index < config.size));
        }
    }
}
