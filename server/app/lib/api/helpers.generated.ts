import { FieldPolicy, FieldReadFunction, TypePolicies, TypePolicy } from '@apollo/client/cache';
export type MutationKeySpecifier = ('test' | 'testFailure' | MutationKeySpecifier)[];
export type MutationFieldPolicy = {
	test?: FieldPolicy<any> | FieldReadFunction<any>,
	testFailure?: FieldPolicy<any> | FieldReadFunction<any>
};
export type QueryKeySpecifier = ('test' | QueryKeySpecifier)[];
export type QueryFieldPolicy = {
	test?: FieldPolicy<any> | FieldReadFunction<any>
};
export type SubscriptionKeySpecifier = ('test' | SubscriptionKeySpecifier)[];
export type SubscriptionFieldPolicy = {
	test?: FieldPolicy<any> | FieldReadFunction<any>
};
export type TestPayloadKeySpecifier = ('ok' | 'value' | TestPayloadKeySpecifier)[];
export type TestPayloadFieldPolicy = {
	ok?: FieldPolicy<any> | FieldReadFunction<any>,
	value?: FieldPolicy<any> | FieldReadFunction<any>
};
export type StrictTypedTypePolicies = {
	Mutation?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | MutationKeySpecifier | (() => undefined | MutationKeySpecifier),
		fields?: MutationFieldPolicy,
	},
	Query?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | QueryKeySpecifier | (() => undefined | QueryKeySpecifier),
		fields?: QueryFieldPolicy,
	},
	Subscription?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | SubscriptionKeySpecifier | (() => undefined | SubscriptionKeySpecifier),
		fields?: SubscriptionFieldPolicy,
	},
	TestPayload?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | TestPayloadKeySpecifier | (() => undefined | TestPayloadKeySpecifier),
		fields?: TestPayloadFieldPolicy,
	}
};
export type TypedTypePolicies = StrictTypedTypePolicies & TypePolicies;