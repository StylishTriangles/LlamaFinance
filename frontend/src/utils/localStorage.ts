export function getFromLocalStorage(key: string, defaultValue: any) {
  const data = localStorage.getItem(key);
  if (data === null)
    return defaultValue;

  if (defaultValue.constructor === String)
    return data ?? defaultValue;

  try {
    const value = JSON.parse(data);
    if (value === null)
      return defaultValue;

    // Let's make sure that tampering with localstorage doesn't break the app.
    // Check if retrieved value has correct type.
    return value.constructor === defaultValue.constructor
      ? value
      : defaultValue;
  } catch {
    return defaultValue;
  }
}
