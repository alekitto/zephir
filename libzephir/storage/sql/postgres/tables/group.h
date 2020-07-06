#ifndef MODEL_PUBLIC_GROUP_H
#define MODEL_PUBLIC_GROUP_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace group_ {

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
	} // namespace group_

	struct group : sqlpp::table_t<group,
				group_::Id,
				group_::Policy_id> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."group")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T group;
					T &operator()() { return group; }
					const T &operator()() const { return group; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
