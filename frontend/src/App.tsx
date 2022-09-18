import { Box, Container } from '@chakra-ui/react';
import { ReactNode, useEffect, useState } from 'react';

export const App = () => {
  const [res, setRes] = useState<string | null>(null);

  useEffect(() => {
    fetch('http://127.0.0.1:8080/')
      .then((v) => v.json())
      .then(setRes);
  }, []);

  return (
    <AppLayout>
      <Box>{res}</Box>
    </AppLayout>
  );
};

type AppLayoutProps = {
  children: ReactNode;
};
const AppLayout = (props: AppLayoutProps) => {
  return (
    <Box h="full" bg="gray.50">
      <Container py="4">{props.children}</Container>
    </Box>
  );
};
