import { Box, Container, VStack } from '@chakra-ui/react';
import { ReactNode } from 'react';

import { ImageUploadForm } from './components/ImageUploadForm';

export const App = () => {
  return (
    <AppLayout>
      <VStack spacing="4">
        <Box fontWeight="bold" fontSize="xl">
          Image Upload Form
        </Box>

        <ImageUploadForm />
      </VStack>
    </AppLayout>
  );
};

type AppLayoutProps = {
  children: ReactNode;
};
const AppLayout = (props: AppLayoutProps) => {
  return (
    <Box h="full">
      <Container h="full" py="4">
        {props.children}
      </Container>
    </Box>
  );
};
