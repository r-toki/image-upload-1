import { Box, Container } from '@chakra-ui/react';
import { ReactNode } from 'react';

export const App = () => {
  return <AppLayout>App</AppLayout>;
};

type AppLayoutProps = {
  children: ReactNode;
};
const AppLayout = (props: AppLayoutProps) => {
  return (
    <Box h="full" bg="gray.50">
      <Container>{props.children}</Container>
    </Box>
  );
};
