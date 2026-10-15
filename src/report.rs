use crate::models::DuplicateGroup;

pub struct ScanStats {
    pub scanned: usize,
    pub analyzed: usize,
    pub duplicate_groups: usize,
    pub duplicate_copies: usize,
    pub reclaimable_bytes: u64,
}

pub fn calculate_stats(
    scanned: usize,
    analyzed: usize,
    duplicates: &[DuplicateGroup],
) -> ScanStats {
    let copies: usize = duplicates.iter().map(|group| group.files.len() - 1).sum();
    let bites: u64 = duplicates
        .iter()
        .map(|group| (group.files.len() as u64 - 1) * group.size)
        .sum();
    ScanStats {
        scanned,
        analyzed,
        duplicate_groups: (duplicates.len()),
        duplicate_copies: (copies),
        reclaimable_bytes: (bites),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stats() -> anyhow::Result<()> {
        use std::path::PathBuf;

        let duplicates: Vec<DuplicateGroup> = vec![
            DuplicateGroup {
                hash: [1u8; 32],
                size: 100,
                files: vec![
                    PathBuf::from("a.txt"),
                    PathBuf::from("b.txt"),
                    PathBuf::from("c.txt"),
                ],
            },
            DuplicateGroup {
                hash: [2u8; 32],
                size: 500,
                files: vec![PathBuf::from("d.txt"), PathBuf::from("e.txt")],
            },
        ];
        let stats = calculate_stats(10, 8, &duplicates);

        assert_eq!(stats.scanned, 10);
        assert_eq!(stats.analyzed, 8);
        assert_eq!(stats.duplicate_groups, 2);
        assert_eq!(stats.duplicate_copies, 3);
        assert_eq!(stats.reclaimable_bytes, 700);
        Ok(())
    }
}
