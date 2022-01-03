module.exports = {
  service: {
    name: "home",
    localSchemaFile: `${__dirname}/lib/api/schema.json`,
  },
  client: {
    includes: [
      `${__dirname}/components/**/*.{ts,tsx}`,
      `${__dirname}/pages/**/*.{ts,tsx}`,
    ],
  },
};
