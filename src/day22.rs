use std::fmt::Formatter;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn day_22_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let cuboids = read_cuboids(&data);

    let initialization_area = Cuboid {
        is_on: false,
        x: Range::new(-50, 50),
        y: Range::new(-50, 50),
        z: Range::new(-50, 50),
    };
    find_volume(cuboids, Some(initialization_area))
}

pub fn day_22_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let cuboids = read_cuboids(&data);
    find_volume(cuboids, None)
}

fn find_volume(cuboids: Vec<Cuboid>, area: Option<Cuboid>) -> usize {
    let mut processed: Vec<Cuboid> = vec![];
    let mut volume = 0;

    for cube in cuboids {
        let cube = match &area {
            None => cube,
            Some(area) => match cube.intersection(area, cube.is_on) {
                None => continue,
                Some(intersect) => intersect,
            },
        };

        for i in 0..processed.len() {
            let overlap = {
                let prev_cube = &processed[i];
                match cube.intersection(prev_cube, !prev_cube.is_on) {
                    Some(intersect) => intersect,
                    None => continue,
                }
            };
            if overlap.is_on {
                volume += overlap.number_of_points();
            } else {
                volume -= overlap.number_of_points();
            }
            processed.push(overlap);
        }
        if cube.is_on {
            volume += cube.number_of_points();
            processed.push(cube.clone())
        }
    }

    volume
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Range {
    min: i32,
    max: i32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right): (i32, i32) = s[2..]
            .split_once("..")
            .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
            .unwrap();

        Ok(Range::new(left, right))
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}..{}", self.min, self.max))
    }
}

impl Range {
    fn new(boundary_1: i32, boundary_2: i32) -> Range {
        Range {
            min: boundary_1.min(boundary_2),
            max: boundary_1.max(boundary_2),
        }
    }

    fn intersects(&self, other: &Range) -> bool {
        !(self.max < other.min || self.min > other.max)
    }

    fn length(&self) -> usize {
        (self.max - self.min).abs() as usize + 1
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Cuboid {
    is_on: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn intersects(&self, other: &Cuboid) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    fn intersection(&self, other: &Cuboid, intersection_on: bool) -> Option<Cuboid> {
        if !self.intersects(other) {
            None
        } else {
            Some(Cuboid {
                is_on: intersection_on,
                x: Range::new(self.x.min.max(other.x.min), self.x.max.min(other.x.max)),
                y: Range::new(self.y.min.max(other.y.min), self.y.max.min(other.y.max)),
                z: Range::new(self.z.min.max(other.z.min), self.z.max.min(other.z.max)),
            })
        }
    }

    fn number_of_points(&self) -> usize {
        self.x.length() * self.y.length() * self.z.length()
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on_off, ranges) = s.split_once(' ').unwrap();

        let is_on = match on_off {
            "on" => true,
            _ => false,
        };

        let mut ranges = ranges
            .split(',')
            .map(|range| Range::from_str(range).unwrap());

        Ok(Cuboid {
            is_on,
            x: ranges.next().unwrap(),
            y: ranges.next().unwrap(),
            z: ranges.next().unwrap(),
        })
    }
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "on: {},  x: {},  y: {},  x: {}",
            self.is_on, self.x, self.y, self.z
        ))
    }
}

fn read_cuboids(data: &str) -> Vec<Cuboid> {
    data.lines().map(|l| Cuboid::from_str(l).unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::day22::{day_22_1, day_22_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const TEST_DATA_2: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    const TEST_DATA_3: &str = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_22", TEST_DATA);
        let file_2 = temp_file_with_content("day_22_2", TEST_DATA_2);
        let file_3 = temp_file_with_content("day_22_3", TEST_DATA_3);

        assert_eq!(day_22_1(&file), 39);
        assert_eq!(day_22_1(&file_2), 590784);
        assert_eq!(day_22_1(&file_3), 474140);
        assert_eq!(day_22_2(&file_3), 2758514936282235);
    }
}
