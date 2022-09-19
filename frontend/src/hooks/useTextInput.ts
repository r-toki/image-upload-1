import { ChangeEventHandler, useState } from 'react';

export const useTextInput = (initValue = '') => {
  const [value, setValue] = useState(initValue);
  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    setValue(e.target.value);
  };
  const reset = () => setValue(initValue);
  return { value, onChange, reset };
};
