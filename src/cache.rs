//! Module gérant un cache LRU (Least Recently Used).
//!
//! Ce module définit une structure `Cache` qui stocke des paires clé-valeur
//! dont la paire la plus ancienne (least recently used) est éjecté
//! lorsque la capacité maximale du cache est dépassée.
//! Un index global (`max_index`) est mis à jour à chaque `get` et `put` pour
//! déterminer l'élément le moins récemment utilisé.

use std::collections::HashMap;
use std::hash::Hash;

/// # Exemple
///
/// ```rust
/// use lru_cache::cache::{Cache, CacheTrait};
///
/// // Création d'un cache avec une capacité de 2 éléments
/// let mut cache: Cache<String, String> = Cache::new(2);
/// cache.put("A".to_string(), "1".to_string());
/// cache.put("B".to_string(), "2".to_string());
/// // cache = {A:1, B:2}
/// assert_eq!(cache.get(&"A".to_string()), Some(&"1".to_string()));
/// // Get de A : son index est mis à jour (cache = {B:2, A:1})
/// cache.put("C".to_string(), "3".to_string());
/// // Capacité de 2 dépassée, l'élément B (le moins récemment utilisé) est éjecté
/// // cache = {A:1, C:3}
/// assert_eq!(cache.get(&"B".to_string()), None);
/// assert_eq!(cache.get(&"C".to_string()), Some(&"3".to_string()));
/// ```
///

/// Cette structure représente un élément stocké dans le cache LRU.
///
/// `Element` contient la valeur ainsi qu'un index indiquant sa position dans l'ordre LRU.
#[derive(Debug, Clone)]
pub struct Element<V> {
    pub index: usize,
    pub value: V,
}

/// Cache LRU (Least Recently Used) générique associant des clés de type `K` à des valeurs de type `V`.
///
/// Le cache a une capacité fixe (`size`). Lorsqu'on insère un nouvel élément au-delà de cette capacité,
/// l'élément le moins récemment utilisé (celui avec l'`index` le plus petit) est automatiquement évincé.
///
/// Les opérations `get` et `put` mettent à jour l'index interne (`max_index`) pour maintenir l'ordre LRU.
///
/// # Exemple
///
/// ```rust
/// use lru_cache::cache::{Cache, CacheTrait};
///
/// // Création d'un cache avec une capacité de 2 éléments
/// let mut cache: Cache<String, String> = Cache::new(2);
/// cache.put("A".to_string(), "1".to_string());
/// cache.put("B".to_string(), "2".to_string());
/// // cache = {A:{index:0, value:"1"}, B:{index:1, value:"2"}
/// assert_eq!(cache.get(&"A".to_string()), Some(&"1".to_string()));
/// // Get de A : son index est mis à jour (cache = {B:{index:1, value:"2"}, A:{index:2, value:"1"})
/// cache.put("C".to_string(), "3".to_string());
/// // Capacité de 2 dépassée, l'élément B (le moins récemment utilisé) est éjecté
/// // cache = {A:{index:2, value:"1"}, C:{index:3, value:"3"}
/// assert_eq!(cache.get(&"B".to_string()), None);
/// assert_eq!(cache.get(&"C".to_string()), Some(&"3".to_string()));
/// ```
///
#[derive(Debug, Clone)]
pub struct Cache<K, V> {
    pub elements: HashMap<K, Element<V>>,
    pub size: usize,
    pub max_index: usize,
}

/// Trait définissant les opérations principales d'un cache LRU.
///
/// Fournit les méthodes pour créer un cache, insérer (`put`) et récupérer (`get`) des éléments,
/// ainsi qu'une méthode `get_elt` pour obtenir un élément complet sans mettre à jour l'ordre LRU.
/// Cette dernière méthode `get_elt` est uniquement utilisée pour des tests.
pub trait CacheTrait<K, V> {
    /// Crée un nouveau cache LRU avec une capacité maximale donnée.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    ///
    /// let cache: Cache<String, String> = Cache::new(3);
    /// assert_eq!(cache.size, 3);
    /// assert_eq!(cache.max_index, 0);
    /// assert!(cache.elements.is_empty());
    /// ```
    fn new(size: usize) -> Self;

