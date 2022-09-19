import { Box, Container, Stack, VStack, Wrap, WrapItem } from '@chakra-ui/react';
import { ReactNode } from 'react';

import { ImageUploadForm } from './components/ImageUploadForm';
import { PhotoItem } from './components/PhotoItem';
import { usePhotos } from './hooks/usePhotos';

export const App = () => {
  const { photos, createPhoto, deletePhoto } = usePhotos();

  return (
    <AppLayout>
      <Stack direction="row" spacing="20">
        <VStack spacing="4">
          <Box fontWeight="bold" fontSize="xl">
            Image Upload Form
          </Box>

          <ImageUploadForm onSubmit={createPhoto} />
        </VStack>

        <Wrap spacing="4">
          {photos.map((photo) => (
            <WrapItem key={photo.id}>
              <PhotoItem photo={photo} onDelete={deletePhoto} />
            </WrapItem>
          ))}
        </Wrap>
      </Stack>
    </AppLayout>
  );
};

type AppLayoutProps = {
  children: ReactNode;
};

const AppLayout = (props: AppLayoutProps) => {
  return (
    <Box h="full">
      <Container maxW="container.xl" h="full" py="4">
        {props.children}
      </Container>
    </Box>
  );
};
