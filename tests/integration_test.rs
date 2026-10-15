use dupply::duplicates::find_duplicates;
use dupply::report::calculate_stats;
use dupply::scanner::scan_directory;

use std::fs;

#[test]
fn complete_duplicate_scan() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;

    // 1. Crea cinque file di prova
    let path_a = dir.path().join("a.txt");
    let path_b = dir.path().join("b.txt");
    let path_c = dir.path().join("c.txt");
    let path_d = dir.path().join("d.txt");
    let path_e = dir.path().join("./e.txt");

    let content = "Hello";
    fs::write(&path_a, content)?;
    fs::write(&path_b, content)?;
    fs::write(&path_c, content)?;
    fs::write(&path_e, content)?;
    //only one different
    fs::write(&path_d, "contents")?;

    // 2. Esegui scan_directory()
    let files = scan_directory(dir.path())?;

    // 3. Esegui find_duplicates()
    let scanned = files.len();
    //files.retain(|file| file.size >= min_size);
    let analyzed = files.len();
    let duplicates = find_duplicates(files)?;

    // 4. Esegui calculate_stats()
    let stats = calculate_stats(scanned, analyzed, &duplicates);

    // 5. Controlla i risultati
    assert_eq!(stats.scanned, 5);
    assert_eq!(stats.analyzed, 5);
    assert_eq!(stats.duplicate_groups, 1);
    assert_eq!(stats.duplicate_copies, 3);
    assert_eq!(stats.reclaimable_bytes, 15);

    Ok(())
}
