use lru_cache::cache::{Cache, CacheTrait};
use lru_cache::persistence::{FilePersistence, Persistence};

fn main() {
    // chemin vers le fichier pour persister cache
    let file_path = "fichiers/demo_cache.txt";

    println!("--- Initialisation cache 1 ---");
    // création d'un de Cache avec une capacité de 3 éléments
    let mut cache: Cache<String, String> = Cache::new(3);

    // Insertion de 3 éléments dans le cache
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());

    println!("Cache après insertion A, B, C: {:?}", cache.elements); //[A,B,C]

    // Récupération de l'élément A
    let _ = cache.get(&"A".to_string());
    println!("Cache après get de A : {:?}",cache.elements); //[B,C,A]

    // Insertion de l'élément D
    cache.put("D".to_string(), "value_d".to_string());
    println!("Cache après put de D : {:?}", cache.elements); //[C,A,D] B éjecté

    // Modification de l'élément C
    let old = cache.put("C".to_string(), "value_C_new".to_string());
    println!("Ancienne valeur de C: {:?}", old); // Some("value_c")
    println!("Cache après maj de C: {:?}", cache.elements); //[A,D,C]

    // Récupération de l'élément X (inexistant)
    println!("Get de X (inexistant): {:?}",cache.get(&"X".to_string())); //None

    println!("\n--- Write cache dans le fichier {} ---", file_path);
    // Écriture du cache dans le fichier
    FilePersistence::write_file(&cache, &file_path);

    println!("\n--- Lecture du cache depuis {} ---", file_path);
    // Lecture du cache depuis le fichier
    let mut loaded_cache1: Cache<String, String> = FilePersistence::read_file(3, &file_path);
    println!("Cache chargé : {:?}", loaded_cache1.elements); //[A,D,C]

    println!("\n--- Ajout de X et get de D pour changer l'ordre et éjecté A ---");
    // Insertion de l'élément X
    loaded_cache1.put("X".to_string(), "value_x".to_string());
    // Récupération de l'élément D pour changer l'ordre et éjecter A
    let _ = loaded_cache1.get(&"D".to_string());

    println!("\n--- Write cache modifié dans le fichier {} ---", file_path);
    // Écriture du cache modifié dans le fichier
    FilePersistence::write_file(&loaded_cache1, &file_path);

    println!("\n--- Lecture du cache depuis {} ---", file_path);
    // Lecture du cache depuis le fichier
    let loaded_cache2: Cache<String, String> = FilePersistence::read_file(3, &file_path);
    println!("Cache chargé: {:?}", loaded_cache2.elements); // [C,X,D]
}
