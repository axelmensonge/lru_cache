use lru_cache::cache::{Cache, CacheTrait};
use lru_cache::persistence::{FilePersistence, Persistence};
use std::fmt::Debug;
use std::fs::remove_file;
use std::hash::Hash;

fn cleanup(path: &str) {
    let _ = remove_file(path);
}

fn is_exist<K, V>(cache: &Cache<K, V>, key: &K, is_exists: bool)
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let boo: bool;
    match cache.get_elt(&key) {
        Some(_) => boo = true,
        None => boo = false,
    }
    assert_eq!(boo, is_exists);
}

fn test_elt_value<K, V>(cache: &Cache<K, V>, key: &K, value: &V)
where
    K: Hash + Eq + Clone,
    V: Clone + Eq + Debug,
{
    if let Some(elt) = cache.get_elt(&key) {
        assert_eq!(elt.value, value.clone());
    }
}

fn test_elt_index<K, V>(cache: &Cache<K, V>, key: &K, index: usize)
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    if let Some(elt) = cache.get_elt(&key) {
        assert_eq!(elt.index, index);
    }
}

#[test]
fn scenario_lru_string_cache() {
    let file_path = "fichiers/ti_string.txt";
    cleanup(file_path);

    let mut cache: Cache<String, String> = Cache::new(3);

    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());
    test_elt_value(&cache, &"A".to_string(), &"value_a".to_string());
    test_elt_value(&cache, &"B".to_string(), &"value_b".to_string());
    test_elt_value(&cache, &"C".to_string(), &"value_c".to_string());

    let _ = cache.get(&"A".to_string());
    test_elt_index(&cache, &"A".to_string(), 4);

    cache.put("D".to_string(), "value_d".to_string());
    assert_eq!(cache.get(&"B".to_string()), None);

    let old = cache.put("C".to_string(), "value_C_new".to_string());
    assert_eq!(old, Some("value_c".to_string()));
    test_elt_value(&cache, &"C".to_string(), &"value_C_new".to_string());

    FilePersistence::write_file(&cache, file_path);
    let loaded_cache: Cache<String, String> = FilePersistence::read_file(3, file_path);
    is_exist(&loaded_cache, &"A".to_string(), true);
    is_exist(&loaded_cache, &"C".to_string(), true);
    is_exist(&loaded_cache, &"D".to_string(), true);
    is_exist(&loaded_cache, &"B".to_string(), false);

    cleanup(file_path);
}

#[test]
fn scenario_lru_usize_cache() {
    let file_path = "fichiers/ti_usize.txt";
    cleanup(file_path);

    let mut cache: Cache<String, usize> = Cache::new(3);

    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3);
    test_elt_value(&cache, &"A".to_string(), &1);
    test_elt_value(&cache, &"B".to_string(), &2);
    test_elt_value(&cache, &"C".to_string(), &3);

    let _ = cache.get(&"A".to_string());
    test_elt_index(&cache, &"A".to_string(), 4);

    cache.put("D".to_string(), 4);
    assert_eq!(cache.get(&"B".to_string()), None);

    let old = cache.put("C".to_string(), 5);
    assert_eq!(old, Some(3));
    test_elt_value(&cache, &"C".to_string(), &5);

    FilePersistence::write_file(&cache, file_path);
    let loaded_cache: Cache<String, usize> = FilePersistence::read_file(3, file_path);
    is_exist(&loaded_cache, &"A".to_string(), true);
    is_exist(&loaded_cache, &"C".to_string(), true);
    is_exist(&loaded_cache, &"D".to_string(), true);
    is_exist(&loaded_cache, &"B".to_string(), false);

    cleanup(file_path);
}

#[test]
fn scenario_lru_bool_cache() {
    let file_path = "fichiers/ti_bool.txt";
    cleanup(file_path);

    let mut cache: Cache<String, bool> = Cache::new(3);

    cache.put("A".to_string(), true);
    cache.put("B".to_string(), false);
    cache.put("C".to_string(), true);
    test_elt_value(&cache, &"A".to_string(), &true);
    test_elt_value(&cache, &"B".to_string(), &false);
    test_elt_value(&cache, &"C".to_string(), &true);

    let _ = cache.get(&"A".to_string());
    test_elt_index(&cache, &"A".to_string(), 4);

    cache.put("D".to_string(), false);
    assert_eq!(cache.get(&"B".to_string()), None);

    let old = cache.put("C".to_string(), false);
    assert_eq!(old, Some(true));
    test_elt_value(&cache, &"C".to_string(), &false);

    FilePersistence::write_file(&cache, file_path);
    let loaded_cache: Cache<String, bool> = FilePersistence::read_file(3, file_path);
    is_exist(&loaded_cache, &"A".to_string(), true);
    is_exist(&loaded_cache, &"C".to_string(), true);
    is_exist(&loaded_cache, &"D".to_string(), true);
    is_exist(&loaded_cache, &"B".to_string(), false);

    cleanup(file_path);
}

#[test]
fn scenario_lru_multiple_writing() {
    let file_path = "fichiers/ti_cache_multiple_writing.txt";
    cleanup(file_path);

    let mut cache: Cache<String, String> = Cache::new(3);

    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());

    test_elt_value(&cache, &"A".to_string(), &"value_a".to_string());
    test_elt_value(&cache, &"B".to_string(), &"value_b".to_string());
    test_elt_value(&cache, &"C".to_string(), &"value_c".to_string());

    let _ = cache.get(&"A".to_string());
    test_elt_index(&cache, &"A".to_string(), 4);

    cache.put("D".to_string(), "value_d".to_string());
    assert_eq!(cache.get(&"B".to_string()), None);

    let old = cache.put("C".to_string(), "value_C_new".to_string());
    assert_eq!(old, Some("value_c".to_string()));
    test_elt_value(&cache, &"C".to_string(), &"value_C_new".to_string());

    FilePersistence::write_file(&cache, file_path);

    let mut loaded_cache1: Cache<String, String> = FilePersistence::read_file(3, file_path);

    is_exist(&loaded_cache1, &"A".to_string(), true);
    is_exist(&loaded_cache1, &"C".to_string(), true);
    is_exist(&loaded_cache1, &"D".to_string(), true);
    is_exist(&loaded_cache1, &"B".to_string(), false);

    test_elt_value(&loaded_cache1, &"A".to_string(), &"value_a".to_string());
    test_elt_value(&loaded_cache1, &"C".to_string(), &"value_C_new".to_string());
    test_elt_value(&loaded_cache1, &"D".to_string(), &"value_d".to_string());

    loaded_cache1.put("X".to_string(), "value_x".to_string());
    let _ = loaded_cache1.get(&"D".to_string());

    FilePersistence::write_file(&loaded_cache1, file_path);

    let loaded_cache2: Cache<String, String> = FilePersistence::read_file(3, file_path);

    is_exist(&loaded_cache2, &"A".to_string(), false);
    is_exist(&loaded_cache2, &"C".to_string(), true);
    is_exist(&loaded_cache2, &"X".to_string(), true);
    is_exist(&loaded_cache2, &"D".to_string(), true);

    test_elt_value(&loaded_cache2, &"A".to_string(), &"value_a".to_string());
    test_elt_value(&loaded_cache2, &"C".to_string(), &"value_C_new".to_string());
    test_elt_value(&loaded_cache2, &"X".to_string(), &"value_x".to_string());

    cleanup(file_path);
}
