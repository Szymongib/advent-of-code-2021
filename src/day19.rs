use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

static ROTATIONS: [fn([i32; 3]) -> [i32; 3]; 24] = [
    |[x, y, z]: [i32; 3]| ([x, y, z]),
    |[x, y, z]: [i32; 3]| ([x, z, -y]),
    |[x, y, z]: [i32; 3]| ([x, -y, -z]),
    |[x, y, z]: [i32; 3]| ([x, -z, y]),
    |[x, y, z]: [i32; 3]| ([y, x, -z]),
    |[x, y, z]: [i32; 3]| ([y, z, x]),
    |[x, y, z]: [i32; 3]| ([y, -x, z]),
    |[x, y, z]: [i32; 3]| ([y, -z, -x]),
    |[x, y, z]: [i32; 3]| ([z, x, y]),
    |[x, y, z]: [i32; 3]| ([z, y, -x]),
    |[x, y, z]: [i32; 3]| ([z, -x, -y]),
    |[x, y, z]: [i32; 3]| ([z, -y, x]),
    |[x, y, z]: [i32; 3]| ([-x, y, -z]),
    |[x, y, z]: [i32; 3]| ([-x, z, y]),
    |[x, y, z]: [i32; 3]| ([-x, -y, z]),
    |[x, y, z]: [i32; 3]| ([-x, -z, -y]),
    |[x, y, z]: [i32; 3]| ([-y, x, z]),
    |[x, y, z]: [i32; 3]| ([-y, z, -x]),
    |[x, y, z]: [i32; 3]| ([-y, -x, -z]),
    |[x, y, z]: [i32; 3]| ([-y, -z, x]),
    |[x, y, z]: [i32; 3]| ([-z, x, -y]),
    |[x, y, z]: [i32; 3]| ([-z, -x, y]),
    |[x, y, z]: [i32; 3]| ([-z, y, x]),
    |[x, y, z]: [i32; 3]| ([-z, -y, -x]),
];

struct ScannerData {
    beacons: Vec<[i32; 3]>,
    beacons_distances: HashSet<usize>,
}

impl FromStr for ScannerData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons: Vec<[i32; 3]> = s
            .lines()
            .skip(1)
            .map(|line| {
                let mut values = line.split(',').map(|v| v.parse().unwrap());
                [
                    values.next().unwrap(),
                    values.next().unwrap(),
                    values.next().unwrap(),
                ]
            })
            .collect();

        let distance_set: HashSet<usize> = beacons
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| manhattan_distance(p1, p2))
            .collect();

        Ok(ScannerData {
            beacons,
            beacons_distances: distance_set,
        })
    }
}

fn merge_scanner_positions(
    absolute_locs: &mut HashSet<[i32; 3]>,
    distance_sets: &mut Vec<HashSet<usize>>,
    scanner_data: &ScannerData,
) -> Option<[i32; 3]> {
    for apply_rotation in ROTATIONS {
        let rotated: Vec<[i32; 3]> = scanner_data
            .beacons
            .iter()
            .map(|&beacon| apply_rotation(beacon))
            .collect();

        let distances = absolute_locs
            .iter()
            .cartesian_product(&rotated)
            .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);

        for [dx, dy, dz] in distances {
            let altered_rotated: Vec<[i32; 3]> = rotated
                .iter()
                .map(|[x, y, z]| [x + dx, y + dy, z + dz])
                .collect();
            let matching = altered_rotated
                .iter()
                .filter(|&v| absolute_locs.contains(v))
                .count();

            if matching >= 12 {
                absolute_locs.extend(altered_rotated);
                distance_sets.push(scanner_data.beacons_distances.clone());
                return Some([dx, dy, dz]);
            }
        }
    }
    None
}

pub fn day_19_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let (beacon_abs_pos, _) = determine_positions(&data);

    beacon_abs_pos.len()
}

pub fn day_19_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let (_, scanner_abs_pos) = determine_positions(&data);

    let mut max_dist = 0;
    for (i, pos) in scanner_abs_pos.iter().enumerate() {
        for pos2 in scanner_abs_pos.iter().skip(i + 1) {
            max_dist = max_dist.max(manhattan_distance(pos, pos2));
        }
    }

    max_dist
}

fn determine_positions(data: &str) -> (HashSet<[i32; 3]>, Vec<[i32; 3]>) {
    let mut scanners_data: Vec<ScannerData> = data
        .split("\n\n")
        .map(|scanner| ScannerData::from_str(scanner).unwrap())
        .collect();

    let first_scanner = scanners_data.remove(0);

    // This represents absolute positions of beacons.
    // We start by adding all beacons from scanner 0.
    let mut beacon_absolute_pos: HashSet<[i32; 3]> = HashSet::from_iter(first_scanner.beacons);

    let mut beacon_distance_sets = Vec::with_capacity(scanners_data.len() + 1);
    beacon_distance_sets.push(first_scanner.beacons_distances);

    let mut scanner_absolute_pos = Vec::with_capacity(scanners_data.len() + 1);
    scanner_absolute_pos.push([0, 0, 0]); // Scanner 0

    while !scanners_data.is_empty() {
        for i in (0..scanners_data.len()).rev() {
            if let Some(pos) = merge_scanner_positions(
                &mut beacon_absolute_pos,
                &mut beacon_distance_sets,
                &scanners_data[i],
            ) {
                scanner_absolute_pos.push(pos);
                scanners_data.swap_remove(i);
            }
        }
    }

    (beacon_absolute_pos, scanner_absolute_pos)
}

fn manhattan_distance([x1, y1, z1]: &[i32; 3], [x2, y2, z2]: &[i32; 3]) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
}

#[cfg(test)]
mod test {
    use crate::day19::{day_19_1, day_19_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_19", TEST_DATA);

        assert_eq!(day_19_1(&file), 79);
        assert_eq!(day_19_2(&file), 3621);
    }
}