    /// Récupère la valeur associée à la clé spécifiée, si elle existe.
    /// Cette opération met à jour l'index LRU de l'élément respecter l'ordre LRU.
    /// Renvoie `None` si la clé n'existe pas dans le cache.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// let mut cache = Cache::new(2);
    /// cache.put("X".to_string(), 10);
    /// cache.put("Y".to_string(), 20);
    /// // cache = {X:{index:1, value:10}, Y:{index:2, value:20}}
    /// // Après le get, l'index de X passe de 1 à 3 pour passer devant Y d'index 2
    /// assert_eq!(cache.get(&"X".to_string()), Some(&10));
    /// // La clef A n'existe pas, None est retourné et aucun index n'est mis à jour
    /// assert_eq!(cache.get(&"A".to_string()), None);
    /// ```
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Insère ou met à jour une paire clé-valeur dans le cache.
    ///
    /// - Si la clé existe déjà, la valeur est mise à jour et l'index de l'élément est incrémenté.
    ///   La valeur précédente est alors retournée (`Some(ancienne_valeur)`).
    /// - Si la clé n'existe pas et que le cache est plein, l'élément le moins récemment utilisé
    ///   (celui avec l'index le plus petit) est éjecté.
    /// - L'ajout d'un nouvel élément (ou la mise à jour) incrémente toujours l'index global.
    ///
    /// Retourne `None` si on ajoute un nouvel élément, ou `Some(ancienne_valeur)` si on met à jour une valeur existante.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// let mut cache = Cache::new(2);
    /// assert_eq!(cache.put("A".to_string(), 1), None);
    /// assert_eq!(cache.put("B".to_string(), 2), None);
    /// // cache = {A:{index:1, value:1}, B:{index:2, value:2}}
    /// assert_eq!(cache.put("A".to_string(), 3), Some(1));
    /// // Mise à jour de A: ancienne valeur 1 retournée
    /// // cache = {A:{index:3, value:3}, B:{index:2, value:2}}
    /// assert_eq!(cache.put("C".to_string(), 4), None);
    /// // Capacité 2 dépassée -> B (le moins récemment utilisé) est éjecté
    /// // cache = {A:{index:3, value:3}, C:{index:4, value:4}}
    /// assert!(cache.get(&"B".to_string()).is_none());
    /// ```
    fn put(&mut self, key: K, value: V) -> Option<V>;

