#include "MatchResult.hpp"
#include "Policy.hpp"

void MatchResult::_update()
{
    if (nullptr != this->m_partial) {
        this->m_partial.reset();
        this->m_partial = nullptr;
    }

    if (
        (m_action.has_value() && ! m_action.value()) ||
        (m_resource.has_value() && ! m_resource.value())
    ) {
        this->_type = FULL;
        this->_outcome = NOT_MATCH;

        return;
    }

    if (m_action.value() || m_resource.value()) {
        this->_outcome = MATCH;
    }

    if (m_action.has_value() && m_resource.has_value()) {
        this->_type = FULL;
    } else {
        this->m_partial = std::make_unique<PartialPolicy>(
            m_policy._version,
            m_policy._effect,
            m_action.has_value() ? std::nullopt : m_policy._actions,
            m_resource.has_value() ? std::nullopt : m_policy._resources
        );
    }
}