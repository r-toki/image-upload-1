import { Button, FormControl, FormLabel, HStack, Input, Stack, useToast } from '@chakra-ui/react';
import { FormEventHandler } from 'react';

import { useFileInput } from '../hooks/useFileInput';
import { useObjectUrl } from '../hooks/useObjectUrl';
import { useTextInput } from '../hooks/useTextInput';
import { ImageItem } from './ImageItem';

type FormValues = {
  name: string;
  file: File;
};

type ImageUploadFormProps = {
  onSubmit: ({ name, file }: FormValues) => Promise<void>;
};

export const ImageUploadForm = (props: ImageUploadFormProps) => {
  const toast = useToast();

  const nameInput = useTextInput();
  const fileInput = useFileInput();
  const objectUrl = useObjectUrl(fileInput.file);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    const file = fileInput.file;
    if (file) {
      await props.onSubmit({ name: nameInput.value, file });
      nameInput.reset();
      fileInput.reset();
    } else {
      toast({ position: 'top-right', status: 'error', title: 'Image is required.' });
    }
  };

  return (
    <Stack onSubmit={onSubmit} as="form" spacing="4" width="md">
      <FormControl isRequired display="flex">
        <FormLabel width="32" flexShrink="0">
          Name
        </FormLabel>
        <Input type="text" size="sm" value={nameInput.value} onChange={nameInput.onChange} />
      </FormControl>

      <FormControl isRequired display="flex">
        <FormLabel width="32">Image</FormLabel>

        <Stack>
          <HStack>
            <input
              type="file"
              accept="image/*"
              style={{ display: 'none' }}
              ref={fileInput.ref}
              onChange={fileInput.onChange}
            />
            <Button size="sm" onClick={fileInput.onClick}>
              Upload
            </Button>
            <Button size="sm" onClick={() => fileInput.setFile(undefined)}>
              Delete
            </Button>
          </HStack>

          <ImageItem src={objectUrl ?? 'https://placehold.jp/280x280.png'} />
        </Stack>
      </FormControl>

      <Button type="submit" colorScheme="blue" size="sm" width="full">
        Post
      </Button>
    </Stack>
  );
};
