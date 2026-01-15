//! Module gérant la persistance (lecture/écriture) d'un cache LRU dans un fichier texte.
//!
//! Ce module définit un trait `Persistence` permettant de sauvegarder et lire
//! le contenu d'un objet `Cache` dans un fichier texte. L'implémentation `FilePersistence`
//! lit et écrit chaque paire clé-valeur sous la forme `clé:valeur` par ligne,
//! en préservant l'ordre LRU à l'aide de l'index interne à l'objet `Cache` définit dans `cache.rs`.

use crate::cache::{Cache, CacheTrait, Element};
use std::fs::{File, read_to_string, write};
use std::hash::Hash;
use std::str::FromStr;

/// Trait pour gérer la persistance d'un cache LRU dans un fichier texte.
///
/// Ce trait définit des méthodes pour lire (`read_file`) et écrire (`write_file`) le contenu d'un cache
/// depuis ou vers un fichier. Les clés et valeurs sont converties en chaînes
/// en utilisant `ToString`/`FromStr`.
pub trait Persistence<K, V> {
    /// Lit le contenu d'un cache depuis un fichier.
    ///
    /// Les paires clé-valeur sont lues ligne par ligne au format `clé:valeur`.
    /// Seules les `size` dernières lignes (les plus récentes) sont utilisées si le fichier contient plus d'entrées.
    /// Si le fichier n'existe pas, il est créé et un cache vide est retourné.
    ///
    /// # Exemple si nombre de ligne est égal à la taille du cache
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// use lru_cache::persistence::{FilePersistence, Persistence};
    /// use std::fs::{remove_file, write};
    ///
    /// // Supposez un fichier "cache.txt" contenant:
    /// // A:valeur1
    /// // B:valeur2
    /// let file_path = "fichiers/cache.txt";
    /// let _ = write(file_path, "A:valeur1\nB:valeur2\n");
    ///
    /// let mut cache: Cache<String, String> = FilePersistence::read_file(2, file_path);
    /// // cache = {cache = {A:{index:1, value:valeur1}, B:{index:2, value:valeur2}}}
    /// assert_eq!(cache.get(&"A".to_string()), Some(&"valeur1".to_string()));
    /// assert_eq!(cache.get(&"B".to_string()), Some(&"valeur2".to_string()));
    /// let _ = remove_file(file_path);
    /// ```
    ///
    /// # Exemple si nombre de ligne est inférieur à la taille du cache
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// use lru_cache::persistence::{FilePersistence, Persistence};
    /// use std::fs::{remove_file, write};
    ///
    /// // Supposez un fichier "cache.txt" contenant:
    /// // A:valeur1
    /// // B:valeur2
    /// let file_path = "fichiers/cache.txt";
    /// let _ = write(file_path, "A:valeur1\nB:valeur2\n");
    /// let mut cache: Cache<String, String> = FilePersistence::read_file(3, file_path);
    /// // cache = {cache = {A:{index:1, value:valeur1}, B:{index:2, value:valeur2}}
    /// assert_eq!(cache.get(&"A".to_string()), Some(&"valeur1".to_string()));
    /// assert_eq!(cache.get(&"B".to_string()), Some(&"valeur2".to_string()));
    /// let _ = remove_file(file_path);
    /// ```
    ///
    /// # Exemple si nombre de ligne est supérieur à la taille du cache
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// use lru_cache::persistence::{FilePersistence, Persistence};
    /// use std::fs::{remove_file, write};
    ///
    /// // Supposez un fichier "cache.txt" contenant:
    /// // A:valeur1
    /// // B:valeur2
    /// // C:valeur3
    /// let file_path = "fichiers/cache.txt";
    /// let _ = write(file_path, "A:valeur1\nB:valeur2\nC:valeur3\n");
    ///
    /// let mut cache: Cache<String, String> = FilePersistence::read_file(2, file_path);
    /// // cache = {cache = {B:{index:1, value:valeur1}, C:{index:2, value:valeur2}}
    /// assert_eq!(cache.get(&"A".to_string()), None);
    /// assert_eq!(cache.get(&"B".to_string()), Some(&"valeur2".to_string()));
    /// assert_eq!(cache.get(&"C".to_string()), Some(&"valeur3".to_string()));
    /// let _ = remove_file(file_path);
    /// ```
    fn read_file(size: usize, file_path: &str) -> Cache<K, V>
    where
        K: Eq + Hash + Clone + FromStr,
        V: Clone + FromStr;

    /// Écrit le contenu du cache dans un fichier.
    ///
    /// Les éléments sont triés par ordre d'index croissant (du plus ancien au plus récent)
    /// pour préserver l'ordre LRU. Chaque ligne du fichier aura la forme `clé:valeur`.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// use lru_cache::persistence::{FilePersistence, Persistence};
    /// use std::fs::{remove_file};
    ///
    /// let file_path = "fichiers/cache.txt";
    /// let mut cache: Cache<String, String> = Cache::new(2);
    /// cache.put("A".to_string(), "1".to_string());
    /// cache.put("B".to_string(), "2".to_string());
    /// FilePersistence::write_file(&cache, file_path);
    /// // Le fichier "cache.txt" contiendra:
    /// // A:1
    /// // B:2
    /// let _ = remove_file(file_path);
    /// ```
    fn write_file(cache: &Cache<K, V>, file_path: &str)
    where
        K: ToString + Clone,
        V: ToString + Clone;
}

/// Implémentation du trait `Persistence` pour des fichiers texte.
///
/// `FilePersistence` lit et écrit sur un fichier texte dont chaque ligne
/// contient une paire `clé:valeur`. Les clés et valeurs doivent implémenter
/// respectivement `ToString`/`FromStr`.
pub struct FilePersistence;

