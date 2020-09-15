#pragma once

#include <memory>
#include <string>
#include "../cache/LruCache.hpp"
#include "../identity/Identity.hpp"
#include "../identity/Group.hpp"
#include "../Policy.hpp"

namespace libzephir {
    using namespace cache;
    using namespace identity;

    namespace storage {
        typedef struct cache {
            LruCache<std::string, std::shared_ptr<Identity>> identities = LruCache<std::string, std::shared_ptr<Identity>>(128);
            LruCache<std::string, std::shared_ptr<Group>> groups = LruCache<std::string, std::shared_ptr<Group>>(128);
            LruCache<std::string, std::vector<std::string>> groupsPerIdentity = LruCache<std::string, std::vector<std::string>>(128);
            LruCache<std::string, std::shared_ptr<Policy>> policies = LruCache<std::string, std::shared_ptr<Policy>>(1024);
        } cache;

        class Manager {
        protected:
            cache m_cache;
            std::recursive_mutex m_lock = std::recursive_mutex();

            virtual std::shared_ptr<Group> _findGroup(const std::string & id) = 0;
            virtual std::shared_ptr<Identity> _findIdentity(const std::string & id) = 0;
            virtual std::shared_ptr<Policy> _findPolicy(const std::string & id) = 0;

        public:
            std::shared_ptr<Identity>
            getIdentity(const std::string & id) {
                lock::Guard g(this->m_lock);

                return this->m_cache.identities
                    .get(id)
                    .value_or(this->_findIdentity(id));
            }

            std::shared_ptr<Group>
            getGroup(const std::string & id) {
                lock::Guard g(this->m_lock);

                return this->m_cache.groups
                    .get(id)
                    .value_or(this->_findGroup(id));
            }

            std::shared_ptr<Policy>
            getPolicy(const std::string & id) {
                lock::Guard g(this->m_lock);

                return this->m_cache.policies
                    .get(id)
                    .value_or(this->_findPolicy(id));
            }

            inline void save(const std::shared_ptr<Group> g) { this->save(*g); }
            virtual void save(const Group & g) = 0;

            inline void save(const std::shared_ptr<Identity> i) { this->save(*i); }
            virtual void save(const Identity & i) = 0;

            inline void save(const std::shared_ptr<Policy> p) { this->save(*p); }
            virtual void save(const Policy & p) = 0;

            virtual std::vector<std::shared_ptr<Group>> getGroupsFor(const Identity & identity) = 0;
            static std::shared_ptr<Manager> createManager(const std::string & dsn);
        };
    }
}
