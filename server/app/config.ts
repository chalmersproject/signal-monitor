import getConfig from "next/config";

export type Config = {
  projectName: string;
  packageName: string;
  packageVersion: string;
  baseUrl: string;
  sentryDSN: string | undefined;
};

const { publicRuntimeConfig, serverRuntimeConfig } = getConfig();
const { projectName, packageName, packageVersion, baseUrl, sentryDSN } = {
  ...publicRuntimeConfig,
  ...serverRuntimeConfig,
} as Config;

export { projectName, packageName, packageVersion, baseUrl, sentryDSN };
