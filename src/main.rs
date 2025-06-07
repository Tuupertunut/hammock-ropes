use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    iter,
    sync::{Arc, Mutex},
};

const ROPE_LENGTH: f32 = 238.;
const KNOT_LENGTH: f32 = 10.;

const TREE_CIRCUMFERENCE: f32 = 60.;
const MAX_KNOTS: usize = 3;
const MAX_INTERVAL: f32 = 34.;

const KNOT_STEPS: usize = 30;

#[derive(Clone, Copy, Debug)]
enum AttachmentType {
    P,
    U,
}

fn main() {
    let best = Arc::new(Mutex::new(None));

    for left_right_knots in (0..=MAX_KNOTS).combinations_with_replacement(2) {
        let left_knots = left_right_knots[0];
        let right_knots = left_right_knots[1];

        (1..KNOT_STEPS)
            .combinations(left_knots)
            .cartesian_product((1..KNOT_STEPS).combinations(right_knots))
            .par_bridge()
            .for_each(|(left_step_indexes, right_step_indexes)| {
                let left_rope_length_with_knots = ROPE_LENGTH - left_knots as f32 * KNOT_LENGTH;
                let right_rope_length_with_knots = ROPE_LENGTH - right_knots as f32 * KNOT_LENGTH;

                let left_knot_positions = left_step_indexes
                    .into_iter()
                    .map(|step_index| {
                        step_index as f32 * (left_rope_length_with_knots / KNOT_STEPS as f32)
                    })
                    .collect::<Vec<_>>();
                let right_knot_positions = right_step_indexes
                    .into_iter()
                    .map(|step_index| {
                        step_index as f32 * (right_rope_length_with_knots / KNOT_STEPS as f32)
                    })
                    .collect::<Vec<_>>();

                /* println!(
                    "left: {} knots at {:?}, right: {} knots at {:?}",
                    left_knots, left_knot_positions, right_knots, right_knot_positions
                ); */

                let left_knot_and_end_positions = iter::once(0.)
                    .chain(left_knot_positions.iter().copied())
                    .chain(iter::once(left_rope_length_with_knots));
                let right_knot_and_end_positions = iter::once(0.)
                    .chain(right_knot_positions.iter().copied())
                    .chain(iter::once(right_rope_length_with_knots));

                let left_valid_attachments = left_knot_and_end_positions
                    .array_combinations::<2>()
                    .filter(|&knot_pair| knot_pair[1] - knot_pair[0] >= TREE_CIRCUMFERENCE)
                    .cartesian_product([AttachmentType::P, AttachmentType::U].into_iter());
                let right_valid_attachments = right_knot_and_end_positions
                    .array_combinations::<2>()
                    .filter(|&knot_pair| knot_pair[1] - knot_pair[0] >= TREE_CIRCUMFERENCE)
                    .cartesian_product([AttachmentType::P, AttachmentType::U].into_iter());

                let attachment_combinations = left_valid_attachments
                    .cartesian_product(right_valid_attachments)
                    .map(
                        |((left_knot_pair, left_type), (right_knot_pair, right_type))| {
                            let left_length = match left_type {
                                AttachmentType::P => {
                                    left_knot_pair[1] - left_knot_pair[0] - TREE_CIRCUMFERENCE
                                }
                                AttachmentType::U => {
                                    (left_knot_pair[1] - left_knot_pair[0] - TREE_CIRCUMFERENCE)
                                        / 2.
                                }
                            };
                            let right_length = match right_type {
                                AttachmentType::P => {
                                    right_knot_pair[1] - right_knot_pair[0] - TREE_CIRCUMFERENCE
                                }
                                AttachmentType::U => {
                                    (right_knot_pair[1] - right_knot_pair[0] - TREE_CIRCUMFERENCE)
                                        / 2.
                                }
                            };

                            let attachment_length = left_length + right_length;
                            return (
                                (left_knot_pair, left_type),
                                (right_knot_pair, right_type),
                                attachment_length,
                            );
                        },
                    )
                    .sorted_by(|(_, _, attachment_length_a), (_, _, attachment_length_b)| {
                        attachment_length_a
                            .partial_cmp(attachment_length_b)
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                /* println!("attachment combinations:");
                for (
                    (left_knot_pair, left_type),
                    (right_knot_pair, right_type),
                    attachment_length,
                ) in attachment_combinations.iter()
                {
                    println!(
                        "left: {:?} {:?}, right: {:?} {:?}, length: {}",
                        left_knot_pair, left_type, right_knot_pair, right_type, attachment_length
                    );
                } */

                let mut longest_range_with_intervals = 0.;
                let mut current_range_with_intervals = 0.;
                for ((_, _, previous_attachment_length), (_, _, attachment_length)) in
                    attachment_combinations.iter().tuple_windows()
                {
                    if attachment_length - previous_attachment_length <= MAX_INTERVAL {
                        current_range_with_intervals +=
                            attachment_length - previous_attachment_length;
                    } else {
                        current_range_with_intervals = 0.;
                    }
                    longest_range_with_intervals =
                        f32::max(longest_range_with_intervals, current_range_with_intervals);
                }

                let mut best = best.lock().unwrap();
                match *best {
                    None => {
                        *best = Some((
                            left_knot_positions,
                            right_knot_positions,
                            attachment_combinations,
                            longest_range_with_intervals,
                        ))
                    }
                    Some((_, _, _, best_range)) if longest_range_with_intervals > best_range => {
                        *best = Some((
                            left_knot_positions,
                            right_knot_positions,
                            attachment_combinations,
                            longest_range_with_intervals,
                        ))
                    }
                    Some(_) => {}
                }
            });
    }

    let best = best.lock().unwrap();
    if let Some((left_knot_positions, right_knot_positions, attachment_combinations, range)) =
        &*best
    {
        println!(
            "left: {} knots at {:?}, right: {} knots at {:?}, longest acceptable range: {}",
            left_knot_positions.len(),
            left_knot_positions,
            right_knot_positions.len(),
            right_knot_positions,
            range
        );
        println!("attachment combinations:");
        for ((left_knot_pair, left_type), (right_knot_pair, right_type), attachment_length) in
            attachment_combinations.iter()
        {
            println!(
                "left: {:?} {:?}, right: {:?} {:?}, length: {}",
                left_knot_pair, left_type, right_knot_pair, right_type, attachment_length
            );
        }
    }
}
