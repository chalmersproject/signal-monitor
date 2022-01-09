module.exports = {
  service: {
    name: "signal-monitor-server",
    localSchemaFile: `${__dirname}/lib/api/schema.generated.json`,
  },
  client: {
    includes: [
      `${__dirname}/components/**/*.{ts,tsx}`,
      `${__dirname}/pages/**/*.{ts,tsx}`,
    ],
  },
};
