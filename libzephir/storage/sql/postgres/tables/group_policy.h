#ifndef MODEL_PUBLIC_GROUP_POLICY_H
#define MODEL_PUBLIC_GROUP_POLICY_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace group_policy_ {

		struct Group_id {
			struct _alias_t {
				static constexpr const char _literal[] = R"("group_id")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T group_id;
						T &operator()() { return group_id; }
						const T &operator()() const { return group_id; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::varchar, sqlpp::tag::require_insert>;
		};

		struct Policy_id {
			struct _alias_t {
				static constexpr const char _literal[] = R"("policy_id")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T policy_id;
						T &operator()() { return policy_id; }
						const T &operator()() const { return policy_id; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::varchar, sqlpp::tag::require_insert>;
		};
	} // namespace group_policy_

	struct group_policy : sqlpp::table_t<group_policy,
				group_policy_::Group_id,
				group_policy_::Policy_id> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."group_policy")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T group_policy;
					T &operator()() { return group_policy; }
					const T &operator()() const { return group_policy; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
