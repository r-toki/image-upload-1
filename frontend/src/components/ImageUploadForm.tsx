import {
  Button,
  FormControl,
  FormLabel,
  HStack,
  Image,
  Input,
  Stack,
  useToast,
} from '@chakra-ui/react';
import { FormEventHandler } from 'react';

import { useFileInput } from '../hooks/useFileInput';
import { useObjectUrl } from '../hooks/useObjectUrl';
import { useTextInput } from '../hooks/useTextInput';
import { encodeFile } from '../lib/encode-file';

export const ImageUploadForm = () => {
  const toast = useToast();

  const nameInput = useTextInput();
  const fileInput = useFileInput();
  const objectUrl = useObjectUrl(fileInput.file);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    const file = fileInput.file;
    if (file) {
      const encoded = await encodeFile(file);

      fetch('http://127.0.0.1:8080/blobs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          encoded_data: encoded,
          name: nameInput.value,
          content_type: file.type,
        }),
      })
        .then((res) => res.json())
        .then(console.log);
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

          <ImageView src={objectUrl ?? 'https://placehold.jp/280x280.png'} />
        </Stack>
      </FormControl>

      <Button type="submit" colorScheme="blue" size="sm" width="full">
        Post
      </Button>
    </Stack>
  );
};

type ImageViewProps = {
  src: string;
};
const ImageView = (props: ImageViewProps) => {
  return (
    <Image
      src={props.src}
      width="280px"
      height="280px"
      border="1px"
      borderColor="gray.200"
      rounded="md"
      objectFit="cover"
    />
  );
};
