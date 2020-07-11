#pragma once

#include "tables/group.h"
#include "tables/group_identity.h"
#include "tables/group_policy.h"
#include "tables/identity.h"
#include "tables/identity_policy.h"
#include "tables/policy.h"

namespace libzephir::storage::sql::postgres {
    typedef ::model::public_::group GroupTable;
    typedef ::model::public_::group_identity GroupIdentityTable;
    typedef ::model::public_::group_policy GroupPolicyTable;
    typedef ::model::public_::identity IdentityTable;
    typedef ::model::public_::identity_policy IdentityPolicyTable;
    typedef ::model::public_::policy PolicyTable;
}
