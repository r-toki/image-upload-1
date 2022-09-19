import { Image } from '@chakra-ui/react';

type ImageItemProps = {
  src: string;
};

export const ImageItem = (props: ImageItemProps) => {
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
