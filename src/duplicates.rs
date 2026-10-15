use std::collections::HashMap;

use std::path::PathBuf;

use crate::hasher::hash_file;
use crate::models::DuplicateGroup;
use crate::models::FileInfo;

pub fn group_by_size(files: Vec<FileInfo>) -> HashMap<u64, Vec<FileInfo>> {
    let mut groups: HashMap<u64, Vec<FileInfo>> = HashMap::new();
    for file in files {
        groups.entry(file.size).or_default().push(file);
    }
    // remove all the group
    groups.retain(|_size, files| files.len() > 1);
    groups
}

pub fn find_duplicates(files: Vec<FileInfo>) -> anyhow::Result<Vec<DuplicateGroup>> {
    // 1. Raggruppa i file per dimensione
    let size_groups = group_by_size(files);

    // 2. Crea una HashMap per raggruppare gli hash
    let mut hash_map: HashMap<[u8; 32], Vec<FileInfo>> = HashMap::new();

    // 3. Attraversa i gruppi per dimensione
    for (_size, group_files) in size_groups {
        for file in group_files {
            // Calcola l'hash del singolo file
            let hash = hash_file(&file.path)?;

            // Aggiungi il file al gruppo con lo stesso hash
            hash_map.entry(hash).or_default().push(file);
        }
    }

    // 4. Prepara il vettore dei risultati
    let mut duplicates: Vec<DuplicateGroup> = Vec::new();

    // 5. Trasforma la HashMap in DuplicateGroup
    for (hash, files) in hash_map {
        // Ignora gli hash con un solo file
        if files.len() <= 1 {
            continue;
        }

        // Recupera la dimensione prima di consumare files
        let size = files[0].size;

        // Trasforma Vec<FileInfo> in Vec<PathBuf>
        let paths: Vec<PathBuf> = files.into_iter().map(|file| file.path).collect();

        // Crea il gruppo finale
        duplicates.push(DuplicateGroup {
            hash,
            size,
            files: paths,
        });
    }

    Ok(duplicates)
}
