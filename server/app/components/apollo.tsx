import { FC, ReactNode } from "react";
import { useCallback, useContext, useMemo } from "react";
import { isEqual } from "lodash";
import merge from "deepmerge";

import { useToast } from "components/toast";

import { ApolloClient as Client } from "@apollo/client";
import { ApolloLink } from "@apollo/client";
import { ApolloProvider as Provider } from "@apollo/client";
import { ApolloError } from "@apollo/client";
import { ServerError } from "@apollo/client";
import { getApolloContext } from "@apollo/client";

import { HttpLink } from "@apollo/client";
import { RetryLink } from "@apollo/client/link/retry";
import { SentryLink } from "apollo-link-sentry";
import { WebSocketLink as WsLink } from "@apollo/client/link/ws";
import { setContext as setLinkContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";
import { InMemoryCache, NormalizedCacheObject } from "@apollo/client";
import { split as splitLinks } from "@apollo/client";
import { from as mergeLinks } from "@apollo/client";

import { TypedTypePolicies as TypePolicies } from "lib/api/helpers.generated";

const typePolicies: TypePolicies = {};

const createTerminatingLink = (): ApolloLink => {
  const httpLink = new HttpLink({ uri: "/api" });
  if (typeof window === "undefined") {
    return httpLink;
  }

  const wsLink = new WsLink({
    uri: (() => {
      const { protocol, host } = window.location;
      switch (protocol) {
        case "http:":
          return `ws://${host}/api`;
        case "https:":
          return `wss://${host}/api`;
        default:
          throw new Error("Unknown protocol.");
      }
    })(),
    options: {
      reconnect: true,
    },
  });

  return splitLinks(
    ({ query }) => {
      const definition = getMainDefinition(query);
      return (
        definition.kind === "OperationDefinition" &&
        definition.operation === "subscription"
      );
    },
    wsLink,
    httpLink,
  );
};

const createAuthLink = (): ApolloLink => {
  return setLinkContext(async (operation, { headers }) => {
    const response = await fetch("/api/auth/token");
    if (response.status === 200) {
      const token = await response.text();
      return {
        headers: {
          ...headers,
          authorization: `Bearer ${token}`,
        },
      };
    }
    return headers;
  });
};

const createApolloClient = (): Client<NormalizedCacheObject> => {
  const isClient = typeof window !== "undefined";
  return new Client({
    ssrMode: !isClient,
    link: mergeLinks([
      ...(isClient ? [new RetryLink()] : []),
      new SentryLink(),
      ...(isClient ? [createAuthLink()] : []),
      createTerminatingLink(),
    ]),
    cache: new InMemoryCache({ typePolicies }),
  });
};

export interface ApolloProviderProps {
  readonly initialState?: NormalizedCacheObject;
  readonly children: ReactNode | ReactNode[] | null;
}

export const ApolloProvider: FC<ApolloProviderProps> = ({
  initialState,
  children,
}) => {
  const apolloContext = useContext(getApolloContext());
  const apolloClient = useMemo(
    () => {
      if (apolloContext.client) {
        return apolloContext.client;
      }
      return initializeApolloClient({ initialState });
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [apolloContext],
  );

  return <Provider client={apolloClient}>{children}</Provider>;
};

let globalApolloClient: Client<NormalizedCacheObject> | undefined;

export type InitializeApolloClientOptions = {
  initialState?: NormalizedCacheObject;
};

export const initializeApolloClient = (
  options?: InitializeApolloClientOptions,
): Client<NormalizedCacheObject> => {
  const { initialState } = options ?? {};
  const client = globalApolloClient ?? createApolloClient();

  // Hydrate cache from initial state
  if (initialState) {
    // Get existing cache, loaded during client side data fetching
    const cache = client.extract();

    // Merge the existing cache with initial state
    const state = merge(initialState, cache, {
      // combine arrays using object equality (like in sets)
      arrayMerge: (destinationArray, sourceArray) => [
        ...sourceArray,
        ...destinationArray.filter(d => sourceArray.every(s => !isEqual(d, s))),
      ],
    });

    // Restore the cache with the merged data
    client.restore(state);
  }

  // For SSG and SSR always create a new client
  if (typeof window === "undefined") {
    return client;
  }

  // For browser, use a global client.
  if (!globalApolloClient) {
    globalApolloClient = client;
  }
  return globalApolloClient;
};

export const useHandleQueryError = (
  title?: string,
): ((error: ApolloError) => void) => {
  const toast = useToast();
  return useCallback(
    error => {
      const description = formatApolloError(error);
      toast({
        status: "error",
        title,
        description,
      });
    },
    [title, toast],
  );
};

export const formatApolloError = (error: ApolloError): string => {
  const { graphQLErrors, networkError, message } = error;
  if (graphQLErrors) {
    const [firstError] = graphQLErrors;
    if (firstError) {
      return `Error: ${firstError.message}`;
    }
  }
  if (networkError) {
    if ((networkError as ServerError | undefined)?.statusCode === 500) {
      return "An internal server error occurred.";
    }
  }
  return message;
};
