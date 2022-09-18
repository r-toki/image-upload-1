export const encodeFile = (file: File): Promise<string> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => {
      const result = reader.result as string;
      const [, encoded] = result.split(';base64,');
      resolve(encoded);
    };
    reader.onerror = (error) => reject(error);
  });
