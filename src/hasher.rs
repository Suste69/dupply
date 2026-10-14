use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn hash_file(path: &Path) -> anyhow::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut hasher: Sha256 = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        // 1. Leggi i byte nel buffer
        let bytes_read = file.read(&mut buffer)?;
        hasher.update(&buffer[..bytes_read]);

        // 2. Se sono stati letti 0 byte, esci dal ciclo
        if bytes_read == 0 {
            break;
        }
    }
    let result: [u8; 32] = hasher.finalize().into();

    Ok(result)
}

// Aprire il file usando File::open().
// Creare un oggetto Sha256 con Sha256::new().
// Creare un buffer di 8192 byte.
// Leggere il file a blocchi utilizzando read().
// Passare soltanto i byte effettivamente letti all'hash tramite update().
// Al termine, ottenere il risultato con finalize().
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn identical_files_have_same_hash() -> anyhow::Result<()> {
        // 1. Crea due file temporanei
        let dir = tempfile::tempdir()?;
        let path_a = dir.path().join("a.txt");
        let path_b = dir.path().join("b.txt");

        // 2. Scrivi lo stesso contenuto in entrambi
        fs::write(&path_a, b"Hello Rust")?;
        fs::write(&path_b, b"Hello Rust")?;

        // 3. Calcola i rispettivi hash
        let x = hash_file(&path_a)?;
        let y = hash_file(&path_b)?;

        // 4. Verifica che siano uguali
        assert_eq!(x, y);
        Ok(())
    }
    #[test]
    fn different_files_have_different_hash() -> anyhow::Result<()> {
        // 1. Crea due file temporanei
        let dir = tempfile::tempdir()?;
        let path_a = dir.path().join("a.txt");
        let path_b = dir.path().join("b.txt");

        fs::write(&path_a, b"Hello Rust")?;
        fs::write(&path_b, b"Hello Html")?;

        // 3. Calcola i rispettivi hash
        let x = hash_file(&path_a)?;
        let y = hash_file(&path_b)?;

        // 4. Verifica che siano uguali
        assert_ne!(x, y);
        Ok(())
    }
    #[test]
    fn large_files_hash_correctly() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path_a = dir.path().join("a.txt");
        let path_b = dir.path().join("b.txt");

        // 2. Scrivi lo stesso contenuto in entrambi
        let mut content = vec![42u8; 20_000];
        fs::write(&path_a, &content)?;
        fs::write(&path_b, &content)?;

        // 3. Calcola i rispettivi hash
        let x = hash_file(&path_a)?;
        let y = hash_file(&path_b)?;

        // 4. Verifica che siano uguali
        assert_eq!(x, y);

        // 5. Modifica un byte dopo la posizione 8192
        content[15_000] = 99;
        fs::write(&path_b, &content)?;
        // 6. Ricalcola l'hash del file modificato
        let y = hash_file(&path_b)?;

        // 7. Verifica che i due hash siano diversi
        assert_ne!(x, y);

        Ok(())
    }
}
