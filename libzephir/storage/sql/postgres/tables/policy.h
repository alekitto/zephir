#ifndef MODEL_PUBLIC_POLICY_H
#define MODEL_PUBLIC_POLICY_H


#include <sqlpp11/table.h>
#include <sqlpp11/char_sequence.h>
#include <sqlpp11/column_types.h>

namespace model {

namespace public_ {
	namespace policy_ {

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

		struct Version {
			struct _alias_t {
				static constexpr const char _literal[] = R"("version")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T version;
						T &operator()() { return version; }
						const T &operator()() const { return version; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::integer>;
		};

		struct Effect {
			struct _alias_t {
				static constexpr const char _literal[] = R"("effect")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T effect;
						T &operator()() { return effect; }
						const T &operator()() const { return effect; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::boolean, sqlpp::tag::require_insert>;
		};

		struct Actions {
			struct _alias_t {
				static constexpr const char _literal[] = R"("actions")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T actions;
						T &operator()() { return actions; }
						const T &operator()() const { return actions; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::text, sqlpp::tag::require_insert>;
		};

		struct Resources {
			struct _alias_t {
				static constexpr const char _literal[] = R"("resources")";
				using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
				template<typename T>
					struct _member_t {
						T resources;
						T &operator()() { return resources; }
						const T &operator()() const { return resources; }
					};
			};

			using _traits = ::sqlpp::make_traits<::sqlpp::text, sqlpp::tag::require_insert>;
		};
	} // namespace policy_

	struct policy : sqlpp::table_t<policy,
				policy_::Id,
				policy_::Version,
				policy_::Effect,
				policy_::Actions,
				policy_::Resources> {
		using _value_type = sqlpp::no_value_t;
		struct _alias_t {
			static constexpr const char _literal[] = R"("public"."policy")";
			using _name_t = sqlpp::make_char_sequence<sizeof(_literal), _literal>;
			template<typename T>
				struct _member_t {
					T policy;
					T &operator()() { return policy; }
					const T &operator()() const { return policy; }
				};
		};
	};
} // namespace public
} // namespace model

#endif
