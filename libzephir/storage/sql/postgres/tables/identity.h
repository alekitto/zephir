#ifndef MODEL_PUBLIC_IDENTITY_H
#define MODEL_PUBLIC_IDENTITY_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace identity_ {

		struct Id {
			struct _alias_t {
				static constexpr const char _literal[] = R"("id")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T id;
						T &operator()() { return id; }
						const T &operator()() const { return id; }
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

			using _traits = ::sqlpp::make_traits<::sqlpp::varchar, sqlpp::tag::can_be_null>;
		};
	} // namespace identity_

	struct identity : sqlpp::table_t<identity,
				identity_::Id,
				identity_::Policy_id> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."identity")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T identity;
					T &operator()() { return identity; }
					const T &operator()() const { return identity; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
