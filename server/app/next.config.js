const { withSentryConfig } = require("@sentry/nextjs");

const projectName = "home";
const {
  name: packageName,
  version: packageVersion,
} = require("./package.json");

const { BASE_URL } = process.env;
const { SENTRY_URL, SENTRY_ORG, SENTRY_PROJECT, SENTRY_DSN } = process.env;

/** @type {import('./config').Config} */
const runtimeConfig = {
  projectName,
  packageName,
  packageVersion,
  baseUrl: BASE_URL,
  sentryDSN: SENTRY_DSN,
};

/** @type {import('next').NextConfig} */
const config = {
  swcMinify: true,
  reactStrictMode: true,
  publicRuntimeConfig: { ...runtimeConfig },
  serverRuntimeConfig: { ...runtimeConfig },
};

if (!!SENTRY_URL && !!SENTRY_ORG && !!SENTRY_PROJECT) {
  /** @type {import('@sentry/webpack-plugin').SentryCliPluginOptions} */
  const sentryOptions = {
    silent: true,
    release: `${projectName}-${packageName}@${packageVersion}`,
  };
  module.exports = withSentryConfig(config, sentryOptions);
} else {
  const missingVariables = Object.entries({
    SENTRY_URL: SENTRY_URL,
    SENTRY_ORG: SENTRY_ORG,
    SENTRY_PROJECT: SENTRY_PROJECT,
  })
    .filter(([, value]) => !value)
    .map(([key]) => key);
  const missingVars = missingVariables.join(", ");
  console.info(
    `[Sentry] Skip uploading sourcemaps (missing variables: ${missingVars})`,
  );
  module.exports = config;
}
