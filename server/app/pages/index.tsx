import type { NextPage } from "next";

import { Box, chakra } from "@chakra-ui/react";
import { Text } from "@chakra-ui/react";

import { gql } from "@apollo/client";
import { useHomePageSubscription } from "lib/api";

gql`
  subscription HomePage {
    test
  }
`;

const HomePage: NextPage = () => {
  const { data } = useHomePageSubscription();
  return (
    <Box>
      <Text>
        Data:{" "}
        <chakra.span fontFamily="mono">{JSON.stringify(data)}</chakra.span>
      </Text>
    </Box>
  );
};

export default HomePage;
