import React, { ReactElement } from "react";

import NextApp from "next/app";
import type { AppProps as NextAppProps } from "next/app";
import type { AppInitialProps as NextAppInitialProps } from "next/app";
import type { AppContext as NextAppContext } from "next/app";

import { ApolloProvider } from "components/apollo";
import { ChakraProvider } from "components/chakra";

import { MetaTitle, MetaDescription, MetaType } from "components/meta";

import "../styles.css";

const App = ({ Component, pageProps }: NextAppProps): ReactElement => {
  const { cookieHeader, apolloState, ...otherProps } = pageProps;
  return (
    <>
      <>
        <MetaTitle site="Chalmers Signal" />
        <MetaDescription description="Quickly find available shelters and respite centers." />
        <MetaType type="website" />
      </>
      <ApolloProvider initialState={apolloState}>
        <ChakraProvider cookies={cookieHeader}>
          <Component {...otherProps} />
        </ChakraProvider>
      </ApolloProvider>
    </>
  );
};

App.getInitialProps = async (
  appCtx: NextAppContext,
): Promise<NextAppInitialProps> => {
  const { pageProps } = await NextApp.getInitialProps(appCtx);
  const { req } = appCtx.ctx;
  if (req) {
    const { cookie: cookieHeader } = req.headers;
    return {
      pageProps: { ...pageProps, cookieHeader },
    };
  }
  return { pageProps };
};

export default App;
