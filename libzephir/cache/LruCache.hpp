#ifndef ZEPHIR_LRUCACHE_HPP
#define ZEPHIR_LRUCACHE_HPP

#include <list>
#include <map>
#include <tuple>

// a cache which evicts the least recently used item when it is full
template<class K, class V>
class LruCache {
    typedef K key_type;
    typedef V value_type;
    typedef std::list<key_type> list_type;
    typedef std::map<
            key_type,
            std::pair<value_type, typename list_type::iterator>
    > map_type;

    map_type m_map;
    list_type m_list;
    size_t m_capacity;

    void evict()
    {
        // evict item from the end of most recently used list
        typename list_type::iterator i = --m_list.end();
        m_map.erase(*i);
        m_list.erase(i);
    }

public:
    LruCache(size_t capacity)
        : m_capacity(capacity) { }

    size_t size() const
    {
        return m_map.size();
    }

    size_t capacity() const
    {
        return m_capacity;
    }

    bool empty() const
    {
        return m_map.empty();
    }

    bool contains(const key_type &key)
    {
        return m_map.find(key) != m_map.end();
    }

    void insert(const key_type &key, const value_type &value)
    {
        typename map_type::iterator i = m_map.find(key);
        if(i == m_map.end()){
            // insert item into the cache, but first check if it is full
            if(size() >= m_capacity){
                // cache is full, evict the least recently used item
                this->evict();
            }

            // insert the new item
            m_list.push_front(key);
            m_map[key] = std::make_pair(value, m_list.begin());
        }
    }

    std::optional<value_type> get(const key_type &key)
    {
        // lookup value in the cache
        typename map_type::iterator i = m_map.find(key);
        if(i == m_map.end()){
            // value not in cache
            return std::nullopt;
        }

        // return the value, but first update its place in the most
        // recently used list
        typename list_type::iterator j = i->second.second;
        if(j != m_list.begin()){
            // move item to the front of the most recently used list
            m_list.erase(j);
            m_list.push_front(key);

            // update iterator in map
            j = m_list.begin();
            const value_type &value = i->second.first;
            m_map[key] = std::make_pair(value, j);

            // return the value
            return std::make_optional(value);
        } else {
            // the item is already at the front of the most recently
            // used list so just return it
            return std::make_optional(i->second.first);
        }
    }

    void clear()
    {
        m_map.clear();
        m_list.clear();
    }
};

#endif //ZEPHIR_LRUCACHE_HPP
