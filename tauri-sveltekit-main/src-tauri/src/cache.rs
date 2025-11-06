// Bounded cache with LRU eviction for memory management
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::hash::Hash;

pub struct CachedItem<V> {
    pub value: V,
    pub inserted_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u32,
}

pub struct BoundedCache<K, V>
where
    K: Eq + Hash + Clone,
{
    cache: HashMap<K, CachedItem<V>>,
    max_size: usize,
    ttl: Duration,
}

impl<K, V> BoundedCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // Evict expired items first
        self.evict_expired();

        if let Some(item) = self.cache.get_mut(key) {
            item.last_accessed = Instant::now();
            item.access_count += 1;
            Some(&item.value)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        // Evict if at capacity
        if self.cache.len() >= self.max_size && !self.cache.contains_key(&key) {
            self.evict_lru();
        }

        let now = Instant::now();
        self.cache.insert(
            key,
            CachedItem {
                value,
                inserted_at: now,
                last_accessed: now,
                access_count: 0,
            },
        );
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.cache.remove(key).map(|item| item.value)
    }

    fn evict_expired(&mut self) {
        let now = Instant::now();
        self.cache.retain(|_, item| {
            now.duration_since(item.inserted_at) < self.ttl
        });
    }

    fn evict_lru(&mut self) {
        // Find least recently used item
        if let Some((key, _)) = self.cache.iter()
            .min_by_key(|(_, item)| item.last_accessed)
            .map(|(k, v)| (k.clone(), v))
        {
            self.cache.remove(&key);
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn get_stats(&self) -> CacheStats {
        let total_accesses: u32 = self.cache.values().map(|item| item.access_count).sum();
        let avg_age = if !self.cache.is_empty() {
            let now = Instant::now();
            let total_age: Duration = self.cache.values()
                .map(|item| now.duration_since(item.inserted_at))
                .sum();
            total_age.as_secs() / self.cache.len() as u64
        } else {
            0
        };

        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            total_accesses,
            avg_age_seconds: avg_age,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub total_accesses: u32,
    pub avg_age_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_basic() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        cache.set("key3".to_string(), "value3".to_string());
        
        // key1 should be evicted (LRU)
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"key1".to_string()), None);
        assert_eq!(cache.get(&"key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()), Some(&"value3".to_string()));
    }

    #[test]
    fn test_cache_lru() {
        let mut cache = BoundedCache::new(2, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        // Access key1 to make it recently used
        cache.get(&"key1".to_string());
        
        // Add key3, key2 should be evicted (LRU)
        cache.set("key3".to_string(), "value3".to_string());
        
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None);
        assert_eq!(cache.get(&"key3".to_string()), Some(&"value3".to_string()));
    }

    #[test]
    fn test_cache_ttl() {
        let mut cache = BoundedCache::new(10, 1); // 1 second TTL
        cache.set("key1".to_string(), "value1".to_string());
        
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
        
        // Wait for TTL to expire
        thread::sleep(Duration::from_secs(2));
        
        // Should be evicted
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = BoundedCache::new(10, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        cache.get(&"key1".to_string());
        cache.get(&"key1".to_string());
        cache.get(&"key2".to_string());
        
        let stats = cache.get_stats();
        assert_eq!(stats.size, 2);
        assert_eq!(stats.max_size, 10);
        assert_eq!(stats.total_accesses, 3);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = BoundedCache::new(10, 300);
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }
}
