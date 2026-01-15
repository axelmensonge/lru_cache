//! # Bibliothèque lru_cache
//!
//! Ce crate fournit une implémentation d'un cache LRU (Least Recently Used)
//! avec enregistrement dans un fichier texte.
//!
//! Le module `cache` définit la structure `Cache` et le trait `CacheTrait` permettant
//! de manipuler un cache à capacité fixe. Le module `persistence` propose des utils
//! pour sauvegarder et lire le contenu du cache depuis un fichier texte.
//!
//! ## Performances
//! - get : O(1)
//! - put : O(n) pour trouver l'élément le moins récemment utilisé

pub mod cache;
pub mod persistence;
