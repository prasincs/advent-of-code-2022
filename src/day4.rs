
use std::vec;

use crate::lines_from_file;
#[derive(Debug,Clone,Copy)]
struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    fn contains(&self, another : Range) -> bool {
        self.begin <= another.begin && self.end >= another.end
    }
    fn overlaps(&self, another: Range) -> bool {
        for i in self.begin..self.end+1 {
            for j in another.begin..another.end+1 {
                if i == j {
                    return true
                }
            }
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn range_check() {
        use crate::day4::Range;
        let r = Range{begin: 2, end: 8 };
        let r2 = Range{begin: 3, end: 7};
        assert!(r.contains(r2));
    }

    #[test]
    fn range_overlap_check(){
        use crate::day4::Range;
        let r = Range{begin: 5, end: 7 };
        let r2 = Range{begin: 7, end: 9};
        assert!(r.overlaps(r2));
        let r = Range{begin: 2, end: 4 };
        let r2 = Range{begin: 6, end: 8};
        assert!(!r.overlaps(r2));
    }
}

/// .
/*
Every section has a unique ID number, and each Elf is assigned a range of section IDs.
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
*/
fn part_one(lines: Vec<String>){
    let mut fully_contains = 0;
    for line in lines {
        let mut ranges : Vec<Range> = vec![];
        let items: Vec<&str> = line.split(",").collect();
        for item in items {
            let range_nums: Vec<&str> = item.split("-").collect();
            let begin: i32 = range_nums.get(0).unwrap().parse().unwrap();
            let end: i32 = range_nums.get(1).unwrap().parse().unwrap();
            ranges.push(Range{begin:begin,end:end});
        }
        if ranges.get(0).unwrap().contains(*ranges.get(1).unwrap()) ||
        ranges.get(1).unwrap().contains(*ranges.get(0).unwrap())
        {
            fully_contains +=1;
        }
    }
    println!("{}", fully_contains);
}

fn part_two(lines: Vec<String>){
   let mut overlaps = 0;
    for line in lines {
        let mut ranges : Vec<Range> = vec![];
        let items: Vec<&str> = line.split(",").collect();
        for item in items {
            let range_nums: Vec<&str> = item.split("-").collect();
            let begin: i32 = range_nums.get(0).unwrap().parse().unwrap();
            let end: i32 = range_nums.get(1).unwrap().parse().unwrap();
            ranges.push(Range{begin:begin,end:end});
        }
        //ranges.get(1).unwrap().overlaps(*ranges.get(0).unwrap()) 
        if ranges.get(0).unwrap().overlaps(*ranges.get(1).unwrap()) 
        {
            overlaps +=1;
        }
    }
    println!("{}", overlaps);
}

fn count_fully_containing(ranges: Vec<Range>) -> i32 {
    let mut fully_contains = 0;
    for (i, r1) in ranges.iter().enumerate(){
        for (j, r2) in ranges.iter().enumerate(){
            if i == j { continue};
            println!("R1:{:?},R2:{:?},R2 in R1: {:?}",r1,r2, r1.contains(*r2));
            if r1.contains(*r2) {
                fully_contains+=1;
            }
        }
    }
    return fully_contains;
}

pub fn run(){
    let lines = lines_from_file("./inputs/day-4");
    part_one(lines.clone());
    part_two(lines.clone());
}