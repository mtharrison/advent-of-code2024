use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiskBlock {
    Free,
    File(usize),
}

#[derive(Debug)]
pub struct FreeRegion {
    start: usize,
    len: usize,
}

#[derive(Debug)]
pub struct File {
    id: usize,
    start: usize,
    len: usize,
}

#[derive(Debug)]
pub struct Disk {
    blocks: Vec<DiskBlock>,
    first_free_idx: usize,
    last_block: usize,
}

impl Disk {
    pub fn new(blocks: Vec<DiskBlock>, first_free_idx: usize, last_block: usize) -> Self {
        Disk {
            blocks,
            first_free_idx,
            last_block,
        }
    }

    pub fn defrag_simple(&mut self) {
        while self.first_free_idx < self.last_block {
            self.blocks.swap(self.first_free_idx, self.last_block);
            while let DiskBlock::File(_) = self.blocks[self.first_free_idx] {
                self.first_free_idx += 1;
            }
            while let DiskBlock::Free = self.blocks[self.last_block] {
                self.last_block -= 1;
            }
        }
    }

    pub fn defrag_files(&mut self) {
        let mut files = self.files();
        let mut free_regions = self.free_regions();
        let last_file_id = files.last().unwrap().id;

        for i in (0..last_file_id + 1).rev() {
            let file = files.iter_mut().find(|f| f.id == i).unwrap();
            let file_start = file.start;
            let file_size = file.len;

            let region = free_regions
                .iter_mut()
                .find(|r| r.len >= file_size && r.start < file_start);

            if let Some(r) = region {
                self.swap_multiple(r.start, file_start, file_size);
                file.start = r.start;
                r.start = file_start;
                free_regions = self.free_regions();
            }
        }
    }

    pub fn checksum(&self) -> i64 {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                DiskBlock::Free => None,
                DiskBlock::File(id) => Some((id * i) as i64),
            })
            .sum::<i64>()
    }

    fn swap_multiple(&mut self, i: usize, j: usize, count: usize) {
        for k in 0..count {
            self.blocks.swap(i + k, j + k);
        }
    }

    fn free_regions(&self) -> Vec<FreeRegion> {
        let mut regions = Vec::new();
        let mut start = 0;
        let mut len = 0;
        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                DiskBlock::Free => {
                    if len == 0 {
                        start = i;
                    }
                    len += 1;
                }
                DiskBlock::File(_) => {
                    if len > 0 {
                        regions.push(FreeRegion { start, len });
                        len = 0;
                    }
                }
            }
        }
        if len > 0 {
            regions.push(FreeRegion { start, len });
        }
        regions
    }

    fn files(&self) -> Vec<File> {
        let mut files = Vec::new();
        let mut start = 0;
        let mut len = 0;
        let mut id = 0;
        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                DiskBlock::File(file_id) => {
                    if len == 0 {
                        start = i;
                        id = *file_id;
                    }
                    if *file_id != id {
                        files.push(File { id, start, len });
                        start = i;
                        id = *file_id;
                        len = 0;
                    }
                    len += 1;
                }
                DiskBlock::Free => {
                    if len > 0 {
                        files.push(File { id, start, len });
                        len = 0;
                    }
                }
            }
        }
        if len > 0 {
            files.push(File { id, start, len });
        }
        files
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for block in &self.blocks {
            match block {
                DiskBlock::Free => s.push('.'),
                DiskBlock::File(id) => s.push_str(&id.to_string()),
            }
        }
        writeln!(f, "{}", s)?;
        writeln!(f, "First free: {}", self.first_free_idx)?;
        writeln!(f, "Last block: {}", self.last_block)
    }
}

impl From<String> for Disk {
    fn from(s: String) -> Self {
        let mut first_free_idx = 0;
        let mut last_block = 0;
        let mut num_blocks = 0;

        let blocks: Vec<DiskBlock> = s
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if !c.is_ascii_digit() {
                    return vec![];
                }
                let digit = c.to_digit(10).unwrap() as usize;
                if i % 2 != 0 {
                    if first_free_idx == 0 {
                        first_free_idx = num_blocks;
                    }
                    num_blocks += digit;
                    return vec![DiskBlock::Free; digit];
                }

                let file_id = i / 2;
                num_blocks += digit;
                last_block = num_blocks - 1;
                vec![DiskBlock::File(file_id); digit]
            })
            .collect();

        Disk::new(blocks, first_free_idx, last_block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse_input as util_parse;

    #[test]
    fn test_example_part1() {
        let mut disk = util_parse::<Disk>("day09", "example.txt");
        disk.defrag_simple();
        assert_eq!(disk.checksum(), 1928);
    }

    #[test]
    fn test_part1() {
        let mut disk = util_parse::<Disk>("day09", "puzzle.txt");
        disk.defrag_simple();
        assert_eq!(disk.checksum(), 6216544403458);
    }

    #[test]
    fn test_example_part2() {
        let mut disk = util_parse::<Disk>("day09", "example.txt");
        disk.defrag_files();
        assert_eq!(disk.checksum(), 2858);
    }

    #[test]
    fn test_part2() {
        let mut disk = util_parse::<Disk>("day09", "puzzle.txt");
        disk.defrag_files();
        assert_eq!(disk.checksum(), 6237075041489);
    }
}
