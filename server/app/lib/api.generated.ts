import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions =  {}
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Mutation = {
  __typename?: 'Mutation';
  test: TestPayload;
  testFailure: TestPayload;
};


export type MutationTestArgs = {
  input: TestInput;
};


export type MutationTestFailureArgs = {
  input: TestInput;
};

export type Query = {
  __typename?: 'Query';
  test: Scalars['Boolean'];
};

export type Subscription = {
  __typename?: 'Subscription';
  test: Scalars['Int'];
};

export type TestInput = {
  value: Scalars['String'];
};

export type TestPayload = {
  __typename?: 'TestPayload';
  ok: Scalars['Boolean'];
  value: Scalars['String'];
};

export type HomePageSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type HomePageSubscription = { __typename?: 'Subscription', test: number };


export const HomePageDocument = gql`
    subscription HomePage {
  test
}
    `;

/**
 * __useHomePageSubscription__
 *
 * To run a query within a React component, call `useHomePageSubscription` and pass it any options that fit your needs.
 * When your component renders, `useHomePageSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useHomePageSubscription({
 *   variables: {
 *   },
 * });
 */
export function useHomePageSubscription(baseOptions?: Apollo.SubscriptionHookOptions<HomePageSubscription, HomePageSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<HomePageSubscription, HomePageSubscriptionVariables>(HomePageDocument, options);
      }
export type HomePageSubscriptionHookResult = ReturnType<typeof useHomePageSubscription>;
export type HomePageSubscriptionResult = Apollo.SubscriptionResult<HomePageSubscription>;