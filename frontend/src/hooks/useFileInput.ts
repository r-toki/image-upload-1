import { ChangeEventHandler, MouseEventHandler, useRef, useState } from 'react';

export const useFileInput = () => {
  const [file, setFile] = useState<File>();

  const ref = useRef<HTMLInputElement>(null);

  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    const files = e.target.files;
    if (files && files[0]) setFile(files[0]);
    e.target.value = '';
  };

  const onClick: MouseEventHandler<HTMLButtonElement> = () => {
    ref.current?.click();
  };

  return {
    file,
    setFile,
    ref,
    onChange,
    onClick,
  };
};