impl<K, V> Persistence<K, V> for FilePersistence {
    fn read_file(size: usize, file_path: &str) -> Cache<K, V>
    where
        K: Eq + Hash + Clone + FromStr,
        V: Clone + FromStr,
    {
        let mut cache = Cache::new(size);

        match read_to_string(file_path) {
            Ok(content) => {
                let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                let n = lines.len();
                if n > size {
                    lines = lines.split_off(n - size);
                }

                for line in lines {
                    if let Some((key, value)) = line.split_once(':') {
                        if let (Ok(k), Ok(v)) = (K::from_str(key), V::from_str(value)) {
                            cache.put(k, v);
                        }
                    }
                }
            }
            Err(_) => {
                let _ = File::create(file_path);
            }
        }

        cache
    }

    fn write_file(cache: &Cache<K, V>, file_path: &str)
    where
        K: ToString + Clone,
        V: ToString + Clone,
    {
        let mut elts: Vec<(K, Element<V>)> = cache
            .elements
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        elts.sort_by_key(|(_, elt)| elt.index);

        let content = elts
            .iter()
            .map(|(k, e)| format!("{}:{}", k.to_string(), e.value.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        let _ = write(file_path, content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{Cache, CacheTrait};
    use std::fs::{read_to_string, remove_file};

    fn cleanup(path: &str) {
        let _ = remove_file(path);
    }

    fn test_elt_value(cache: &Cache<String, String>, key: String, value: String) {
        if let Some(elt) = cache.get_elt(&key.to_string()) {
            assert_eq!(elt.value, value);
        }
    }

    fn test_elt_index(cache: &Cache<String, String>, key: String, value: usize) {
        if let Some(elt) = cache.get_elt(&key.to_string()) {
            assert_eq!(elt.index, value);
        }
    }

    #[test]
    fn read_file_pas_de_fichier() {
        let file_path = "fichiers/test_pas_de_fichier.txt";
        cleanup(file_path);

        let cache: Cache<String, String> = FilePersistence::read_file(3, file_path);

        assert_eq!(cache.elements.len(), 0);
        assert_eq!(cache.size, 3);
        assert!(std::path::Path::new(file_path).exists());

        cleanup(file_path);
    }

    #[test]
    fn write_read_file_cache_vide() {
        let file_path = "fichiers/test_cache_vide.txt";
        cleanup(file_path);

        let cache_write: Cache<String, String> = Cache::new(3);

        FilePersistence::write_file(&cache_write, file_path);
        let read_cache: Cache<String, String> = FilePersistence::read_file(2, file_path);

        assert_eq!(read_cache.elements.len(), 0);
        assert_eq!(read_cache.size, 2);
        assert_eq!(read_cache.max_index, 0);

        cleanup(file_path);
    }

    #[test]
    fn read_file_lines_inf_size_cache() {
        let file_path = "fichiers/test_read_lines_inf_size_cache.txt";
        cleanup(file_path);

        let _ = write(file_path, "A:value_a\nB:value_b\n");

        let cache: Cache<String, String> = FilePersistence::read_file(3, file_path);

        test_elt_value(&cache, "A".to_string(), "value_a".to_string());
        test_elt_value(&cache, "B".to_string(), "value_b".to_string());

        cleanup(file_path);
    }

    #[test]
    fn read_file_lines_sup_size_cache() {
        let file_path = "fichiers/test_read_lines_sup_size_cache.txt";
        cleanup(file_path);

        let _ = write(file_path, "A:value1\nB:value2\nC:value3\nD:value4");

        let cache: Cache<String, String> = FilePersistence::read_file(3, file_path);

        assert_eq!(cache.elements.len(), 3);
        assert_eq!(cache.elements.contains_key("A"), false);
        assert_eq!(cache.elements.contains_key("B"), true);
        assert_eq!(cache.elements.contains_key("C"), true);
        assert_eq!(cache.elements.contains_key("D"), true);

        cleanup(file_path);
    }

    #[test]
    fn write_file_bon_format() {
        let file_path = "fichiers/test_write_file.txt";
        cleanup(file_path);

        let mut cache: Cache<String, String> = Cache::new(3);
        cache.put("A".to_string(), "value_a".to_string());
        cache.put("B".to_string(), "value_b".to_string());

        FilePersistence::write_file(&cache, file_path);

        if let Ok(content) = read_to_string(file_path) {
            assert!(content.contains("A:value_a"));
            assert!(content.contains("B:value_b"));
        }

        cleanup(file_path);
    }

    #[test]
    fn write_file_index_ordre() {
        let file_path = "fichiers/test_write_ordre.txt";
        cleanup(file_path);

        let mut cache_write: Cache<String, String> = Cache::new(3);
        cache_write.put("A".to_string(), "a".to_string());
        cache_write.put("B".to_string(), "b".to_string());
        cache_write.put("C".to_string(), "c".to_string());

        let _ = cache_write.get(&"A".to_string());

        test_elt_index(&cache_write, "A".to_string(), 4);
        test_elt_index(&cache_write, "B".to_string(), 2);
        test_elt_index(&cache_write, "C".to_string(), 3);

        FilePersistence::write_file(&cache_write, file_path);
        let cache_read: Cache<String, String> = FilePersistence::read_file(3, file_path);

        test_elt_index(&cache_read, "A".to_string(), 3);
        test_elt_index(&cache_read, "B".to_string(), 1);
        test_elt_index(&cache_read, "C".to_string(), 2);

        cleanup(file_path);
    }
}
