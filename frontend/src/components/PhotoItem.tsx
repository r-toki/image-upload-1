import { DeleteIcon } from '@chakra-ui/icons';
import { Box, IconButton, VStack } from '@chakra-ui/react';

import { Photo } from '../hooks/usePhotos';
import { ImageItem } from './ImageItem';

type PhotoItemProps = { photo: Photo; onDelete: (id: string) => Promise<void> };

export const PhotoItem = ({ photo, onDelete }: PhotoItemProps) => {
  const src = ['data:', photo.contentType, ';base64,', photo.encoded].join('');

  const handleDelete = async () => {
    if (confirm('Are you sure you want to delete it?')) await onDelete(photo.id);
  };

  return (
    <VStack
      position="relative"
      padding="2"
      border="1px"
      borderColor="gray.200"
      rounded="md"
      bg="gray.50"
    >
      <ImageItem src={src} />
      <Box fontWeight="bold">{photo.name}</Box>

      <Box position="absolute" right="2" bottom="1">
        <IconButton
          icon={<DeleteIcon />}
          aria-label=""
          size="sm"
          variant="ghost"
          onClick={handleDelete}
        />
      </Box>
    </VStack>
  );
};
