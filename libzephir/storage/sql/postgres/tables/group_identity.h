#ifndef MODEL_PUBLIC_GROUP_IDENTITY_H
#define MODEL_PUBLIC_GROUP_IDENTITY_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace group_identity_ {

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
	} // namespace group_identity_

	struct group_identity : sqlpp::table_t<group_identity,
				group_identity_::Group_id,
				group_identity_::Identity_id> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."group_identity")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T group_identity;
					T &operator()() { return group_identity; }
					const T &operator()() const { return group_identity; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
