use itertools::Itertools;
use lazy_regex::regex_captures;

pub fn run(input: &str) -> (i64, i64) {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<i64> = groups[0].split_whitespace().skip(1).map(to_i64).collect();

    let mut mappers: Vec<Mapper> = vec![];
    for &gstr in &groups[1..] {
        let mut lines = gstr.split('\n');
        let name = lines.next().unwrap().trim_end_matches(':').to_string();

        let mut start: i64 = 0;
        let mut offsets: Vec<Offset> = vec![];
        for e in lines
            .map(|s| regex_captures!(r"(\d+) (\d+) (\d+)", &s).unwrap())
            .map(|(_, dst, src, len)| MapEntry::new(dst, src, len))
            .sorted_by(|a, b| a.start.cmp(&b.start)) {
            //
            if e.start > start { offsets.push(Offset { start, adjust: 0 }); }
            offsets.push(Offset { start: e.start, adjust: e.adjust });
            start = e.end;
        }
        offsets.push(Offset { start, adjust: 0 });
        mappers.push(Mapper { name, offsets });
    }

    (solve(seeds.iter().map(|&n| (n, n)).collect(), &mappers),
     solve(seeds.iter().tuples().map(|(&n, &m)| (n, n + (m - 1))).collect(), &mappers))
}

fn solve(mut ranges: Vec<(i64, i64)>, mappers: &Vec<Mapper>) -> i64 {
    for mapper in mappers {
        ranges = ranges.iter().flat_map(|r| mapper.apply_map(r)).sorted().collect();
    }
    ranges[0].0
}

#[derive(Debug)]
struct Mapper {
    #[allow(unused)]
    name: String,
    offsets: Vec<Offset>,
}

impl Mapper {
    fn apply_map(&self, r: &(i64, i64)) -> Vec<(i64, i64)> {
        let mut mapped: Vec<(i64, i64)> = vec![];
        let (first, last) = *r;

        let mut idx = self.find_index(first);
        let end_idx = self.find_index(last);

        let mut cur = self.offsets[idx];
        let mut start = first;
        while idx < end_idx {
            idx += 1;
            let nxt = self.offsets[idx];
            mapped.push((start + cur.adjust, nxt.start + cur.adjust));
            cur = nxt;
            start = nxt.start;
        }
        mapped.push((start + cur.adjust, last + cur.adjust));
        mapped
    }

    fn find_index(&self, v: i64) -> usize {
        match self.offsets.binary_search_by_key(&v, |e| e.start) {
            Ok(i) => i,
            Err(i) => {
                assert!(i > 0);
                i - 1
            }
        }
    }
}


#[derive(Debug, Copy, Clone)]
struct Offset {
    start: i64,
    adjust: i64,
}

#[derive(Debug, Copy, Clone)]
struct MapEntry {
    start: i64,
    end: i64,
    adjust: i64,
}

impl MapEntry {
    fn new(dst: &str, src: &str, len: &str) -> MapEntry {
        let start = to_i64(src);
        let end = start + to_i64(len);
        let adjust = to_i64(dst) - start;
        MapEntry { start, end, adjust }
    }
}

fn to_i64(s: &str) -> i64 {
    u64::from_str_radix(s, 10).unwrap() as i64
}
