#ifndef MODEL_PUBLIC_IDENTITY_POLICY_H
#define MODEL_PUBLIC_IDENTITY_POLICY_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace identity_policy_ {

		struct Identity_id {
			struct _alias_t {
				static constexpr const char _literal[] = R"("identity_id")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T identity_id;
						T &operator()() { return identity_id; }
						const T &operator()() const { return identity_id; }
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
	} // namespace identity_policy_

	struct identity_policy : sqlpp::table_t<identity_policy,
				identity_policy_::Identity_id,
				identity_policy_::Policy_id> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."identity_policy")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T identity_policy;
					T &operator()() { return identity_policy; }
					const T &operator()() const { return identity_policy; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
