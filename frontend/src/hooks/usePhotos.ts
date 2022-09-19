import { useEffect, useState } from 'react';

import { encodeFile } from '../lib/encode-file';

export type Photo = {
  id: string;
  encoded: string;
  name: string;
  contentType: string;
  byteSize: number;
  metadata: { width: number; height: number };
  createdAt: string;
};

type CreatePhotoInput = { file: File; name: string };

export const usePhotos = () => {
  const [photos, setPhotos] = useState<Photo[]>([]);

  const fetchPhotos = async () => {
    await fetch('http://127.0.0.1:8080/blobs')
      .then((res) => res.json())
      .then(setPhotos);
  };

  const createPhoto = async ({ file, name }: CreatePhotoInput) => {
    const encoded = await encodeFile(file);

    await fetch('http://127.0.0.1:8080/blobs', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        encoded: encoded,
        name,
        contentType: file.type,
      }),
    });

    await fetchPhotos();
  };

  const deletePhoto = async (id: string) => {
    await fetch(`http://127.0.0.1:8080/blobs/${id}`, {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        id,
      }),
    });

    await fetchPhotos();
  };

  useEffect(() => {
    fetchPhotos();
  }, []);

  return { photos, createPhoto, deletePhoto };
};