    /// Récupère un élément complet (valeur et index) dans le cache sans mettre à jour l'ordre LRU.
    /// Cette méthode est destinée principalement aux tests unitaires.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use lru_cache::cache::{Cache, CacheTrait};
    /// let mut cache = Cache::new(2);
    /// cache.put("A".to_string(), "val".to_string());
    /// if let Some(elt) = cache.get_elt(&"A".to_string()) {
    ///     assert_eq!(elt.value, "val");
    /// }
    /// ```
    fn get_elt(&self, key: &K) -> Option<&Element<V>>;
}

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(size: usize) -> Self {
        Self {
            elements: HashMap::new(),
            size,
            max_index: 0,
        }
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        match self.elements.get_mut(&key) {
            Some(elt) => {
                let old_value = elt.value.clone();
                elt.value = value.clone();
                self.max_index += 1;
                elt.index = self.max_index;
                return Some(old_value);
            }
            None => {
                if self.elements.len() >= self.size {
                    if let Some((oldest_key, _)) =
                        self.elements.iter().min_by_key(|(_, elt)| elt.index)
                    {
                        self.elements.remove(&oldest_key.clone());
                    }
                }

                self.max_index += 1;
                self.elements.insert(
                    key,
                    Element {
                        index: self.max_index,
                        value: value,
                    },
                );
                return None;
            }
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        match self.elements.get_mut(key) {
            Some(elt) => {
                self.max_index += 1;
                elt.index = self.max_index;
                return Some(&elt.value);
            }
            None => None,
        }
    }

    fn get_elt(&self, key: &K) -> Option<&Element<V>> {
        match self.elements.get(key) {
            Some(elt) => {
                return Some(elt);
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_lru_cache_prof() {
        let mut cache = Cache::new(3);

        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Cache == [B, C, D]

        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"D"), Some(&"value_d".to_string()));

        assert_eq!(cache.get(&"B"), Some(&"value_b".to_string()));
        // Cache == [C, D, B]

        assert_eq!(cache.get(&"C"), Some(&"value_c".to_string()));
        // Cache == [D, B, C]

        assert_eq!(cache.get(&"X"), None);

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        assert_eq!(cache.get(&"B"), None);
        assert_eq!(cache.get(&"D"), None);
    }

    #[test]
    fn test_new_cache_vide() {
        let cache: Cache<String, String> = Cache::new(3);
        assert!(cache.elements.is_empty());
        assert_eq!(cache.size, 3);
        assert_eq!(cache.max_index, 0);
    }

    #[test]
    fn test_cache_usize_type() {
        let mut cache: Cache<usize, usize> = Cache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);

        assert_eq!(cache.get(&1), Some(&10));
        assert_eq!(cache.get(&2), Some(&20));
        assert_eq!(cache.get(&3), Some(&30));

        cache.put(4, 40);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&20));
        assert_eq!(cache.get(&3), Some(&30));
        assert_eq!(cache.get(&4), Some(&40));
    }

    #[test]
    fn test_cache_bool_type() {
        let mut cache: Cache<usize, bool> = Cache::new(3);
        cache.put(1, true);
        cache.put(2, false);
        cache.put(3, true);

        assert_eq!(cache.get(&1), Some(&true));
        assert_eq!(cache.get(&2), Some(&false));
        assert_eq!(cache.get(&3), Some(&true));

        cache.put(4, false);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&false));
        assert_eq!(cache.get(&3), Some(&true));
        assert_eq!(cache.get(&4), Some(&false));
    }

    #[test]
    fn test_put_and_get_basic() {
        let mut cache: Cache<String, String> = Cache::new(2);

        assert_eq!(cache.put("A".to_string(), "value_a".to_string()), None);
        assert_eq!(cache.put("B".to_string(), "value_b".to_string()), None);

        test_elt_value(&cache, "A".to_string(), "value_a".to_string());
        test_elt_value(&cache, "B".to_string(), "value_b".to_string());
        test_elt_index(&cache, "A".to_string(), 1);
        test_elt_index(&cache, "B".to_string(), 2);

        assert_eq!(cache.get(&"A".to_string()), Some(&"value_a".to_string()));
        assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string()));

        test_elt_index(&cache, "A".to_string(), 3);
        test_elt_index(&cache, "B".to_string(), 4);
    }

    #[test]
    fn test_put_updates_existing_value() {
        let mut cache: Cache<String, String> = Cache::new(3);

        assert_eq!(cache.put("A".to_string(), "value_a".to_string()), None);
        assert_eq!(cache.put("B".to_string(), "value_b".to_string()), None);
        assert_eq!(
            cache.put("A".to_string(), "value_A".to_string()),
            Some("value_a".to_string())
        );

        test_elt_value(&cache, "A".to_string(), "value_A".to_string());
        test_elt_index(&cache, "A".to_string(), 3);

        assert_eq!(cache.elements.len(), 2)
    }

    #[test]
    fn test_lru_ejection() {
        let mut cache: Cache<String, String> = Cache::new(2);

        let _ = cache.put("A".to_string(), "value_a".to_string());
        let _ = cache.put("B".to_string(), "value_b".to_string());
        let _ = cache.put("C".to_string(), "value_c".to_string());

        assert_eq!(cache.get(&"A".to_string()), None);
        assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string()));
        assert_eq!(cache.get(&"C".to_string()), Some(&"value_c".to_string()));
    }

    #[test]
    fn test_lru_ejection_ordre() {
        let mut cache: Cache<String, String> = Cache::new(2);

        let _ = cache.put("A".to_string(), "value_a".to_string());
        let _ = cache.put("B".to_string(), "value_b".to_string());

        assert_eq!(cache.get(&"A".to_string()), Some(&"value_a".to_string()));

        let _ = cache.put("C".to_string(), "value_c".to_string());

        assert_eq!(cache.get(&"B".to_string()), None);
        assert_eq!(cache.get(&"A".to_string()), Some(&"value_a".to_string()));
        assert_eq!(cache.get(&"C".to_string()), Some(&"value_c".to_string()));
    }

    #[test]
    fn test_get_pas_de_clef() {
        let mut cache: Cache<String, String> = Cache::new(2);
        assert_eq!(cache.get(&"X".to_string()), None);
    }

    #[test]
    fn test_get_multiple_index() {
        let mut cache: Cache<String, String> = Cache::new(3);

        let _ = cache.put("A".to_string(), "value_a".to_string());
        let _ = cache.put("B".to_string(), "value_b".to_string());
        let _ = cache.put("C".to_string(), "value_c".to_string());

        test_elt_index(&cache, "A".to_string(), 1);
        test_elt_index(&cache, "B".to_string(), 2);
        test_elt_index(&cache, "C".to_string(), 3);

        let _ = cache.get(&"B".to_string());
        let _ = cache.get(&"B".to_string());
        let _ = cache.get(&"B".to_string());
        let _ = cache.get(&"B".to_string());
        test_elt_index(&cache, "A".to_string(), 1);
        test_elt_index(&cache, "B".to_string(), 7);
        test_elt_index(&cache, "C".to_string(), 3);

        let _ = cache.get(&"A".to_string());
        test_elt_index(&cache, "A".to_string(), 8);
        test_elt_index(&cache, "B".to_string(), 7);
        test_elt_index(&cache, "C".to_string(), 3);
    }
}
